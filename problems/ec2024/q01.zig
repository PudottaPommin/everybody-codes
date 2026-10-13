const std = @import("std");
const common = @import("common");

const Enemies = enum {
    Ant,
    Beetle,
    Cockroach,
    Dragonfly,

    const Self = @This();

    fn fromChar(c: u8) !?Enemies {
        return switch (c) {
            'x' => null,
            'A' => .Ant,
            'B' => .Beetle,
            'C' => .Cockroach,
            'D' => .Dragonfly,
            else => error.InvalidEnemy,
        };
    }

    fn required_potions(self: Self) u8 {
        return switch (self) {
            .Ant => 0,
            .Beetle => 1,
            .Cockroach => 3,
            .Dragonfly => 5,
        };
    }
};

pub fn part1(alloc: std.mem.Allocator, input: []const u8) !u64 {
    _ = alloc;
    var total: u64 = 0;
    for (input) |c| {
        if (try Enemies.fromChar(c)) |e| total += e.required_potions();
    }
    return total;
}

pub fn part2(alloc: std.mem.Allocator, input: []const u8) !u64 {
    _ = alloc;
    var total: u64 = 0;
    var i: usize = 0;
    while (i + 1 < input.len) : (i += 2) {
        var n: u8 = 0;
        for (input[i..(i + 2)]) |ch| {
            if (try Enemies.fromChar(ch)) |e| {
                total += e.required_potions();
                n += 1;
            }
        }
        if (n > 1) total += n * (n - 1);
    }
    return total;
}

pub fn part3(alloc: std.mem.Allocator, input: []const u8) !u64 {
    _ = alloc;
    var total: u64 = 0;
    var i: usize = 0;
    while (i + 2 < input.len) : (i += 3) {
        var n: u8 = 0;
        for (input[i..][0..3]) |ch| {
            if (try Enemies.fromChar(ch)) |e| {
                total += e.required_potions();
                n += 1;
            }
        }
        if (n > 1) total += n * (n - 1);
    }
    return total;
}

test "part1-test" {
    var arena: std.heap.ArenaAllocator = .init(std.testing.allocator);
    defer arena.deinit();
    const input: []const u8 = "ABBAC";
    const r = try part1(arena.allocator(), input);
    try std.testing.expectEqual(5, r);
}

test "part2-test" {
    var arena: std.heap.ArenaAllocator = .init(std.testing.allocator);
    defer arena.deinit();
    const input: []const u8 = "AxBCDDCAxD";
    const r = try part2(arena.allocator(), input);
    try std.testing.expectEqual(28, r);
}

test "part3-test" {
    var arena: std.heap.ArenaAllocator = .init(std.testing.allocator);
    defer arena.deinit();
    const input: []const u8 = "xBxAAABCDxCC";
    const r = try part3(arena.allocator(), input);
    try std.testing.expectEqual(30, r);
}

test "parseEnum" {
    try std.testing.expectEqual(Enemies.Beetle, try Enemies.fromChar('B'));
    try std.testing.expectEqual(null, try Enemies.fromChar('x'));
    try std.testing.expectError(error.InvalidEnemy, Enemies.fromChar('z'));
}
