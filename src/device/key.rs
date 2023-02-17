// Key struct wraps struct provided by another device interface library, currently evdev, but
// this library could be changed if compiling for a different OS, or if another library is later preferred.

pub struct Key(pub evdev::Key);

impl Key {
    #[inline]
    pub const fn new(key: evdev::Key) -> Self {
        Self(key)
    }

    #[inline]
    pub const fn code(self) -> u16 {
        self.0.code()
    }
}

impl ToString for Key {
    fn to_string(&self) -> String {
        format!("{:?}", self.0)
    }
}

impl From<u16> for Key {
    fn from(code: u16) -> Self {
        Self::new(evdev::Key::new(code))
    }
}

impl From<evdev::Key> for Key {
    fn from(key: evdev::Key) -> Self {
        Self::new(key)
    }
}
