struct GameObject {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    color: [f32; 3],
    velocity: [i32; 2],
    acceleration: [i32; 2],
    is_visible: bool,
    is_static: bool,
    is_physics: bool,
}

impl GameObject
{
    fn new(x: i32, y: i32, width: i32, height: i32, color: [f32; 3], velocity: [i32; 2], acceleration: [i32; 2], is_visible: bool, is_static: bool, is_physics: bool) -> GameObject
    {
        GameObject {
            x: x,
            y: y,
            width: width,
            height: height,
            color: color,
            velocity: velocity,
            acceleration: acceleration,
            is_visible: is_visible,
            is_static: is_static,
            is_physics: is_physics,
        }
    }
    fn draw()
    {

    }
}




