//create a char buffer object
//write the whole buffer to console
//implement the double buffs
//implement the CHAR_INFO buffer
use winapi::um::wincon::GetConsoleWindow;
use winapi::shared::windef::HWND;

use std::io::{Write, stdout};
use std::fmt;
mod terminal;
pub use terminal::*;
use winapi::um::winnt::HANDLE;
use std::time::Duration;
use std::time::Instant;
use std::ffi::CString;

struct GraphicsTerminal{
    width: i32,
    height: i32,

}

fn main() {


    let mut console = unsafe{Terminal{ console_handles: [0 as HANDLE;2], 
        width: 0, height: 0, wHndl: GetConsoleWindow(), current_buffer: 0}};
    console.get_handles();
    hide_cursor();
    clear_terminal();
    console.setup_font(32);
    let fg = ansi_color_value::bright_green as u8;
    let bg = ansi_color_value::black as u8;
    let mut gb_profile = ansi_color{ foreground:fg, background:bg};
    set_foreground_color(&gb_profile);
    //set_background_color(&gb_profile);
    //println!("▄▄▄▄▄▄▄▄");//pixel
    
    let (mut width, mut height);
    console.resize_window(800, 600);
    console.get_terminal_size();
    width = console.width;
    height = console.height;
    //width *= 6;
    //height *= 6;
    console.resize_buffer(width as i16, height as i16);
    let bit_buff = create_bit_buff(width,height);
    let pbuff = &render_loop(width,height, &bit_buff);
    let mut j = 0;
    loop{
        clear_terminal();
        hide_cursor();
        if j > 5000 {break;}
        j+=1;

        match j%60{
            (0..=20) => gb_profile = ansi_color{ foreground: ansi_color_value::cyan as u8, background:bg},
            (20..=40) => gb_profile = ansi_color{ foreground: ansi_color_value::magenta as u8, background:bg},
            _ => gb_profile = ansi_color{ foreground: ansi_color_value::bright_green as u8, background:bg},
        }

        set_foreground_color(&gb_profile);
        console.swap_buffers();
        console.write_buffer(pbuff);
    }
    reset_ansi_attribs();
}


fn create_bit_buff(width: u32, height: u32) -> Vec<u32>{
    let mut i = 0;
    let mut buff = Vec::new();
    loop{
        buff.push(i%3);
        if i >= (width * height){ break; }
        i = i+1;
    }
    buff
}

fn render_loop(width: u32, height: u32, bit_buffer: &Vec<u32>) -> CString{
    let mut pix_buff = Vec::<u8>::new();
    let mut i = 0;
    'outer: loop{
        let mut j = 0;
        'inner: loop{
            let pos = (i*width+j) as usize;
            if (bit_buffer[pos] & (pix_flags::bottom as u32) != (pix_flags::bottom as u32)) && (bit_buffer[pos] & (pix_flags::top as u32) != (pix_flags::top as u32)){
                pix_buff.push('█' as u8);
            }else if bit_buffer[pos] & (pix_flags::bottom as u32) != (pix_flags::bottom as u32) {
                pix_buff.push('▄' as u8);
            }else{
                pix_buff.push('▀' as u8);
            }
            //print!("▄");
            j=j+1;
            if j>=width{
                break 'inner;
            }
        }
        i=i+1;
        if i>=height {
            break 'outer;
        }
    }
    let op = CString::new(pix_buff);
    match op{
        Ok(o) => o,
        Err(e) => panic!("o no")
    }
}

enum pix_flags{
    top = 0x02,
    bottom = 0x01,
}

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



fn clear_terminal(){
    print!("\x1B[2J");//clear shown
    print!("\x1B[3J");//clear backlog
    print!("\x1B[1;1H");//reset cursor to top
}

fn set_foreground_color(col: &ansi_color){
    print!("\x1B[38;5;{}m", col.foreground as i32);
}

fn set_background_color(col: &ansi_color){
    print!("\x1B[48;5;{}m", col.background as i32);
}

fn reset_ansi_attribs(){
    print!("\x1B[0m");
}

fn hide_cursor(){
    print!("\x1B[?25l");
}

fn show_cursor(){
    print!("\x1B[?25h");
}

fn alt_buff(){
    print!("\x1B[?1049h");
}

fn main_buff(){
    print!("\x1B[?1049l");
}

