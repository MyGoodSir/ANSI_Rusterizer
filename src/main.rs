//create a char buffer object
//implement the double buffs
//implement the CHAR_INFO buffer
mod terminal;
mod graphics;
pub use terminal::*;
pub use graphics::*;
use winapi::um::wincon::GetConsoleWindow;
use winapi::shared::windef::HWND;
use std::io::{Write, stdout};
use std::fmt;
use winapi::um::winnt::HANDLE;
use std::time::Duration;
use std::time::Instant;
use std::ffi::OsString;
use std::os::windows::prelude::*;


fn main() {
    let mut console = Terminal::new();
    console.get_handles();
    console.setup_font(4);
    console.resize_window(1280, 720);
    console.get_terminal_size();
    console.resize_buffer(console.width as i16, console.height as i16);
    
    console.hide_cursor();
    console.clear();
    let mut rn = Renderer::new(console.width as i32, console.height as i32);
    rn.init_pbuff();
    rn.draw_circle(25, 50, 50);
    rn.bits_to_px(); 
    let pbuff = rn.px_to_string();
    let mut j = 0;
    loop{
        console.hide_cursor();
        console.clear();
        if j > 1500 {break;}
        j+=1;
        rn.draw_circle(25, 50+(j/5)%300, 10+(j)%300);
        rn.bits_to_px(); 
        print!("{}",rn.px_to_string().to_string_lossy());
        rn.reset_pbuff();
        rn.reset_bbuff();
        console.clear();
    }
    
    console.reset_ansi_attribs();
    console.setup_font(8);
    print!("{:?}", console);
}
/*

struct ansi_color{
    pub foreground:u8,
    pub background:u8
}

pub enum ansi_color_value{
    black = 0,
    red = 1,
    green = 2,
    yellow = 3,
    blue = 4,
    magenta =5,
    cyan =6,
    gray=7,
    dark_gray=8,
    bright_red=9,
    bright_green=10,
    bright_yellow=11,
    bright_blue=12,
    bright_magenta=13,
    bright_cyan=14,
    white=15,
}



fn set_foreground_color(col: &ansi_color){
    print!("\x1B[38;5;{}m", col.foreground as i32);
}

fn set_background_color(col: &ansi_color){
    print!("\x1B[48;5;{}m", col.background as i32);
}

*/
