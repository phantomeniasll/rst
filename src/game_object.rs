fn enmpty_function(m: &module)
{

}
struct game_object
{
    modules : Vec<module>,
}
impl game_object
{
    fn get_module(&mut self, module_name: &'static str) -> Result<&mut module,  &'static str>
    {
        for mut module in &mut self.modules
        {
            if module.name == module_name
            {
                return Ok(&mut module);
            }
        }
        Err("Module not found")
    }

}
struct module
{
    name: &'static str,
    information: Vec<f32>,
    tick_function: fn(m: &module),
    attached_object: game_object,
}

impl module
{
    fn new_transfrom(attached_object: game_object, x: f32,y: f32) -> module
    {
        let mut information = vec!(x,y);
        module{
            attached_object: attached_object,
            name: "transform",
            information: information,
            tick_function: enmpty_function,
        }
    }


}
impl module
{
    fn move_function(&self)
    {
        let mut transform = self.attached_object.get_module("transform").unwrap();
        transform.information[0] += 0.01;
    }
    fn new_move(attached_object: game_object) -> module
    {
        module
        {
            attached_object: attached_object,
            name: "move",
            information: vec![],
            tick_function: module::move_function,
        }
    }

}