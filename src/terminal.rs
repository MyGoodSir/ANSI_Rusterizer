use std::fmt;
use winapi::ctypes::c_void;
use winapi::shared::ntdef::NULL;
use winapi::shared::windef::{HWND, RECT};
use winapi::um::winuser::{MoveWindow, GetWindowRect};
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::um::processenv::GetStdHandle;
use winapi::um::winbase::STD_OUTPUT_HANDLE;
use winapi::um::winnt::{HANDLE, CHAR};
use winapi::um::minwinbase::SECURITY_ATTRIBUTES;
use winapi::um::wingdi::{FF_DONTCARE, FW_NORMAL};
use winapi::um::wincontypes::CHAR_INFO_Char;
use std::ffi::CString;
use std::mem::size_of;
use winapi::um::wincon::{
    GetConsoleWindow,
    SetConsoleWindowInfo,
    WriteConsoleOutputCharacterA,
    GetConsoleScreenBufferInfo, 
    CreateConsoleScreenBuffer,
    SetConsoleActiveScreenBuffer,
    SetConsoleScreenBufferSize,
    SetCurrentConsoleFontEx,
    CONSOLE_SCREEN_BUFFER_INFO,
    CONSOLE_SCREEN_BUFFER_INFOEX,
    CONSOLE_FONT_INFOEX,
    CHAR_INFO, COORD, SMALL_RECT};


    const CONSOLE_TEXTMODE_BUFFER: u32 = 1u32;
    const FILE_SHARE_READ: u32 = 1u32;
    const FILE_SHARE_WRITE: u32 = 2u32;
    const GENERIC_READ: u32 = 0x80000000;
    const GENERIC_WRITE: u32 = 0x40000000;

pub struct Terminal{
    pub console_handles: [HANDLE; 2],
    pub width: u32, pub height:u32,
    pub wHndl: HWND, pub current_buffer: u32,

} 
impl Terminal{

    pub fn get_handles(&mut self){
        let hndl = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)};
        if hndl == INVALID_HANDLE_VALUE {
            panic!("Invalid Handle!!!");
        }
        self.console_handles[0] = hndl;

        //I had to hack this together for four hours before it started working.
        let s = std::mem::size_of::<SECURITY_ATTRIBUTES>();
        let address = 0x0usize;
        let lpvoid = address as *mut c_void; 
        let sv: *const SECURITY_ATTRIBUTES = &SECURITY_ATTRIBUTES {nLength: s as u32,lpSecurityDescriptor: lpvoid ,bInheritHandle: 0i32};
        let h_new_screen_buffer = unsafe{CreateConsoleScreenBuffer(GENERIC_READ | GENERIC_WRITE, FILE_SHARE_READ | FILE_SHARE_WRITE, sv, CONSOLE_TEXTMODE_BUFFER, NULL)};

        if h_new_screen_buffer == INVALID_HANDLE_VALUE {
            panic!("Invalid Handle!!!");
        }
        self.console_handles[1] = h_new_screen_buffer;


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
        unsafe{SetConsoleScreenBufferSize(self.console_handles[0], COORD{X:width, Y:height})};
        unsafe{SetConsoleScreenBufferSize(self.console_handles[1], COORD{X:width, Y:height})};
    }

    pub fn resize_window(&mut self, width: i32, height: i32){
        let mut console_rect = RECT{left:0, right:0, top:0, bottom:0,};

        unsafe{GetWindowRect(self.wHndl, &mut console_rect)};
        unsafe{MoveWindow(self.wHndl, console_rect.left, console_rect.top, width, height, 0)};
    }

    pub fn write_buffer(&mut self, buff: &CString){
        let bout: *const CHAR = buff.as_ptr() ;
        let rect = Box::new(SMALL_RECT{Top:0, Left:0, Right:self.width as i16, Bottom:self.height as i16});
        //let rectptr: *mut SMALL_RECT = Box::<SMALL_RECT>::into_raw(rect) ;
        unsafe{
        WriteConsoleOutputCharacterA(self.console_handles[self.current_buffer as usize], 
            bout, buff.as_bytes().len() as u32, COORD{X:0,Y:0}, 
            Box::<SMALL_RECT>::into_raw(rect) as *mut u32);
        }

        
    }

    pub fn swap_buffers(&mut self){
        self.current_buffer ^= 1;

        match self.current_buffer{
            0 => print!("\x1B[?1049l"),
            1 => print!("\x1B[?1049h"),
            _ => ()
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
                        dwFontSize: COORD{X:0, Y:size},                  // Height
                        FontFamily: FF_DONTCARE,
                        FontWeight: FW_NORMAL as u32,
                        FaceName: fn_staging, // Choose your font
                    }
                );
            cfi = &mut *cfi2;
            SetCurrentConsoleFontEx(GetStdHandle(STD_OUTPUT_HANDLE), 0, cfi );
        }
    }
}

/*
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

*/

/*
HANDLE hStdout, hNewScreenBuffer; 
    SMALL_RECT srctReadRect; 
    SMALL_RECT srctWriteRect; 
    CHAR_INFO chiBuffer[160]; // [2][80]; 
    COORD coordBufSize; 
    COORD coordBufCoord; 
    BOOL fSuccess; 
    CHAR_INFO chiFill;
    chiFill.Attributes = BACKGROUND_GREEN | FOREGROUND_RED; 
    chiFill.Char.AsciiChar = (char)
 
    // Get a handle to the STDOUT screen buffer to copy from and 
    // create a new screen buffer to copy to. 
 
    hStdout = GetStdHandle(STD_OUTPUT_HANDLE); 
    hNewScreenBuffer = CreateConsoleScreenBuffer( 
       GENERIC_READ |           // read/write access 
       GENERIC_WRITE, 
       FILE_SHARE_READ | 
       FILE_SHARE_WRITE,        // shared 
       NULL,                    // default security attributes 
       CONSOLE_TEXTMODE_BUFFER, // must be TEXTMODE 
       NULL);                   // reserved; must be NULL 
    if (hStdout == INVALID_HANDLE_VALUE || 
            hNewScreenBuffer == INVALID_HANDLE_VALUE) 
    {
        printf("CreateConsoleScreenBuffer failed - (%d)\n", GetLastError()); 
        return 1;
    }
 
    // Make the new screen buffer the active screen buffer. 
 
    if (! SetConsoleActiveScreenBuffer(hNewScreenBuffer) ) 
    {
        printf("SetConsoleActiveScreenBuffer failed - (%d)\n", GetLastError()); 
        return 1;
    }
 
    // Set the source rectangle. 
 
    srctReadRect.Top = 0;    // top left: row 0, col 0 
    srctReadRect.Left = 0; 
    srctReadRect.Bottom = 1; // bot. right: row 1, col 79 
    srctReadRect.Right = 79; 
 
    // The temporary buffer size is 2 rows x 80 columns. 
 
    coordBufSize.Y = 2; 
    coordBufSize.X = 80; 
 
    // The top left destination cell of the temporary buffer is 
    // row 0, col 0. 
 
    coordBufCoord.X = 0; 
    coordBufCoord.Y = 0; 
 
    // Copy the block from the screen buffer to the temp. buffer. 
 
    fSuccess = ReadConsoleOutput( 
       hStdout,        // screen buffer to read from 
       chiBuffer,      // buffer to copy into 
       coordBufSize,   // col-row size of chiBuffer 
       coordBufCoord,  // top left dest. cell in chiBuffer 
       &srctReadRect); // screen buffer source rectangle 
    if (! fSuccess) 
    {
        printf("ReadConsoleOutput failed - (%d)\n", GetLastError()); 
        return 1;
    }
 
    // Set the destination rectangle. 
 
    srctWriteRect.Top = 10;    // top lt: row 10, col 0 
    srctWriteRect.Left = 0; 
    srctWriteRect.Bottom = 11; // bot. rt: row 11, col 79 
    srctWriteRect.Right = 79; 
 
    // Copy from the temporary buffer to the new screen buffer. 
 
    fSuccess = WriteConsoleOutput( 
        hNewScreenBuffer, // screen buffer to write to 
        chiBuffer,        // buffer to copy from 
        coordBufSize,     // col-row size of chiBuffer 
        coordBufCoord,    // top left src cell in chiBuffer 
        &srctWriteRect);  // dest. screen buffer rectangle 
    if (! fSuccess) 
    {
        printf("WriteConsoleOutput failed - (%d)\n", GetLastError()); 
        return 1;
    }
    Sleep(5000); 
 
    // Restore the original active screen buffer. 
 
    if (! SetConsoleActiveScreenBuffer(hStdout)) 
    {
        printf("SetConsoleActiveScreenBuffer failed - (%d)\n", GetLastError()); 
        return 1;
    }

    return 0;
    */