//create a char buffer object
//implement the double buffs
//implement the CHAR_INFO buffer
mod terminal;
mod graphics;
mod rasterizer;
mod geometry;
pub use terminal::*;
pub use graphics::*;
pub use rasterizer::*;
pub use geometry::*;
use std::io::{Write, stdout};
use std::time::Duration;
use std::time::Instant;
use std::fmt::Write as fmtWrite;
use std::thread::sleep; 


fn main() {
    
    let mut world = Scene::new(1280,720);
    
    world.add_object(create_cube());
    let mut j = 0;
    
    loop{
        if j > 1500 {break;}
        j+=1;

        let timer = Instant::now();
        world.reset_frame();
        world.update();
        world.draw_objects_wireframe();
        let fps = 1.0f64 / timer.elapsed().as_secs_f64();
        let mut fps_str = String::new();
        write!(fps_str, "FPS:  {},  WIDTH: {}, HEIGHT: {}", fps,  world.console.width, world.console.height);
        world.rend.draw_string(fps_str);
        world.display();
    }
    
    //console.reset_ansi_attribs();
    //console.setup_font(8);
    //print!("{:?}", console);


}

