const std = @import("std");

pub const input = @import("input.zig");
pub const parse = @import("parse.zig");

test {
    std.testing.refAllDecls(@This());
}