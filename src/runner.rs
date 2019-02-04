
use gfx::format::Rgba8;
use gfx::format::DepthStencil;
use glutin::dpi::*;

use error;
use loader;
use gfx;
use gfx::traits::FactoryExt;

use glutin::{ElementState, MouseButton, GlContext};

use std::time::Instant;

pub enum TextureId {
    ZERO,
    ONE,
    TWO,
    THREE,
}

type ColorFormat = gfx::format::Rgba8;
type DepthFormat = gfx::format::DepthStencil;


gfx_defines! {
    vertex Vertex {
        pos: [f32; 2] = "position",
    }

    pipeline pipe {
        // Vertex buffer
        vbuf: gfx::VertexBuffer<Vertex> = (),
        // Uniforms
        i_global_time: gfx::Global<f32> = "iGlobalTime",
        i_time: gfx::Global<f32> = "iTime",
        i_resolution: gfx::Global<[f32; 3]> = "iResolution",
        i_mouse: gfx::Global<[f32; 4]> = "iMouse",
        i_frame: gfx::Global<i32> = "iFrame",
        i_channel0: gfx::TextureSampler<[f32; 4]> = "iChannel0",
        i_channel1: gfx::TextureSampler<[f32; 4]> = "iChannel1",
        i_channel2: gfx::TextureSampler<[f32; 4]> = "iChannel2",
        i_channel3: gfx::TextureSampler<[f32; 4]> = "iChannel3",

        // Output color
        frag_color: gfx::RenderTarget<ColorFormat> = "fragColor",
    }
}

const SCREEN: [Vertex; 4] = [
    Vertex{pos: [ 1.0,  1.0]}, // Top right
    Vertex{pos: [-1.0,  1.0]}, // Top left
    Vertex{pos: [-1.0, -1.0]}, // Bottom left
    Vertex{pos: [ 1.0, -1.0]}, // Bottom right
];

const SCREEN_INDICES: [u16; 6] = [
    0, 1, 2,
    0, 2, 3,
];

const CLEAR_COLOR: [f32; 4] = [1.0; 4];

pub fn run() -> error::Result<()> {

    let (mut width, mut height) = (320.0,200.0);
    let vert_src_buf = loader::load_vertex_shader();
    let frag_src_buf = loader::load_fragment_shader();
   
    let (vert_src_buf, frag_src_buf) = (vert_src_buf.as_slice(), frag_src_buf.as_slice());

    let mut events_loop = glutin::EventsLoop::new();

    let window_builder = glutin::WindowBuilder::new()
        .with_title("demolished-rs")
        .with_dimensions(LogicalSize::new(320.0, 200.0));

    let (api, version) = (glutin::Api::OpenGl, (3, 2));

    let context = glutin::ContextBuilder::new()
        .with_gl(glutin::GlRequest::Specific(api, version))
        .with_vsync(true);

   
    let (window,  mut device,  mut factory, main_color, main_depth) =
        gfx_window_glutin::init::<Rgba8, DepthStencil>(window_builder, context, &events_loop)
            .expect("Failed to create window");

    let mut encoder = gfx::Encoder::from(factory.create_command_buffer());

    let mut pso = factory.create_pipeline_simple(&vert_src_buf, &frag_src_buf, pipe::new()).unwrap();

    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&SCREEN, &SCREEN_INDICES[..]);

    // Load default textures
    let texture0 = loader::load_texture(&TextureId::ZERO, "channel0.jpg", &mut factory)?;
    let texture1 = loader::load_texture(&TextureId::ONE, "channel1.jpg", &mut factory)?;
    let texture2 = loader::load_texture(&TextureId::TWO, "channel2.jpg", &mut factory)?;
    let texture3 = loader::load_texture(&TextureId::THREE, "channel3.jpg", &mut factory)?;

    let sampler = factory.create_sampler_linear();

    let mut data = pipe::Data {
        vbuf: vertex_buffer,

        i_global_time: 0.0,
        i_time: 0.0,
        i_resolution: [width, height, width/height],
        i_mouse: [0.0; 4],
        i_frame: -1,

        i_channel0: (texture0, sampler.clone()),
        i_channel1: (texture1, sampler.clone()),
        i_channel2: (texture2, sampler.clone()),
        i_channel3: (texture3, sampler.clone()),

        frag_color: main_color,
    };

    let mut last_mouse = ElementState::Released;
    let mut current_mouse = ElementState::Released;

    let (mut mx, mut my) = (0.0, 0.0);

    let mut xyzw = [0.0; 4];

    let mut start_time = Instant::now();
    let mut running = true;

    while running {
        events_loop.poll_events(|event| {
    
            use glutin::{Event, KeyboardInput, VirtualKeyCode, WindowEvent};

            if let Event::WindowEvent { event, .. } = event {
                match event {

                    WindowEvent::CloseRequested => running = false,

                    WindowEvent::KeyboardInput {
                        input: KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                        ..
                    } => running = false,

                
                     WindowEvent::Resized(logical_size) => {
                        let dpi_factor = window.get_hidpi_factor();
                        window.resize(logical_size.to_physical(dpi_factor));
                    },
                   

                    WindowEvent::MouseInput{state, button, ..} => {
                        last_mouse = current_mouse;
                        if state == ElementState::Pressed && button == MouseButton::Left {
                            current_mouse = ElementState::Pressed;
                        } else {
                            current_mouse = ElementState::Released;
                        }
                    },

                    _ => (),
                }
            }
        });

        // Mouse
        if current_mouse == ElementState::Pressed {
            xyzw[0] = mx;
            xyzw[1] = my;
            if last_mouse == ElementState::Released {
                xyzw[2] = mx;
                xyzw[3] = my;
            }
        } else {
            xyzw[2] = 0.0;
            xyzw[3] = 0.0;
        }
        data.i_mouse = xyzw;

        // Elapsed time

        let elapsed = start_time.elapsed();
        let elapsed_ms = (elapsed.as_secs() * 1000) + u64::from(elapsed.subsec_nanos()/1_000_000);

        let elapsed_sec = (elapsed_ms as f32) / 1000.0;
        data.i_global_time = elapsed_sec;
        data.i_time = elapsed_sec;

        // Resolution
        data.i_resolution = [width, height, width/height];

        // Frame
        data.i_frame += 1;

        // Draw
        encoder.clear(&data.frag_color, CLEAR_COLOR);
        encoder.draw(&slice, &pso, &data);
        encoder.flush(&mut device);

        window.swap_buffers().unwrap();
     //   device.cleanup();
    }

    Ok(())
}
