import std.array;

struct LocationOffset {
    s8 x;
    s8 y;
};

struct FrameDef {
    LocationOffset offsets[parent.offset_count] [[inline]];
};

struct LoFile {
    u32 frame_count;
    u32 offset_count;
    FrameDef* frames[frame_count]: u32 [[inline]];
};

 LoFile lo @ 0x00;