extern crate piston_window;
extern crate vecmath;
extern crate camera_controllers;
#[macro_use]
extern crate gfx;
extern crate gfx_device_gl;
extern crate sdl2_window;
extern crate piston_meta;
extern crate piston_meta_search;

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

gfx_vertex!( Vertex {
    a_pos@ a_pos: [f32; 3],
});

impl Vertex {
    fn new(pos: [f32; 3]) -> Vertex {
        Vertex {
            a_pos: pos,
        }
    }
}

gfx_parameters!( Params {
    u_model_view_proj@ u_model_view_proj: [[f32; 4]; 4],
});

//----------------------------------------

fn main() {
    let mut events: PistonWindow<(), Sdl2Window> =
        WindowSettings::new("piston: cube", [640, 480])
        .exit_on_esc(true)
        .samples(4)
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

    let mesh = factory.create_mesh(&vertex_data);

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

    let slice = index_data.to_slice(factory, gfx::PrimitiveType::TriangleList);

    let program = {
        let vertex = gfx::ShaderSource {
            glsl_120: Some(include_bytes!("assets/cube_colored_120.glslv")),
            glsl_150: Some(include_bytes!("assets/cube_colored_150.glslv")),
            .. gfx::ShaderSource::empty()
        };
        let fragment = gfx::ShaderSource {
            glsl_120: Some(include_bytes!("assets/cube_colored_120.glslf")),
            glsl_150: Some(include_bytes!("assets/cube_colored_150.glslf")),
            .. gfx::ShaderSource::empty()
        };
        factory.link_program_source(vertex, fragment).unwrap()
    };

    let mut data = Params {
        u_model_view_proj: vecmath::mat4_id(),
        _r: std::marker::PhantomData,
    };

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
    let state = gfx::DrawState::new().depth(gfx::state::Comparison::LessEqual, true);

    for e in events {
        first_person.event(&e);

        e.draw_3d(|stream| {
            let args = e.render_args().unwrap();
            stream.clear(
                gfx::ClearData {
                    color: [0.3, 0.3, 0.3, 1.0],
                    depth: 1.0,
                    stencil: 0,
                }
            );
            data.u_model_view_proj = model_view_projection(
                model,
                first_person.camera(args.ext_dt).orthogonal(),
                projection
            );
            stream.draw(&(&mesh, slice.clone(), &program, &data, &state)).unwrap();
        });

        if let Some(_) = e.resize_args() {
            projection = get_projection(&e);
        }
    }
}
