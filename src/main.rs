extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;

use gfx::{Device, Encoder};
use glutin::GlContext;

pub type Rgba8Format = gfx::format::Rgba8;
pub type DepthStencilFormat = gfx::format::DepthStencil;

const CLEAR_COLOR: [f32; 4] = [0.0, 0.25, 0.5, 1.0];

mod tracer;

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let wnd_builder = glutin::WindowBuilder::new()
        .with_title("Ice Beam - Path Tracer".to_string())
        .with_dimensions(1024, 768);
    let ctx_builder = glutin::ContextBuilder::new()
        .with_gl(glutin::GL_CORE)
        .with_vsync(true);
    let (window, mut device, mut factory, rt_view, _) =
        gfx_window_glutin::init::<Rgba8Format, DepthStencilFormat>(wnd_builder, ctx_builder, &events_loop);

    let mut encoder: Encoder<_, _> = factory.create_command_buffer().into();

    let mut running = true;
    while running {
        events_loop.poll_events(|event| {
            if let glutin::Event::WindowEvent {event, .. } = event {
                match event {
                    glutin::WindowEvent::Closed => running = false,
                    glutin::WindowEvent::KeyboardInput {
                        input: glutin::KeyboardInput {
                            virtual_keycode: Some(glutin::VirtualKeyCode::Escape), ..
                        }, ..
                    } => running = false,
                    _ => {}
                }
            }
        });

        encoder.clear(&rt_view, CLEAR_COLOR);
        encoder.flush(&mut device);

        match window.swap_buffers() {
            Ok(_) => {},
            Err(..) => running = false
        }
        device.cleanup();
    }

    println!("Hello, world!");
}
