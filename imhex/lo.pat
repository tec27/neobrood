import std.array;

struct OverlayOffset {
    s8 x;
    s8 y;
};

struct FrameDef {
    OverlayOffset offsets[parent.overlay_count] [[inline]];
};

struct LoFile {
    u32 frame_count;
    u32 overlay_count;
    FrameDef* frames[frame_count]: u32 [[inline]];
};

 LoFile lo @ 0x00;