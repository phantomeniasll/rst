//mod game_object;

//mod game_objects;

mod teapot;

extern crate glium;
extern crate image;

use std::time::Instant;
use glium::{Display, Frame, implement_vertex, Program, Surface, uniform};

fn cross_product(vec1: &[f32;3], vec2: &[f32;3]) -> [f32;3]
{
    [vec1[1] * vec2[2] - vec1[2] * vec2[1],
             vec1[2] * vec2[0] - vec1[0] * vec2[2],
             vec1[0] * vec2[1] - vec1[1] * vec2[0]]
}

fn normalize(vec: &[f32; 3]) -> [f32;3]
{
        let len = vec[0] * vec[0] + vec[1] * vec[1] + vec[2] * vec[2];
        let len = len.sqrt();
        [vec[0] / len, vec[1] / len, vec[2] / len]
}

fn view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
    // get the length of the camera direction vector and divide it by the length
    let camera_direction = normalize(direction);

    let s = cross_product(up, &camera_direction);

    let s_norm = normalize(&s);

    let u = cross_product(&camera_direction, &s_norm);

    let p = [-position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
             -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
             -position[0] * camera_direction[0] - position[1] * camera_direction[1] - position[2] * camera_direction[2]];

    [
        [s_norm[0], u[0], camera_direction[0], 0.0],
        [s_norm[1], u[1], camera_direction[1], 0.0],
        [s_norm[2], u[2], camera_direction[2], 0.0],
        [p[0], p[1], p[2], 1.0],
    ]
}


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
    out vec3 v_position;

    uniform mat4 perspective;
    uniform mat4 view;
    uniform mat4 model;

    void main() {
        mat4 modelview = view * model;
        v_normal = transpose(inverse(mat3(modelview))) * normal;
        gl_Position = perspective * modelview * vec4(position, 1.0);
        v_position = gl_Position.xyz / gl_Position.w;

    }
"#;

    let fragment_shader_src = r#"
     #version 150
    out vec4 color;
    in vec3 v_normal;
    in vec3 v_position;
    uniform vec3 u_light;
    uniform float time;

    const vec3 ambient_color = vec3(0.2, 0.0, 0.0);
    const vec3 diffuse_color = vec3(0.6, 0.0, 0.0);
    const vec3 specular_color = vec3(1.0, 1.0, 1.0);

    void main() {
        float diffuse = max(dot(normalize(v_normal), normalize(u_light)), 0.0);

        vec3 camera_dir = normalize(-v_position);
        vec3 half_direction = normalize(normalize(u_light) + camera_dir);
        float specular = pow(max(dot(half_direction, normalize(v_normal)), 0.0), 16.0);

        color = vec4(ambient_color + diffuse * diffuse_color + specular * specular_color, 1.0);

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

        let model = [
            [0.01*time.cos(), -0.01*time.sin(), 0.0,0.0],
            [0.01*time.sin(), 0.01*time.cos(), 0.0, 0.0],
            [0.0, 0.0, 0.01, 0.0],
            [0., 0., 2.0, 1.0f32]
        ];
        let view = view_matrix(&[2.0, -1.0, 1.0], &[-2.0, 1.0, 1.0], &[0.0, 1.0, 0.0]);

        target.draw((&positions, &normals), &indices, &program, &uniform! { model: model, view: view, u_light: light, perspective: perspective, time:time },
            &params).unwrap();

        target.finish().unwrap();



        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_333_333); // limit the fps
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
