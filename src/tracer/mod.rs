mod tracer;

struct Size {
    width: u32,
    height: u32
}

// 32-bit floats for each channel should allow a lot of range, and then it can be converted
// into any color bit depth you want later.
struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32
}

struct Vector {
    x: f32,
    y: f32,
    z: f32
}

struct Ray {
    vec: Vector,
    bounces: i32
}

pub trait Traceable {
    fn trace();
}

trait Tracer<D : From<u8>> {
    fn render_scene(size: Size) -> [Color];
}