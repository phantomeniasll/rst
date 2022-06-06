// File containing my tries on 2d rendering
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