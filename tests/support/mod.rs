/*!
Test supports module.

*/

#![allow(dead_code)]

use glium::{self, glutin, DisplayBuild};
use glium::backend::Facade;
use glium::index::PrimitiveType;

use std::env;

/// Builds a headless display for tests.
pub fn build_display() -> glium::Display {
    let version = parse_version();

    let display = if env::var("GLIUM_HEADLESS_TESTS").is_ok() {
        glutin::HeadlessRendererBuilder::new(1024, 768).with_gl_debug_flag(true)
                                                       .with_gl(version)
                                                       .build_glium().unwrap()
    } else {
        glutin::WindowBuilder::new().with_gl_debug_flag(true).with_visibility(false)
                                    .with_gl(version).build_glium().unwrap()
    };

    display
}

/// Rebuilds an existing display.
///
/// In real applications this is used for things such as switching to fullscreen. Some things are
/// invalidated during a rebuild, and this has to be handled by glium.
pub fn rebuild_display(display: &glium::Display) {
    let version = parse_version();

    if env::var("GLIUM_HEADLESS_TESTS").is_ok() {
        glutin::HeadlessRendererBuilder::new(1024, 768).with_gl_debug_flag(true)
                                                       .with_gl(version)
                                                       .rebuild_glium(display).unwrap();
    } else {
        glutin::WindowBuilder::new().with_gl_debug_flag(true).with_visibility(false)
                                    .with_gl(version).rebuild_glium(display).unwrap();
    }
}

fn parse_version() -> glutin::GlRequest {
    match env::var("GLIUM_GL_VERSION") {
        Ok(version) => {
            // expects "OpenGL 3.3" for example

            let mut iter = version.rsplitn(2, ' ');

            let version = iter.next().unwrap();
            let ty = iter.next().unwrap();

            let mut iter = version.split('.');
            let major = iter.next().unwrap().parse().unwrap();
            let minor = iter.next().unwrap().parse().unwrap();

            let ty = if ty == "OpenGL" {
                glutin::Api::OpenGl
            } else if ty == "OpenGL ES" {
                glutin::Api::OpenGlEs
            } else if ty == "WebGL" {
                glutin::Api::WebGl
            } else {
                panic!();
            };

            glutin::GlRequest::Specific(ty, (major, minor))
        },
        Err(_) => glutin::GlRequest::Latest,
    }
}

/// Builds a 2x2 unicolor texture.
pub fn build_unicolor_texture2d<F>(facade: &F, red: f32, green: f32, blue: f32)
    -> glium::Texture2d where F: Facade
{
    let color = ((red * 255.0) as u8, (green * 255.0) as u8, (blue * 255.0) as u8);

    glium::texture::Texture2d::new(facade, vec![
        vec![color, color],
        vec![color, color],
    ]).unwrap()
}

/// Builds a vertex buffer, index buffer, and program, to draw red `(1.0, 0.0, 0.0, 1.0)` to the whole screen.
pub fn build_fullscreen_red_pipeline<F>(facade: &F) -> (glium::vertex::VertexBuffer<Vertex>,
    glium::index::IndexBuffer<u8>, glium::Program) where F: Facade
{
    (
        glium::VertexBuffer::new(facade, &[
            Vertex { position: [-1.0,  1.0] }, Vertex { position: [1.0,  1.0] },
            Vertex { position: [-1.0, -1.0] }, Vertex { position: [1.0, -1.0] },
        ]).unwrap(),

        glium::IndexBuffer::new(facade, PrimitiveType::TriangleStrip, &[0u8, 1, 2, 3]).unwrap().into(),

        program!(facade,
            110 => {
                vertex: "
                    #version 110

                    attribute vec2 position;

                    void main() {
                        gl_Position = vec4(position, 0.0, 1.0);
                    }
                ",
                fragment: "
                    #version 110

                    void main() {
                        gl_FragColor = vec4(1.0, 0.0, 0.0, 1.0);
                    }
                ",
            },
            100 => {
                vertex: "
                    #version 100

                    attribute lowp vec2 position;

                    void main() {
                        gl_Position = vec4(position, 0.0, 1.0);
                    }
                ",
                fragment: "
                    #version 100

                    void main() {
                        gl_FragColor = vec4(1.0, 0.0, 0.0, 1.0);
                    }
                ",
            },
        ).unwrap()
    )
}

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

/// Builds a vertex buffer and an index buffer corresponding to a rectangle.
///
/// The vertex buffer has the "position" attribute of type "vec2".
pub fn build_rectangle_vb_ib<F>(facade: &F)
    -> (glium::vertex::VertexBuffer<Vertex>, glium::index::IndexBuffer<u8>) where F: Facade
{
    (
        glium::VertexBuffer::new(facade, &[
            Vertex { position: [-1.0,  1.0] }, Vertex { position: [1.0,  1.0] },
            Vertex { position: [-1.0, -1.0] }, Vertex { position: [1.0, -1.0] },
        ]).unwrap(),

        glium::IndexBuffer::new(facade, PrimitiveType::TriangleStrip, &[0u8, 1, 2, 3]).unwrap().into(),
    )
}

/// Builds a texture suitable for rendering.
pub fn build_renderable_texture<F>(facade: &F) -> glium::Texture2d where F: Facade {
    glium::Texture2d::empty(facade, 1024, 1024).unwrap()
}
