const std = @import("std");
const common = @import("common");
const problem = @import("problem");
const build_options = @import("build_options");

pub fn main(init: std.process.Init) !void {
    const arena = init.arena;
    const io = init.io;

    var buf: [4096]u8 = undefined;
    var stdout = std.Io.File.stdout().writer(io, &buf);
    const out = &stdout.interface;
    defer out.flush() catch {};

    const argv = try init.minimal.args.toSlice(init.arena.allocator());
    const only: ?u8 = if (argv.len > 1) try std.fmt.parseInt(u8, argv[1], 10) else null;

    try out.print("== {s} ==\n", .{build_options.id});
    inline for (.{ 1, 2, 3 }) |part| {
        const name = std.fmt.comptimePrint("part{d}", .{part});
        if (@hasDecl(problem, name)) {
            if (only == null or only.? == part) try runPart(arena, io, out, part, @field(problem, name));
        }
    }
}

fn runPart(arena: *std.heap.ArenaAllocator, io: std.Io, out: *std.Io.Writer, comptime part: u8, comptime func: anytype) !void {
    const alloc = arena.allocator();

    const input = try common.input.load(io, alloc, build_options.id, part);

    const t0 = std.Io.Timestamp.now(io, .awake);
    const result = try func(alloc, input);
    const t1 = std.Io.Timestamp.now(io, .awake);
    const ns = t1.nanoseconds - t0.nanoseconds;

    const fmt = comptime if (@typeInfo(@TypeOf(result)) == .pointer) "{s}" else "{any}";
    try out.print("part {d}: " ++ fmt ++ "  ({d:.3} ms)\n", .{ part, result, @as(f64, @floatFromInt(ns)) / 1e6 });
}
