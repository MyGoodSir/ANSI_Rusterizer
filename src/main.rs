//create a char buffer object
//write the whole buffer to console
use std::io::{Write, stdout};
use std::fmt;
mod terminal;
pub use terminal::*;

struct GraphicsTerminal{
    width: i32,
    height: i32,

}

fn main() {


    let mut console = Terminal{ console_handles: Vec::new(), width: 0, height: 0};
    console.get_handles();
    clear_terminal();
    let fg = ansi_color_value::bright_green as u8;
    let bg = ansi_color_value::bright_blue as u8;
    let gb_profile = ansi_color{ foreground:fg, background:bg};
    set_foreground_color(&gb_profile);
    set_background_color(&gb_profile);
    //println!("▄▄▄▄▄▄▄▄");//pixel
    
    alt_buff();
    clear_terminal();
    main_buff();
    let (mut width, mut height) = (0,0);
    console.get_terminal_size();
    width = console.width;
    height = console.height;
    let bit_buff = create_bit_buff(width,height);
    let mut j = 0;
    loop{
        render_loop(width,height, &bit_buff);
        clear_terminal();
        if j > 500 {break;}
        j+=1;
    }
    reset_ansi_attribs();
}

pub struct ToFrontBuff;
pub struct ToBackBuff;

impl fmt::Display for ToFrontBuff{
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result{
        write!(fmtr, "\x1B[?1049l")
    }
}

impl fmt::Display for ToBackBuff{
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result{
        write!(fmtr, "\x1B[?1049h")
    }
}

pub struct OutBuff<W: Write>{
    t_out: W,
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

fn render_loop(width: u32, height: u32, bit_buffer: &Vec<u32>){
    let mut pix_buff = String::new();
    let mut i = 0;
    'outer: loop{
        let mut j = 0;
        'inner: loop{
            let pos = (i*width+j) as usize;
            if (bit_buffer[pos] & (pix_flags::bottom as u32) != (pix_flags::bottom as u32)) && (bit_buffer[pos] & (pix_flags::top as u32) != (pix_flags::top as u32)){
                pix_buff.push('█');
            }else if bit_buffer[pos] & (pix_flags::bottom as u32) != (pix_flags::bottom as u32) {
                pix_buff.push('▄');
            }else{
                pix_buff.push('▀');
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
    print!("{}",pix_buff);
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

fn alt_buff(){
    print!("\x1B[?1049h");
}

fn main_buff(){
    print!("\x1B[?1049l");
}

