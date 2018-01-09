use Construct;
use kernel::windows_kernel::cursor;
use super::base_cursor::ITerminalCursor;

/// This struct will be used for cursor actions in windows terminals performed by winapi.
pub struct WinApiCursor {
    has_moved: bool,
}

impl Construct for WinApiCursor {
    fn new() -> Box<WinApiCursor> {
        Box::from(WinApiCursor { has_moved: false })
    }
}

impl ITerminalCursor for WinApiCursor {
    fn goto(&self, x: u16, y: u16) {
        cursor::set(x as i16, y as i16);
    }

    fn pos(&self) -> (i16, i16) {
        (cursor::xpos(), cursor::ypos())
    }

    fn move_up(&self, count: u16) {
        let xpos = cursor::xpos();
        let ypos = cursor::ypos();

        cursor::set(xpos, ypos - count as i16);
    }

    fn move_right(&self, count: u16) {
        let xpos = cursor::xpos();
        let ypos = cursor::ypos();

        cursor::set(xpos + count as i16, ypos);
    }

    fn move_down(&self, count: u16) {
        let xpos = cursor::xpos();
        let ypos = cursor::ypos();

        cursor::set(xpos, ypos + count as i16);
    }

    fn move_left(&self, count: u16) {
        let xpos = cursor::xpos();
        let ypos = cursor::ypos();

        cursor::set(xpos - count as i16, ypos);
    }
}