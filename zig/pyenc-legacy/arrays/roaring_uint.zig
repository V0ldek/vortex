//! Python bindings for the pyenc library.
const std = @import("std");
const py = @import("pydust");
const enc = @import("pyenc");
const pyarray = @import("./array.zig");
const pybuffer = @import("../pybuffer.zig");
const pyenc = @import("../pyenc.zig");
const pyio = @import("../pyio.zig");

/// A pyenc wrapper around a bool array.
pub const RoaringUIntArray = py.class(struct {
    const Self = @This();

    pub usingnamespace pyarray.Subclass(Self, enc.RoaringUIntArray);

    array: pyenc.Array,

    pub fn to_int_array(self: *const Self) !*const pyenc.PrimitiveArray {
        return pyenc.PrimitiveArray.wrapOwned(try self.unwrap().toIntArray(pyenc.allocator()));
    }

    pub fn __str__(self: *const Self) !py.PyString {
        _ = self;
        return try py.PyString.create("RoaringUIntArray()");
    }
});
