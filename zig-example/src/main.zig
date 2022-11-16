const std = @import("std");
const zm = @import("zmath");




pub fn main() anyerror!void {
    const writer = std.io.getStdOut().writer();
    const IMAGE_HEIGHT: u16 = 256;
    const IMAGE_WIDTH: u16 = 256;

    try writer.print("P3\n{} {}\n255\n", .{ IMAGE_WIDTH, IMAGE_HEIGHT });

    var j: u16 = IMAGE_HEIGHT - 1;

    while (j > 0) : (j-=1){
        var i: u16 = 0;

        while (i < IMAGE_WIDTH) : (i += 1) {
            const color = createColor(i, j, IMAGE_WIDTH, IMAGE_HEIGHT);
            // Pass a reference of the writer to writeColor
            try writeColor(&writer, color);
        }
        std.debug.print("Lines Remaining: {}\n", .{j});
    }

    std.debug.print("Done!", .{});
}

fn createColor(i: u16, j: u16, IMAGE_WIDTH: u16, IMAGE_HEIGHT: u16) zm.F32x4 {
    const r: f16 = @intToFloat(f16, i) / @intToFloat(f16, (IMAGE_WIDTH - 1));
    const g: f16 = @intToFloat(f16, j) / @intToFloat(f16, (IMAGE_HEIGHT - 1));
    const b: f16 = 0.25;

    return zm.f32x4(r, g, b, 0);
}


// Be sure to add a type after the parenthesis and before the {
pub fn createRayColor() zm.F32x4 {
}


//anytype for maximum YOLO
fn writeColor(writer: anytype, color: zm.F32x4) std.os.WriteError!void {
    const r: u16 = @floatToInt(u16, color[0] * 255.999);
    const g: u16 = @floatToInt(u16, color[1] * 255.999);
    const b: u16 = @floatToInt(u16, color[2] * 255.999);

    // .* is how pointers are dereferenced in zig
    // e.g. 
    // const val = 5
    // const ptr = &val
    // ptr.* == 5
    try writer.*.print("{} {} {}\n", .{ r, g, b });

    return;
}

test "basic test" {
    try std.testing.expectEqual(10, 3 + 7);
}
