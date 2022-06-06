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


    //3d Stuff
    let positions = glium::VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
    let normals = glium::VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
    let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList,
                                      &teapot::INDICES).unwrap();


    let vertex_shader_src = r#"
    #version 150
    in vec3 position;
    in vec3 normal;

    out vec3 v_normal;
    out vec3 pos;
    uniform mat4 matrix;

    void main() {
        v_normal = transpose(inverse(mat3(matrix))) * normal;
        gl_Position = matrix * vec4(position, 1.0);
        pos = gl_Position.xyz;
    }
"#;

    let fragment_shader_src = r#"
     #version 150
    out vec4 color;
    in vec3 v_normal;
    in vec3 pos;
    uniform vec3 u_light;

    void main() {
        float brightness = dot(normalize(v_normal), normalize(u_light));
        vec4 dark_color = vec4(0.0, 0.2, cos(pos.y), 1.0);
        vec4 bright_color = vec4(1.0, 1.0, sin(pos.x), 1.0);
        color = mix(dark_color, bright_color, brightness);
    }
"#;
    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();
    let start = Instant::now();
    let light = [-1.0, 0.4, 0.9f32];


    event_loop.run(move |ev, _, control_flow| {
        let time = start.elapsed().as_secs_f32();
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        let matrix = [
            [0.01, 0.0, 0.0, 0.0],
            [0.0, 0.01, 0.0, 0.0],
            [0.0, 0.0, 0.01, 0.0],
            [0.0, 0.0, 0.0, 1.0f32]
        ];

        target.draw((&positions, &normals), &indices, &program, &uniform! { matrix: matrix, u_light: light },
            &Default::default()).unwrap();

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
