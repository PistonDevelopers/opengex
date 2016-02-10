extern crate piston_window;
extern crate vecmath;
extern crate camera_controllers;
#[macro_use]
extern crate gfx;
extern crate gfx_device_gl;
extern crate sdl2_window;
extern crate piston_meta;
extern crate piston_meta_search;
extern crate shader_version;

use shader_version::Shaders;
use shader_version::glsl::GLSL;
use std::fs::File;
use std::io::Read;
use sdl2_window::Sdl2Window;
use piston_window::*;
use camera_controllers::{
    FirstPersonSettings,
    FirstPerson,
    CameraPerspective,
    model_view_projection
};
use gfx::traits::*;
use piston_meta::*;
use piston_meta_search::*;

//----------------------------------------
// Cube associated data

gfx_vertex_struct!( Vertex {
    a_pos: [f32; 3] = "a_pos",
});

impl Vertex {
    fn new(pos: [f32; 3]) -> Vertex {
        Vertex {
            a_pos: pos,
        }
    }
}

gfx_pipeline!( pipe {
    vbuf: gfx::VertexBuffer<Vertex> = (),
    u_model_view_proj: gfx::Global<[[f32; 4]; 4]> = "u_model_view_proj",
    out_color: gfx::RenderTarget<gfx::format::Rgba8> = "o_Color",
    out_depth: gfx::DepthTarget<gfx::format::DepthStencil> =
        gfx::preset::depth::LESS_EQUAL_WRITE,
});

//----------------------------------------

fn main() {
    let opengl = OpenGL::V3_2;

    let mut events: PistonWindow<(), Sdl2Window> =
        WindowSettings::new("piston: cube", [640, 480])
        .exit_on_esc(true)
        .samples(4)
        .opengl(opengl)
        .build()
        .unwrap();
    events.set_capture_cursor(true);

    let ref mut factory = events.factory.borrow().clone();

    // Read OpenGEX meta rules.
    let mut file_h = File::open("examples/assets/opengex-syntax.txt").unwrap();
    let mut source = String::new();
    file_h.read_to_string(&mut source).unwrap();
    let rules = stderr_unwrap(&source, syntax(&source));

    // Read cube.ogex.
    let mut file_h = File::open("examples/assets/cube.ogex").unwrap();
    let mut source = String::new();
    file_h.read_to_string(&mut source).unwrap();
    let mut data = vec![];
    stderr_unwrap(&source, parse(&rules, &source, &mut data));

    let s = Search::new(&data);
    let vertex_data: Vec<Vertex> = stderr_unwrap(&source, s.for_bool("position", true,
        |ref mut s| {
            let mut vs = Vec::with_capacity(24);
            for _ in 0 .. 24 {
                vs.push(Vertex::new([
                    try!(s.f64("x")) as f32,
                    try!(s.f64("y")) as f32,
                    try!(s.f64("z")) as f32
                ]));
            }
            Ok(vs)
        }));

    let index_data: Vec<u8> = stderr_unwrap(&source, s.for_node("IndexArray",
        |ref mut s| {
            let mut is = Vec::with_capacity(36);
            for _ in 0 .. 12 {
                is.push(try!(s.f64("a")) as u8);
                is.push(try!(s.f64("b")) as u8);
                is.push(try!(s.f64("c")) as u8);
            }
            Ok(is)
        }));

    let (vbuf, slice) = factory.create_vertex_buffer_indexed(&vertex_data,
        &index_data[..]);

    let glsl = opengl.to_glsl();
    let pso = factory.create_pipeline_simple(
            Shaders::new()
                .set(GLSL::V1_20, include_str!("assets/cube_colored_120.glslv"))
                .set(GLSL::V1_50, include_str!("assets/cube_colored_150.glslv"))
                .get(glsl).unwrap().as_bytes(),
            Shaders::new()
                .set(GLSL::V1_20, include_str!("assets/cube_colored_120.glslf"))
                .set(GLSL::V1_50, include_str!("assets/cube_colored_150.glslf"))
                .get(glsl).unwrap().as_bytes(),
            gfx::state::CullFace::Nothing,
            pipe::new()
        ).unwrap();

    let get_projection = |w: &PistonWindow<(), Sdl2Window>| {
        let draw_size = w.window.borrow().draw_size();
        CameraPerspective {
            fov: 90.0, near_clip: 0.1, far_clip: 1000.0,
            aspect_ratio: (draw_size.width as f32) / (draw_size.height as f32)
        }.projection()
    };

    let model = vecmath::mat4_id();
    let mut projection = get_projection(&events);
    let mut first_person = FirstPerson::new(
        [0.5, 0.5, 4.0],
        FirstPersonSettings::keyboard_wasd()
    );

    for e in events {
        first_person.event(&e);

        e.draw_3d(|encoder| {
            let args = e.render_args().unwrap();

            encoder.clear(&e.output_color, [0.3, 0.3, 0.3, 1.0]);
            encoder.clear_depth(&e.output_stencil, 1.0);

            let data = pipe::Data {
                vbuf: vbuf.clone(),
                u_model_view_proj: model_view_projection(
                    model,
                    first_person.camera(args.ext_dt).orthogonal(),
                    projection
                ),
                out_color: (*e.output_color).clone(),
                out_depth: (*e.output_stencil).clone(),
            };
            encoder.draw(&slice, &pso, &data);
        });
        e.draw_2d(|c, g| {
            ellipse([1.0, 1.0, 0.0, 1.0], [0.0, 0.0, 10.0, 10.0],
                c.transform, g);
        });

        if let Some(_) = e.resize_args() {
            projection = get_projection(&e);
        }
    }
}
