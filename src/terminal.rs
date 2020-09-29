

use winapi::shared::windef::{HWND, RECT};
use winapi::um::winuser::{MoveWindow, GetWindowRect};
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::um::processenv::GetStdHandle;
use winapi::um::winbase::STD_OUTPUT_HANDLE;
use winapi::um::winnt::{HANDLE};
use winapi::um::wingdi::{FF_DONTCARE, FW_NORMAL};
use std::ffi::OsString;
use std::os::windows::prelude::*;
use winapi::um::wincon::{
    GetConsoleWindow,
    WriteConsoleOutputCharacterW,
    GetConsoleScreenBufferInfo, 
    SetConsoleScreenBufferSize,
    SetCurrentConsoleFontEx,
    CONSOLE_SCREEN_BUFFER_INFO,
    CONSOLE_FONT_INFOEX,
    COORD, SMALL_RECT};

#[derive(Debug)]
pub struct Terminal{
    pub console_handles: [HANDLE; 2],
    pub width: u32, pub height:u32,
    pub wHndl: HWND, pub current_buffer: u32,

} 
impl Terminal{
    pub fn new() -> Terminal{
        unsafe{Terminal{ console_handles: [0 as HANDLE;2], 
        width: 0, height: 0, wHndl: GetConsoleWindow(), current_buffer: 0}}
    }

    pub fn get_handles(&mut self){
        let hndl = unsafe{ GetStdHandle(STD_OUTPUT_HANDLE) };
        if hndl == INVALID_HANDLE_VALUE {
            panic!("Invalid Handle!!!");
        }
        self.console_handles[0] = hndl;

    }

    pub fn get_terminal_size(&mut self){
        let hndl = self.console_handles[0];
        let cc = COORD{X:0,Y:0};
        let mut csbi = CONSOLE_SCREEN_BUFFER_INFO {
            dwSize: cc,
            dwCursorPosition: cc,
            wAttributes: 0,
            srWindow: SMALL_RECT{Left:0, Right:0, Top:0, Bottom:0},
            dwMaximumWindowSize: cc,
        };

        if unsafe{ GetConsoleScreenBufferInfo(hndl, &mut csbi) } == 0 {return ();}

        self.width = (csbi.srWindow.Right - csbi.srWindow.Left + 1) as u32;
        self.height = (csbi.srWindow.Bottom - csbi.srWindow.Top + 1) as u32;
    }

    pub fn resize_buffer(&mut self, width: i16, height: i16){
        unsafe{SetConsoleScreenBufferSize(self.console_handles[0], COORD{X:self.width as i16, Y:self.height as i16})};
    }

    pub fn resize_window(&mut self, width: i32, height: i32){
        let mut console_rect = RECT{left:0, right:0, top:0, bottom:0,};

        unsafe{GetWindowRect(self.wHndl, &mut console_rect)};
        unsafe{MoveWindow(self.wHndl, console_rect.left, console_rect.top, width, height, 0)};
    }

    pub fn write_buffer(&mut self, buff: &OsString){
        let rect = Box::new(SMALL_RECT{Top:0, Left:0, Right:self.width as i16, Bottom:self.height as i16});
        let bvec: Vec<u16> = buff.encode_wide().collect();
        unsafe{
        WriteConsoleOutputCharacterW(self.console_handles[0], 
            bvec.as_ptr() , buff.len() as u32, COORD{X:0,Y:0}, 
            Box::<SMALL_RECT>::into_raw(rect) as *mut u32);
        }

        
    }

    pub fn setup_font(&mut self, size: i16){
        unsafe{
            let fn_staging: [u16;32] = ['C' as u16,'o' as u16,'n' as u16,'s' as u16,'o' as u16,'l' as u16,'a' as u16,'s' as u16,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,] ;
            let cfi: *mut  CONSOLE_FONT_INFOEX;
            let mut cfi2 = Box::new(
                    CONSOLE_FONT_INFOEX{
                        cbSize: std::mem::size_of::<CONSOLE_FONT_INFOEX>() as u32,
                        nFont: 0,
                        dwFontSize: COORD{X:size, Y:size*2},                  // Height
                        FontFamily: FF_DONTCARE,
                        FontWeight: FW_NORMAL as u32,
                        FaceName: fn_staging, // Choose your font
                    }
                );
            cfi = &mut *cfi2;
            SetCurrentConsoleFontEx(self.console_handles[0], 0, cfi );
        }
    }

    pub fn clear(&mut self){
        print!("\x1B[2J");//clear shown
        print!("\x1B[3J");//clear backlog
        print!("\x1B[1;1H");//reset cursor to top
    }

    pub fn reset_ansi_attribs(&mut self){
        print!("\x1B[0m");
    }
    
    pub fn hide_cursor(&mut self){
        print!("\x1B[?25l");
    }
    
    pub fn show_cursor(&mut self){
        print!("\x1B[?25h");
    }
    }
