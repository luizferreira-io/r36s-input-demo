use std::fs::File;
use std::io::{self, Read};
use std::os::unix::io::AsRawFd;

/// Represents a joystick event in Linux Input API format
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct JsEvent {
    pub time: u32,
    pub value: i16,
    pub type_: u8,
    pub number: u8,
}

/// Maintains the current state of all joystick buttons and axes
#[derive(Debug, Clone)]
pub struct JoystickState {
    pub buttons: [bool; 32],
    pub axes: [i16; 4],
}

impl JoystickState {
    pub fn new() -> Self {
        Self {
            buttons: [false; 32],
            axes: [0; 4],
        }
    }

    /// Updates state based on a received event
    pub fn update(&mut self, event: &JsEvent) {
        const JS_EVENT_BUTTON: u8 = 0x01;
        const JS_EVENT_AXIS: u8 = 0x02;
        const JS_EVENT_INIT: u8 = 0x80;

        let event_type = event.type_ & !JS_EVENT_INIT;

        match event_type {
            JS_EVENT_BUTTON => {
                if (event.number as usize) < self.buttons.len() {
                    self.buttons[event.number as usize] = event.value != 0;
                }
            }
            JS_EVENT_AXIS => {
                if (event.number as usize) < self.axes.len() {
                    self.axes[event.number as usize] = event.value;
                }
            }
            _ => {}
        }
    }

    /// Checks if SELECT and START buttons are pressed simultaneously
    pub fn is_exit_combo_pressed(&self) -> bool {
        self.buttons[12] && self.buttons[13]
    }
}

impl Default for JoystickState {
    fn default() -> Self {
        Self::new()
    }
}

/// Manages joystick event reading
pub struct JoystickReader {
    file: File,
}

impl JoystickReader {
    /// Opens joystick device and configures non-blocking mode
    pub fn open(device_path: &str) -> io::Result<Self> {
        let file = File::open(device_path)?;
        set_nonblocking(&file)?;
        Ok(Self { file })
    }

    /// Reads next joystick event (non-blocking)
    pub fn read_event(&mut self) -> io::Result<Option<JsEvent>> {
        let mut buffer = [0u8; 8];

        match self.file.read_exact(&mut buffer) {
            Ok(_) => {
                let event = JsEvent {
                    time: u32::from_ne_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]),
                    value: i16::from_ne_bytes([buffer[4], buffer[5]]),
                    type_: buffer[6],
                    number: buffer[7],
                };
                Ok(Some(event))
            }
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => Ok(None),
            Err(e) => Err(e),
        }
    }
}

/// Configures a file descriptor for non-blocking mode
fn set_nonblocking(file: &File) -> io::Result<()> {
    let fd = file.as_raw_fd();
    unsafe {
        let flags = libc::fcntl(fd, libc::F_GETFL);
        if flags < 0 {
            return Err(io::Error::last_os_error());
        }
        if libc::fcntl(fd, libc::F_SETFL, flags | libc::O_NONBLOCK) < 0 {
            return Err(io::Error::last_os_error());
        }
    }
    Ok(())
}
