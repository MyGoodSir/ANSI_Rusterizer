use std::ffi::OsString;
use std::os::windows::prelude::*;
use std::fmt;
use fmt::Formatter;
pub struct Renderer{
    pub f_buff: Vec<u8>,//flag buffer ( bitwise '&' double pass on lines )
    pub b_buff: Vec<bool>, //bit buffer ( expanded y to separate top pixels from bottom )
    pub p_buff: Vec<px>,// pixel buffer array of px structs (can i use this to compress data?)
                        /*i might need this for writing color codes to output string without
                         * affecting the position of other elements in the array
                         */
	pub width: i32, pub height: i32,
}

pub struct cell{
    pub x: u16,
    pub y: u16,
    pub col: ColorValue
}

pub struct px{
    pub has_col: bool,
    pub col: ansi_color,

}

impl fmt::Display for px{
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "{}▀\x1b[0m", self.col)
    }
}

//

impl Renderer{
	pub fn new(width: i32, height: i32) -> Renderer{
		
		Renderer{
            f_buff: vec![0; (width*height) as usize],
            b_buff: vec![false; (width*height*2) as usize],
            p_buff: Vec::<px>::with_capacity((width*height) as usize),
            
            width: width,
            height: height,
        }
    }

    pub fn init_pbuff(&mut self){
        for j in 0..=(self.height-1) {
            for i in 0..=self.width-1 {
                let c = ansi_color{foreground: 0, background: 0};
                let p = px{has_col: false, col: c};
                self.p_buff.push(p);
            }
        }
    }

    pub fn reset_pbuff(&mut self){
        for j in 0..=(self.height-1) {
            for i in 0..=self.width-1 {
                let c = ansi_color{foreground: 0, background: 0};
                let p = px{has_col: false, col: c};
                self.p_buff[(j*self.width + i) as usize] = p;
                self.b_buff[(j*self.width + i) as usize] = false;
            }
        }

    }

    pub fn px_to_string(&mut self) -> OsString {
        let mut str_buff = String::new();
        for j in 0..=(self.height-1) {
            for i in 0..=self.width-1 {
                match self.p_buff[(j*self.width+i) as usize].has_col{
                    true => str_buff.extend(self.p_buff[(j*self.width+i) as usize].to_string().chars()),
                    _ => str_buff.push(' '),
                }
            }
            //print!("\n");
        }
        OsString::from(str_buff)
    }
    
    pub fn bits_to_px(&mut self){
        for j in 0..=(self.height*2-1) {
            for i in 0..=self.width-1 {
                match self.b_buff[(j*self.width+i) as usize]{
                    false => (),
                    true => { 
                        self.p_buff[((j/2)*self.width + i) as usize].has_col = true;
                        match j%2{
                            1 => self.p_buff[((j/2)*self.width + i) as usize].col.background = 14,
                            _ => self.p_buff[((j/2)*self.width + i) as usize].col.foreground = 14,
                        }
                    }
                }
            }
        }
    }

    
    pub fn draw_circle(&mut self, radius: i16, cx:i32, cy:i32){
        let r2: i32 = (radius*radius) as i32;
        for j in 0..=(self.height*2-1) {
            for i in 0..=self.width-1 {
                let dist2 = (i - cx).pow(2) + (j - cy).pow(2);
                if dist2 <= r2 { self.b_buff[(j*self.width+i) as usize] = true; }
            }
        }
    }
   
}


pub enum PxFlag{
    top = 0x02,
    bottom = 0x01,
}

pub struct ansi_color{
    pub foreground:u8,
    pub background:u8
}

impl fmt::Display for ansi_color{
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "\x1B[38;5;{}m\x1B[48;5;{}m", self.foreground, self.background)
    }
}

pub enum ColorValue{
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



    /*
    pub struct Cell {
    pub color: Color,
    pub char: char
}
impl Cell {
    pub fn new() -> Self {
        Self { color:Color::new(), char:' ' }
    }
}
impl Display for Cell {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "{}{}\x1b[0m", self.color, self.char)
    }
}

pub struct TPixel {
    pub cell: Cell
}
impl TPixel {
    pub fn new() -> Self {
        let mut cell = Cell::new();
        cell.char = '▀';
        Self { cell }
    }

    pub fn settop (&mut self, r:u8, g:u8, b:u8) {
        self.cell.color.setfg(r, g, b);
    }
    pub fn gettop (&self) {
        self.cell.color.getfg();
    }

    pub fn setbtm (&mut self, r:u8, g:u8, b:u8) {
        self.cell.color.setbg(r, g, b);
    }
    pub fn getbtm (&self) {
        self.cell.color.getbg();
    }
}
impl Display for TPixel {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "{}", self.cell)
    }
}
*/

/*
    pub fn set (&mut self, x:f32, y:f32, c:(u8,u8,u8)) {
        if y < self.h as f32 && x < self.w as f32 && x>=0. && y>=0. {
            let (r, g, b) = c;
            let cell = &mut self.grid[(y/2.) as usize][x as usize];
            if y as usize %2 == 0 {
                cell.settop(r, g, b)
            } else {
                cell.setbtm(r, g, b)
            }
        }
    }
    */