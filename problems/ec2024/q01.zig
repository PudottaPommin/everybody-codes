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

    fn potionCost(c: u8) !?u8 {
        return switch (c) {
            'x' => null,
            'A' => 0,
            'B' => 1,
            'C' => 3,
            'D' => 5,
            else => error.InvalidEnemy,
        };
    }
};

fn solve(comptime chunk_size: usize, input: []const u8) !u64 {
    var total: u64 = 0;
    var i: usize = 0;
    while (i + chunk_size <= input.len) : (i += chunk_size) {
        var n: u8 = 0;
        for (input[i..][0..chunk_size]) |ch| {
            if (try Enemies.potionCost(ch)) |cost| {
                total += cost;
                n += 1;
            }
        }
        if (n > 1) total += n * (n - 1);
    }
    return total;
}

pub fn part1(_: std.mem.Allocator, input: []const u8) !u64 {
    return solve(1, input);
}

pub fn part2(_: std.mem.Allocator, input: []const u8) !u64 {
    return solve(2, input);
}

pub fn part3(_: std.mem.Allocator, input: []const u8) !u64 {
    return solve(3, input);
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
