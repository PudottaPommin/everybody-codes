const std = @import("std");

pub const input = @import("input.zig");
pub const parse = @import("parse.zig");

pub fn trimAllWhitespaces(slice: []const u8) []const u8 {
    return std.mem.trim(u8, slice, " \r\t\n");
}

pub fn reverseSlice(comptime T: type, alloc: std.mem.Allocator, needle: []const T) ![]const T {
    var rev_needle = try alloc.alignedAlloc(T, .@"8", needle.len);
    for (0..rev_needle.len) |i| {
        rev_needle[i] = needle[needle.len - 1 - i];
    }
    return rev_needle;
}

pub fn findAllPos(comptime T: type, alloc: std.mem.Allocator, haystack: []const T, needle: []const T) ![]const usize {
    if (haystack.len < needle.len or needle.len == 0) return &.{};

    var arr: std.ArrayList(usize) = .empty;
    defer arr.deinit(alloc);
    var start: usize = 0;
    while (std.mem.findPos(T, haystack, start, needle)) |pos| {
        try arr.append(alloc, pos);
        start = pos + 1;
    }
    return arr.toOwnedSlice(alloc);
}

pub fn countBothWays(alloc: std.mem.Allocator, haystack: []const u8, needle: []const u8) !usize {
    var count = std.mem.count(u8, haystack, needle);

    var rev_needle = try alloc.allocSentinel(u8, needle.len, 0);
    defer alloc.free(rev_needle);
    for (0..rev_needle.len) |i| {
        rev_needle[i] = needle[needle.len - 1 - i];
    }
    count += std.mem.count(u8, haystack, rev_needle);

    return count;
}

pub fn splitIntoWords(text: []const u8) std.mem.SplitIterator(u8, .scalar) {
    return std.mem.splitScalar(u8, text, ' ');
}

pub fn splitByIntoArray(alloc: std.mem.Allocator, text: []const u8, ch: u8) !std.ArrayList([]const u8) {
    const count = std.mem.countScalar(u8, text, ch);
    var list: std.ArrayList([]const u8) = try .initCapacity(alloc, count);
    // defer list.deinit(alloc);

    var it = std.mem.splitScalar(u8, text, ch);
    var i: usize = 0;
    while (it.next()) |part| : (i += 1) {
        const p = trimAllWhitespaces(part);
        if (p.len > 0) {
            try list.append(alloc, p);
        }
    }
    return list;
}

test {
    std.testing.refAllDecls(@This());
}

test "splitByIntoArray" {
    var r = try splitByIntoArray(std.testing.allocator, "A,B,C,D,E", ',');
    defer r.deinit(std.testing.allocator);

    const expected = [_][]const u8{ "A", "B", "C", "D", "E" };
    try std.testing.expectEqual(expected.len, r.items.len);
    for (expected, r.items) |e, a| {
        try std.testing.expectEqualStrings(e, a);
    }
}

test "findAllPos" {
    const r = try findAllPos(u8, std.testing.allocator, "QAQAQA", "QAQ");
    defer std.testing.allocator.free(r);
    try std.testing.expectEqualSlices(usize, &.{ 0, 2 }, r);
}
