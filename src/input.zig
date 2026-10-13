const std = @import("std");

pub fn readFile(io: std.Io, alloc: std.mem.Allocator, path: []const u8) ![]u8 {
    const file = try std.Io.Dir.cwd().openFile(io, path, .{});
    defer file.close(io);

    var buf: [4096]u8 = undefined;
    var reader = file.reader(io, &buf);
    return reader.interface.allocRemaining(alloc, .limited(64 * 1024 * 1024));
}

/// Tries inputs/<id>/<part>.txt first, then falls back to inputs/<id>.txt.
/// Trailing newlines are trimmed.
pub fn load(io: std.Io, alloc: std.mem.Allocator, id: []const u8, part: u8) ![]const u8 {
    const per_part = try std.fmt.allocPrint(alloc, "inputs/{s}/{d}.txt", .{ id, part });
    const data = readFile(io, alloc, per_part) catch |err| switch (err) {
        error.FileNotFound => blk: {
            const shared = try std.fmt.allocPrint(alloc, "inputs/{s}.txt", .{id});
            break :blk try readFile(io, alloc, shared);
        },
        else => return err,
    };
    return std.mem.trimEnd(u8, data, "\r\n");
}