//mod game_object;

//mod game_objects;

mod teapot;

extern crate glium;
extern crate image;

use std::time::Instant;
use glium::{Display, Frame, implement_vertex, Program, Surface, uniform};



fn main() {
    use glium::glutin;




    let event_loop = glutin::event_loop::EventLoop::new();

    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();


    let image = image::open(r"C:\Users\Julian\Meine Ablage\Programieren\rst\src\texture.png").unwrap().to_rgba8();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

    let texture = glium::texture::SrgbTexture2d::new(&display, image).unwrap();


    let vertex_shader_src = r#"
     #version 140
    uniform float time;
    in vec2 position;
    in vec2 tex_coords;
    out vec2 pos;
    out vec2 v_tex_coords;
    uniform mat4 matrix;
    void main() {
        pos = position;
        v_tex_coords = tex_coords;
        gl_Position = matrix * vec4(position, 0.0, 1.0);
    }
"#;

    let fragment_shader_src = r#"
     #version 140
    uniform float time;
    uniform vec3 base_color;
    uniform sampler2D tex;
    in vec2 v_tex_coords;
    in vec2 pos;
    out vec4 color;

    void main() {
        vec2 pos2 = mod(pos,0.1);
        //color = vec4(1., 0.5, mod(pos2.x * 10. + pos.y,1.)+sin(time), 1.0) * vec4(base_color,1.);
        color = texture(tex, v_tex_coords + 0.1 * vec2(sin(time),cos(time)));
    }
"#;
    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();
    let start = Instant::now();


    event_loop.run(move |ev, _, control_flow| {
        let time = start.elapsed().as_secs_f32();
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        target.finish().unwrap();



        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(33_333_333); // limit the fps
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);


        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            _ => (),
        }
});
}
