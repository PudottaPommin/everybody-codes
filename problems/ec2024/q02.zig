const std = @import("std");
const common = @import("common");

pub fn part1(alloc: std.mem.Allocator, input: []const u8) !u64 {
    var total: u64 = 0;
    var lines = std.mem.splitScalar(u8, input, '\n');

    var words = try splitWords(alloc, lines.first());
    defer words.deinit(alloc);
    const sentence_raw = lines.rest();

    var words_total = try alloc.alloc(usize, words.items.len);
    defer alloc.free(words_total);
    @memset(words_total, 0);

    var sentence_it = std.mem.splitScalar(u8, sentence_raw, ' ');
    while (sentence_it.next()) |sentence| {
        var itTotal: u64 = 0;
        for (words.items, 0..) |w, idx| {
            const count: usize = std.mem.count(u8, sentence, w);
            words_total[idx] += count;
            itTotal += count;
        }
        total += itTotal;
    }
    return total;
}

pub fn part2(alloc: std.mem.Allocator, input: []const u8) !u64 {
    var total: u64 = 0;
    var lines_it = common.parse.lines(input);

    var search_words = try splitWords(alloc, lines_it.next().?);
    defer search_words.deinit(alloc);

    const search_words_len = search_words.items.len;
    for (0..search_words_len) |i| {
        const rev = try common.reverseSlice(u8, alloc, search_words.items[i]);
        if (!std.mem.eql(u8, search_words.items[i], rev)) {
            try search_words.append(alloc, rev);
        } else {
            alloc.free(rev);
        }
    }

    var matched_buf: [4096]bool = undefined;
    while (lines_it.next()) |l| {
        // std.debug.print("line: {s}\n", .{l});
        var line_words_it = common.splitIntoWords(l);
        while (line_words_it.next()) |lw| {
            const matched = if (lw.len <= matched_buf.len)
                matched_buf[0..lw.len]
            else
                try alloc.alloc(bool, lw.len);
            defer if (lw.len > matched_buf.len) alloc.free(matched);

            @memset(matched, false);

            for (search_words.items) |sw| {
                if (lw.len < sw.len) continue;
                var start: usize = 0;
                while (std.mem.findPos(u8, lw, start, sw)) |pos| {
                    @memset(matched[pos .. pos + sw.len], true);
                    start = pos + 1;
                }
            }

            for (matched) |m| {
                if (m) total += 1;
            }
        }
    }

    return total;
}

fn splitWords(alloc: std.mem.Allocator, l: []const u8) !std.ArrayList([]const u8) {
    const words_raw = common.trimAllWhitespaces(l);
    const idx = std.mem.findScalar(u8, words_raw, ':') orelse return error.NoWordsSplitChar;
    return common.splitByIntoArray(alloc, words_raw[idx + 1 ..], ',');
}

test "part1-test" {
    var arena: std.heap.ArenaAllocator = .init(std.testing.allocator);
    defer arena.deinit();

    {
        const input = "WORDS:THE,OWE,MES,ROD,HER\n\nAWAKEN THE POWER ADORNED WITH THE FLAMES BRIGHT IRE";
        const r = try part1(arena.allocator(), input);
        try std.testing.expectEqual(4, r);
    }
    {
        const input = "WORDS:THE,OWE,MES,ROD,HER\n\nTHE FLAME SHIELDED THE HEART OF THE KINGS";
        const r = try part1(arena.allocator(), input);
        try std.testing.expectEqual(3, r);
    }
    {
        const input = "WORDS:THE,OWE,MES,ROD,HER\n\nPOWE PO WER P OWE R";
        const r = try part1(arena.allocator(), input);
        try std.testing.expectEqual(2, r);
    }
    {
        const input = "WORDS:THE,OWE,MES,ROD,HER\n\nTHERE IS THE END";
        const r = try part1(arena.allocator(), input);
        try std.testing.expectEqual(3, r);
    }
}

test "part2-test" {
    var arena: std.heap.ArenaAllocator = .init(std.testing.allocator);
    defer arena.deinit();

    {
        const input =
            \\WORDS:THE,OWE,MES,ROD,HER,QAQ
            \\
            \\AWAKEN THE POWE ADORNED WITH THE FLAMES BRIGHT IRE
            \\THE FLAME SHIELDED THE HEART OF THE KINGS
            \\POWE PO WER P OWE R
            \\THERE IS THE END
            \\QAQAQ
        ;
        const r = try part2(arena.allocator(), input);
        try std.testing.expectEqual(42, r);
    }
}
