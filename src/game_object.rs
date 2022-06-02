fn enmpty_function(m: &mut module)
{

}
fn empty_mess(m: &mut module, message: &Message)
{

}
enum Message
{
Move([f32;2])
}
struct game_object
{
    modules : Vec<module>,
}
impl game_object
{
    /*fn get_module<T>(&mut self, module_name: &'static str) -> Result<&T,  &'static str>
    {
        for mut module in &mut self.modules
        {
            if module.name == module_name
            {
                return Ok(module);
            }
        }
        Err("Module not found")
    }*/
    fn send_message(&mut self, message: Message)
    {
        for module in &mut self.modules
        {
            (module.on_message)(module, &message);
        }
    }


}
struct module
{
    name: &'static str,
    information: Vec<f32>,
    pub tick_function: fn(m: &mut module),
    pub on_message: fn(m: &mut module, message: &Message),
    attached_object: game_object,
}
fn transform_message(shelf: &mut module, message: &Message)
{
    match message
    {
        Message::Move(pos) => shelf.information = pos.to_vec()
    }
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
            on_message: transform_message,
        }
    }


}
impl module
{
    fn move_function(&mut self)
    {
        //let mut transform = self.attached_object.get_module("transform").unwrap();
        //transform.information[0] += 0.01;
        self.attached_object.send_message(Message::Move([0.2,0.2]))
    }
    fn new_move(attached_object: game_object) -> module
    {
        module
        {
            attached_object: attached_object,
            name: "move",
            information: vec![],
            tick_function: module::move_function,
            on_message: empty_mess,
        }
    }

}