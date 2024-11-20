use std::process::ExitCode;
use std::sync;
use std::sync::mpsc::Receiver;
use std::time::{Duration, Instant};

use bench_vortex::setup_logger;
use bench_vortex::tpch::dbgen::{DBGen, DBGenOptions};
use bench_vortex::tpch::{
    load_datasets, run_tpch_query, tpch_queries, Format, EXPECTED_ROW_COUNTS,
};
use clap::{ArgAction, Parser, ValueEnum};
use futures::future::try_join_all;
use indicatif::ProgressBar;
use itertools::Itertools;
use log::LevelFilter;
use serde::Serialize;
use tabled::builder::Builder;
use tabled::settings::themes::Colorization;
use tabled::settings::{Color, Style};
use vortex::aliases::hash_map::HashMap;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_delimiter = ',')]
    queries: Option<Vec<usize>>,
    #[arg(short, long, value_delimiter = ',')]
    exclude_queries: Option<Vec<usize>>,
    #[arg(short, long)]
    threads: Option<usize>,
    #[arg(short, long, default_value_t = true, default_missing_value = "true", action = ArgAction::Set)]
    warmup: bool,
    #[arg(short, long, default_value = "8")]
    iterations: usize,
    #[arg(long)]
    only_vortex: bool,
    #[arg(short, long)]
    verbose: bool,
    #[arg(short, long, default_value_t, value_enum)]
    display_format: DisplayFormat,
}

#[derive(ValueEnum, Default, Clone, Debug)]
enum DisplayFormat {
    #[default]
    Table,
    GhJson,
}

#[derive(Clone)]
struct Measurement {
    query_idx: usize,
    time: Duration,
    format: Format,
}

#[derive(Serialize)]
struct JsonValue {
    name: String,
    unit: String,
    value: u128,
}

impl Measurement {
    fn to_json(&self) -> JsonValue {
        let name = format!(
            "tpch_q{query_idx}/{format}",
            format = self.format.name(),
            query_idx = self.query_idx
        );

        JsonValue {
            name,
            unit: "ns".to_string(),
            value: self.time.as_nanos(),
        }
    }
}

fn main() -> ExitCode {
    let args = Args::parse();

    setup_logger(if args.verbose {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    });

    let runtime = match args.threads {
        Some(0) => panic!("Can't use 0 threads for runtime"),
        Some(1) => tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build(),
        Some(n) => tokio::runtime::Builder::new_multi_thread()
            .worker_threads(n)
            .enable_all()
            .build(),
        None => tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build(),
    }
    .expect("Failed building the Runtime");

    runtime.block_on(bench_main(
        args.queries,
        args.exclude_queries,
        args.iterations,
        args.warmup,
        args.only_vortex,
        args.display_format,
    ))
}

fn render_table(receiver: Receiver<Measurement>, formats: &[Format]) -> anyhow::Result<()> {
    let mut measurements: HashMap<Format, Vec<Measurement>> = HashMap::default();

    while let Ok(m) = receiver.recv() {
        measurements.entry(m.format).or_default().push(m);
    }

    measurements.values_mut().for_each(|v| {
        v.sort_by_key(|m| m.query_idx);
    });

    // The first format serves as the baseline
    let baseline_format = &formats[0];
    let baseline = measurements[baseline_format].clone();

    let mut table_builder = Builder::default();
    let mut colors = vec![];

    let mut header = vec!["Query".to_string()];
    header.extend(formats.iter().map(|f| format!("{:?}", f)));
    table_builder.push_record(header);

    for (query_idx, baseline_measure) in baseline.iter().enumerate().take(22) {
        let query_baseline = baseline_measure.time.as_micros();
        let mut row = vec![query_idx.to_string()];
        for (col_idx, format) in formats.iter().enumerate() {
            let time_us = measurements[format][query_idx].time.as_micros();

            if format != baseline_format {
                let color = color(query_baseline, time_us);

                colors.push(Colorization::exact(
                    vec![color],
                    (query_idx + 1, col_idx + 1),
                ))
            }

            let ratio = time_us as f64 / query_baseline as f64;
            row.push(format!("{time_us} us ({ratio:.2})"));
        }
        table_builder.push_record(row);
    }

    let mut table = table_builder.build();
    table.with(Style::modern());

    for color in colors.into_iter() {
        table.with(color);
    }

    println!("{table}");

    Ok(())
}

fn color(baseline_time: u128, test_time: u128) -> Color {
    if test_time > (baseline_time + baseline_time / 2) {
        Color::BG_RED
    } else if test_time > (baseline_time + baseline_time / 10) {
        Color::BG_YELLOW
    } else {
        Color::BG_BRIGHT_GREEN
    }
}

fn print_measurements_json(receiver: Receiver<Measurement>) -> anyhow::Result<()> {
    let mut measurements = Vec::new();

    while let Ok(m) = receiver.recv() {
        measurements.push(m.to_json());
    }

    let output = serde_json::to_string(&measurements)?;

    print!("{output}");

    Ok(())
}

async fn bench_main(
    queries: Option<Vec<usize>>,
    exclude_queries: Option<Vec<usize>>,
    iterations: usize,
    warmup: bool,
    only_vortex: bool,
    display_format: DisplayFormat,
) -> ExitCode {
    // uncomment the below to enable trace logging of datafusion execution
    // setup_logger(LevelFilter::Trace);

    // Run TPC-H data gen.
    let data_dir = DBGen::new(DBGenOptions::default()).generate().unwrap();

    // The formats to run against (vs the baseline)
    let formats = if only_vortex {
        vec![
            Format::Arrow,
            Format::OnDiskVortex {
                enable_compression: true,
            },
        ]
    } else {
        vec![
            Format::Arrow,
            Format::Parquet,
            Format::OnDiskVortex {
                enable_compression: true,
            },
        ]
    };

    // Load datasets
    let ctxs = try_join_all(
        formats
            .iter()
            .map(|format| load_datasets(&data_dir, *format)),
    )
    .await
    .unwrap();

    let query_count = queries.as_ref().map_or(22, |c| c.len());

    // Setup a progress bar
    let progress = ProgressBar::new((query_count * formats.len()) as u64);

    // Send back a channel with the results of Row.
    let (measurements_tx, measurements_rx) = sync::mpsc::channel();
    let (row_count_tx, row_count_rx) = sync::mpsc::channel();

    for (query_idx, sql_queries) in tpch_queries() {
        if queries
            .as_ref()
            .map_or(false, |included| !included.contains(&query_idx))
        {
            continue;
        }

        if exclude_queries
            .as_ref()
            .map_or(false, |e| e.contains(&query_idx))
        {
            continue;
        }
        let ctxs = ctxs.clone();
        let tx = measurements_tx.clone();
        let count_tx = row_count_tx.clone();
        let progress = progress.clone();
        let formats = formats.clone();

        for (ctx, format) in ctxs.iter().zip(formats.iter()) {
            if warmup {
                for i in 0..2 {
                    let row_count = run_tpch_query(ctx, &sql_queries, query_idx, *format).await;
                    if i == 0 {
                        count_tx.send((query_idx, *format, row_count)).unwrap();
                    }
                }
            }

            let mut measures = Vec::new();
            for _ in 0..iterations {
                let start = Instant::now();
                run_tpch_query(ctx, &sql_queries, query_idx, *format).await;
                let elapsed = start.elapsed();
                measures.push(elapsed);
            }
            let fastest = measures.iter().cloned().min().unwrap();

            tx.send(Measurement {
                query_idx,
                time: fastest,
                format: *format,
            })
            .unwrap();

            progress.inc(1);
        }
    }

    // delete parent handle to tx
    drop(measurements_tx);
    drop(row_count_tx);

    let mut format_row_counts: HashMap<Format, Vec<usize>> = HashMap::new();
    while let Ok((idx, format, row_count)) = row_count_rx.recv() {
        format_row_counts
            .entry(format)
            .or_insert_with(|| vec![0; EXPECTED_ROW_COUNTS.len()])[idx] = row_count;
    }

    progress.finish();

    let mut mismatched = false;
    for (format, row_counts) in format_row_counts {
        row_counts
            .into_iter()
            .zip_eq(EXPECTED_ROW_COUNTS)
            .enumerate()
            .filter(|(idx, _)| queries.as_ref().map(|q| q.contains(idx)).unwrap_or(true))
            .for_each(|(idx, (row_count, expected_row_count))| {
                if row_count != expected_row_count {
                    eprintln!("Mismatched row count {row_count} instead of {expected_row_count} in query {idx} for format {format:?}");
                    mismatched = true;
                }
            })
    }

    match display_format {
        DisplayFormat::Table => {
            render_table(measurements_rx, &formats).unwrap();
        }
        DisplayFormat::GhJson => {
            print_measurements_json(measurements_rx).unwrap();
        }
    }

    if mismatched {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
