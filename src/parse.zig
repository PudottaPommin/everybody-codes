const std = @import("std");

pub fn lines(text: []const u8) std.mem.TokenIterator(u8, .scalar) {
    return std.mem.tokenizeScalar(u8, text, '\n');
}

/// Extracts every (optionally negative) integer from arbitrary text.
pub fn ints(comptime T: type, alloc: std.mem.Allocator, text: []const u8) ![]T {
    var list: std.ArrayList(T) = .empty;
    var i: usize = 0;
    while (i < text.len) {
        const neg = text[i] == '-' and i + 1 < text.len and std.ascii.isDigit(text[i + 1]);
        if (std.ascii.isDigit(text[i]) or neg) {
            var j = i + 1;
            while (j < text.len and std.ascii.isDigit(text[j])) j += 1;
            try list.append(alloc, try std.fmt.parseInt(T, text[i..j], 10));
            i = j;
        } else i += 1;
    }
    return list.toOwnedSlice(alloc);
}

test "ints" {
    const r = try ints(i32, std.testing.allocator, "a=1, b=-22;3");
    defer std.testing.allocator.free(r);
    try std.testing.expectEqualSlices(i32, &.{ 1, -22, 3 }, r);
}
