//mod game_object;

//mod game_objects;

extern crate glium;
extern crate image;

use std::time::Instant;
use glium::{Display, Frame, implement_vertex, Program, Surface, uniform};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

impl Vertex {
    fn new(position: [f32; 2], tex_coords: [f32;2]) -> Vertex {
        Vertex {
            position: position,
            tex_coords: tex_coords,
        }
    }
}

implement_vertex!(Vertex, position, tex_coords);

struct Box
{
    position: [f32; 2],
    size: [f32; 2],
    color: [f32; 3],
}

impl Box
{
    fn new(x: f32, y: f32, w: f32, h: f32, r: f32, g: f32, b: f32) -> Box
    {
        Box {
            position: [x, y],
            size: [w, h],
            color: [r,g,b],
        }
    }
    fn to_vertex(&self) -> [Vertex; 6]
    {
        let mut vertices = [Vertex{position: [0.0 ,0.0], tex_coords: [0.0 ,0.0] };6];
        let pos = self.position;
        let size = self.size;
        vertices[0] = Vertex::new([pos[0] - size[0],pos[1] - size[1]], [0.0,0.0]); // bottom left
        vertices[1] = Vertex::new([pos[0] + size[0],pos[1] - size[1]], [1.0,0.0]); // bottom right
        vertices[2] = Vertex::new([pos[0] + size[0],pos[1] + size[1]], [1.0,1.0]); // top right
        vertices[3] = Vertex::new([pos[0] - size[0],pos[1] - size[1]], [0.0,0.0]); // bottom left
        vertices[4] = Vertex::new([pos[0] - size[0],pos[1] + size[1]], [0.0,1.0]); // top left
        vertices[5] = Vertex::new([pos[0] + size[0],pos[1] + size[1]], [1.0,1.0]); // top right
        vertices
    }
    fn draw(&self, display: &Display, mut target: Frame, shader_program: &Program, time: &f32, texture: &glium::texture::SrgbTexture2d) -> Frame
    {
        let vertex_buffer = glium::VertexBuffer::new(display, &self.to_vertex()).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        let uniforms = uniform! {
            time: time.to_owned(),
            base_color: self.color,
            matrix: [
            [1.0, 0., 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [ 0. , 0.0, 0.0, 1.0f32],
            ],
            tex: texture,
        };
        target.draw(&vertex_buffer, &indices, &shader_program, &uniforms,
                    &Default::default()).unwrap();
        return target;


    }
}

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




    let mut boxes = Vec::new();
    let my_box = Box::new(0.0, 0.0, 0.5, 0.5, 0.2,1.0, 0.0);
    let my_box2 = Box::new(0.2, 0.2, 0.5, 0.5, 1.0, 0.0, 1.0);
    boxes.push(my_box);
    boxes.push(my_box2);


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
        color = texture(tex, v_tex_coords);
    }
"#;
    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();
    let start = Instant::now();


    event_loop.run(move |ev, _, control_flow| {
        let time = start.elapsed().as_secs_f32();
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        for box_ in &boxes
        {
            target = box_.draw(&display, target, &program, &time, &texture);
        }
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
