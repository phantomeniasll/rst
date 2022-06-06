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
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();


    let image = image::open(r"C:\Users\Julian\Meine Ablage\Programieren\rst\src\texture.png").unwrap().to_rgba8();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);


    //3d Stuff
    let positions = glium::VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
    let normals = glium::VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
    let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList,
                                      &teapot::INDICES).unwrap();

    let params = glium::DrawParameters {
    depth: glium::Depth {
        test: glium::draw_parameters::DepthTest::IfLess,
        write: true,
        .. Default::default()
    },
    .. Default::default()
    };


    let vertex_shader_src = r#"
    #version 150
    in vec3 position;
    in vec3 normal;

    out vec3 v_normal;
    out vec3 pos;
    uniform mat4 matrix;
    uniform mat4 perspective;

    void main() {
        v_normal = transpose(inverse(mat3(matrix))) * normal;
        gl_Position = perspective * matrix * vec4(position, 1.0);
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
        vec4 dark_color = vec4(0.5, 0.1, 0.1, 1.0);
        vec4 bright_color = vec4(1.,0.3, 0.3, 1.0);
        color = mix(dark_color, bright_color, brightness);
    }
"#;
    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();
    let start = Instant::now();
    let light = [-1.0, 0.4, 0.9f32];


    event_loop.run(move |ev, _, control_flow| {
        let time = start.elapsed().as_secs_f32();


        let mut target = display.draw();
        let perspective = {
        let (width, height) = target.get_dimensions();
        let aspect_ratio = height as f32 / width as f32;

        let fov: f32 = 3.141592 / 3.0;
        let zfar = 1024.0;
        let znear = 0.1;

        let f = 1.0 / (fov / 2.0).tan();

        [
            [f *   aspect_ratio   ,    0.0,              0.0              ,   0.0],
            [         0.0         ,     f ,              0.0              ,   0.0],
            [         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
            [         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
        ]
        };
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

        let matrix = [
            [0.01, 0.0, 0.0,0.0],
            [0.0, 0.01, 0.0, 0.0],
            [0.0, 0.0, 0.01, 0.0],
            [0.0, 0.0, 2.0, 1.0f32]
        ];

        target.draw((&positions, &normals), &indices, &program, &uniform! { matrix: matrix, u_light: light, perspective: perspective },
            &params).unwrap();

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
