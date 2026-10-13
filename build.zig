const std = @import("std");

pub fn build(b: *std.Build) !void { // $ls root_id 4
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const common = b.addModule("common", .{
        .root_source_file = b.path("src/common.zig"),
        .target = target,
        .optimize = optimize,
    });

    const test_step = b.step("test", "Run tests of common + all problems");
    test_step.dependOn(&b.addRunArtifact(b.addTest(.{ .root_module = common })).step);

    const io = b.graph.io;

    var dir = try b.root.root_dir.handle.openDir(io, "problems", .{ .iterate = true });
    defer dir.close(io);

    var walker = try dir.walk(b.allocator);
    defer walker.deinit();

    while (try walker.next(io)) |entry| {
        if (entry.kind == .directory) {
            b.dependOnDirectoryContents(b.path(b.fmt("problems/{s}", .{entry.path})));
            continue;
        }
        if (entry.kind != .file or !std.mem.endsWith(u8, entry.basename, ".zig")) continue;

        // "ec2025\q01.zig" (Windows) or "ec2025/q01.zig" -> id "ec2025/q01", step "ec2025-q01"
        const id = try b.allocator.dupe(u8, entry.path[0 .. entry.path.len - ".zig".len]);
        std.mem.replaceScalar(u8, id, '\\', '/');
        const step_name = try b.allocator.dupe(u8, id);
        std.mem.replaceScalar(u8, step_name, '/', '-');

        const problem = b.addModule(step_name, .{
            .root_source_file = b.path(b.fmt("problems/{s}.zig", .{id})),
            .target = target,
            .optimize = optimize,
            .imports = &.{.{ .name = "common", .module = common }},
        });

        const opts = b.addOptions();
        opts.addOption([]const u8, "id", id);

        const runner = b.createModule(.{
            .root_source_file = b.path("src/runner.zig"),
            .target = target,
            .optimize = optimize,
            .imports = &.{
                .{ .name = "common", .module = common },
                .{ .name = "problem", .module = problem },
                .{ .name = "build_options", .module = opts.createModule() },
            },
        });

        const exe = b.addExecutable(.{ .name = step_name, .root_module = runner });
        const run = b.addRunArtifact(exe);
        run.setCwd(b.path(".")); // so "inputs/..." paths resolve
        run.addPassthruArgs();

        const step = b.step(step_name, b.fmt("Run {s}", .{id}));
        step.dependOn(&run.step);

        // test_step.dependOn(&b.addRunArtifact(b.addTest(.{ .root_module = problem })).step);
        const problem_tests = b.addRunArtifact(b.addTest(.{ .root_module = problem }));
        test_step.dependOn(&problem_tests.step);

        const ptest = b.step(b.fmt("test-{s}", .{step_name}), b.fmt("Test {s}", .{id}));
        ptest.dependOn(&problem_tests.step);
    }
}
