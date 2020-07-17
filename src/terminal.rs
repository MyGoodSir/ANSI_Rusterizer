
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
pub fn write_buffer(&mut self, buff: [char]){

}
pub fn swap_buffers(&mut self){
    
}
}

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