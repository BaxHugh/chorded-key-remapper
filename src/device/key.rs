// Key struct wraps struct provided by another device interface library, currently evdev, but
// this library could be changed if compiling for a different OS, or if another library is later preferred.

pub struct Key(pub evdev::Key);

impl Key {
    #[inline]
    pub const fn new(key: evdev::Key) -> Self {
        Self(key)
    }
}

impl ToString for Key {
    fn to_string(&self) -> String {
        format!("{:?}", self.0)
    }
}
