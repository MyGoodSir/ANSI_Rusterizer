
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::um::processenv::GetStdHandle;
use winapi::um::winbase::STD_OUTPUT_HANDLE;
use winapi::um::winnt::HANDLE;
use winapi::um::wincon::{GetConsoleScreenBufferInfo, 
    CONSOLE_SCREEN_BUFFER_INFO,
    PCONSOLE_SCREEN_BUFFER_INFOEX,
    CreateConsoleScreenBuffer,
    CHAR_INFO,
    SetConsoleActiveScreenBuffer, COORD, SMALL_RECT};

pub struct Terminal{
    pub console_handles: Vec<HANDLE>,
    pub width: u32, pub height:u32,

}
impl Terminal{
pub fn get_handles(&mut self){
    let hndl = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)};
    self.console_handles.push(hndl);
}
pub fn get_terminal_size(&mut self){
    let hndl = self.console_handles[0];
    if hndl == INVALID_HANDLE_VALUE {
        return ();
    }
    let cc = COORD{X:0,Y:0};
    let mut csbi = CONSOLE_SCREEN_BUFFER_INFO {
        dwSize: cc,
        dwCursorPosition: cc,
        wAttributes: 0,
        srWindow: SMALL_RECT{
            Left:0,
            Right:0,
            Top:0,
            Bottom:0,
        },
        dwMaximumWindowSize: cc,
    };
    if unsafe{ GetConsoleScreenBufferInfo(hndl, &mut csbi) } == 0 {
        return ();
    }
    self.width = (csbi.srWindow.Right - csbi.srWindow.Left + 1) as u32;
    self.height = (csbi.srWindow.Bottom - csbi.srWindow.Top + 1) as u32;
}
}