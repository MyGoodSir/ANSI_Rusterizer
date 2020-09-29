use std::ffi::OsString;
use std::os::windows::prelude::*;
use std::fmt;
use fmt::Formatter;
pub struct Renderer{
    pub f_buff: Vec<u8>,//flag buffer ( bitwise '&' double pass on lines )
    pub b_buff: Vec<cell>, //bit buffer ( expanded y to separate top pixels from bottom )
    pub p_buff: Vec<px>,// pixel buffer array of px structs (can i use this to compress data?)
                        /*i might need this for writing color codes to output string without
                         * affecting the position of other elements in the array
                         */
	pub width: i32, pub height: i32,
}
#[derive(Copy, Clone)]
pub struct cell{
    pub active:bool,
    pub col: ColorValue
}
#[derive(Copy, Clone)]
pub struct px{
    pub has_col: bool,
    pub col: ansi_color,
    pub char: char,
}

impl fmt::Display for px{
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "{}{}\x1b[0m", self.col, self.char)
    }
}


impl Renderer{
	pub fn new(width: i32, height: i32) -> Renderer{
		
		Renderer{
            f_buff: vec![0; (width*height) as usize],
            b_buff: vec![cell{col:ColorValue::black, active:false}; (width*height*2) as usize],
            p_buff: Vec::<px>::with_capacity((width*height) as usize),
            
            width: width,
            height: height,
        }
    }

    pub fn init_pbuff(&mut self){
        for j in 0..=(self.height-1) {
            for i in 0..=self.width-1 {
                let c = ansi_color{foreground: ColorValue::black, background: ColorValue::black};
                let p = px{has_col: false, col: c, char:' '};
                self.p_buff.push(p);
            }
        }
    }

    pub fn reset_pbuff(&mut self){
        for j in 0..=(self.height-1) {
            for i in 0..=self.width-1 {
                let c = ansi_color{foreground: ColorValue::black, background: ColorValue::black};
                let p = px{has_col: false, col: c, char: ' '};
                self.p_buff[(j*self.width + i) as usize] = p;
            }
        }

    }
    pub fn reset_bbuff(&mut self){
        for j in 0..=(self.height*2-1) {
            for i in 0..=self.width-1 {
                //maybe need to change color value?
                self.b_buff[(j*self.width+i) as usize].active = false;
            }
        }
    }

    pub fn px_to_string(&mut self) -> String {
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
        str_buff
    }
    
    pub fn bits_to_px(&mut self){
        for j in 0..=(self.height*2-1) {
            for i in 0..=self.width-1 {
                match self.b_buff[(j*self.width+i) as usize].active{
                    false => (),
                    true => { 
                        self.p_buff[((j/2)*self.width + i) as usize].has_col = true;
                        self.p_buff[((j/2)*self.width + i) as usize].char = '▀';
                        match j%2{
                            1 => self.p_buff[((j/2)*self.width + i) as usize].col.background =
                                         self.b_buff[(j*self.width+i) as usize].col,

                            _ => self.p_buff[((j/2)*self.width + i) as usize].col.foreground = 
                                         self.b_buff[(j*self.width+i) as usize].col,
                        }
                    }
                }
            }
        }
    }
    pub fn draw_string(&mut self, text:String){
        for ic in text.char_indices(){
            let (i, c) = ic;
            self.p_buff[i].has_col = true;
            self.p_buff[i].col = ansi_color{foreground: ColorValue::white, background:ColorValue::black};
            self.p_buff[i].char = c;
        }
    }
    
    pub fn draw_circle(&mut self, radius: i16, cx:i32, cy:i32){
        let r2: i32 = (radius*radius) as i32;
        for j in 0..=(self.height*2-1) {
            for i in 0..=self.width-1 {
                let dist2 = (i - cx).pow(2) + (j - cy).pow(2);
                if dist2 <= r2 { 
                    self.b_buff[(j*self.width+i) as usize].active = true;  
                    self.b_buff[(j*self.width+i) as usize].col = ColorValue::green
                }
            }
        }
    }
    pub fn draw_line(&mut self, x1:i32, y1:i32, x2:i32, y2:i32, col:ColorValue){
        let dist_x = x2 - x1; 
        let dist_y = y2 - y1;
        let a_dist_x = dist_x.abs();
        let a_dist_y = dist_y.abs();
        let mut m_x = 2 * a_dist_y - a_dist_x;
        let mut m_y = 2 * a_dist_x - a_dist_y;
        if a_dist_y <= a_dist_x{
            let mut x_start:i32; let mut y_start:i32;
            let mut x_end:i32;
            if dist_x >=0 { x_start = x1; y_start = y1; x_end = x2; }
            else { x_start = x2; y_start = y2; x_end = x1; }
            
            self.b_buff[(y_start*self.width + x_start) as usize].active = true;
            self.b_buff[(y_start*self.width + x_start) as usize].col = col;

            for x in x_start..=(x_end-1){
                
                if m_x < 0 { m_x += 2 * a_dist_y; }
                else {
                    //if both dist are negative or both are positive
                    if dist_x * dist_y > 0 { y_start += 1; }
                    else { y_start -= 1; }
                    
                    m_x += 2 * (a_dist_y - a_dist_x);
                }

                self.b_buff[(y_start*self.width + x) as usize].active = true;
                self.b_buff[(y_start*self.width + x) as usize].col = col;

            }
        }
        else{
            let mut x_start:i32; let mut y_start:i32;
            let mut y_end:i32;
            if dist_y >=0 { x_start = x1; y_start = y1; y_end = y2; }
            else { x_start = x2; y_start = y2; y_end = y1; }
            
            self.b_buff[(y_start*self.width + x_start) as usize].active = true;
            self.b_buff[(y_start*self.width + x_start) as usize].col = col;

            for y in y_start..=(y_end-1){
                
                if m_y <= 0 { m_y += 2 * a_dist_x; }
                else {
                    //if both dist are negative or both are positive
                    if dist_x * dist_y > 0 { x_start += 1; }
                    else { x_start -= 1; }
                    
                    m_y += 2 * (a_dist_x - a_dist_y);
                }
                self.b_buff[(y*self.width + x_start) as usize].active = true;
                self.b_buff[(y*self.width + x_start) as usize].col = col;

            }
        }
    }

    pub fn draw_triangle(&mut self, x1:i32, y1:i32, x2:i32, y2:i32, x3:i32, y3:i32, col:ColorValue){
        self.draw_line(x1,y1,x2,y2,col);
        self.draw_line(x2,y2,x3,y3,col);
        self.draw_line(x3, y3, x1, y1,col);
    }
   
}


#[derive(Copy, Clone)]
pub struct ansi_color{
    pub foreground:ColorValue,
    pub background:ColorValue
}

impl fmt::Display for ansi_color{
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "\x1B[38;5;{}m\x1B[48;5;{}m", self.foreground as u8, self.background as u8)
    }
}
#[derive(Copy, Clone)]
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


