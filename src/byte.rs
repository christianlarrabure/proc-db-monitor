pub struct Byte (pub u64);

impl Byte {
    pub fn new(value: u64) -> Byte {
        Byte(value)
    }
}


impl From<Byte> for String {
    fn from(byte: Byte) -> String {
        if byte.0 < 1024 {
            format!("{} B", byte.0)
        } else if byte.0 < 1024 * 1024 {
            format!("{:.2} KB", byte.0 as f32 / 1024.0)
        } else if byte.0 < 1024 * 1024 * 1024 {
            format!("{:.2} MB", byte.0 as f32 / 1024.0 / 1024.0)
        } else {
            format!("{:.2} GB", byte.0 as f32 / 1024.0 / 1024.0 / 1024.0)
        }
    }
}

impl std::fmt::Display for Byte {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", String::from(Byte(self.0)))
    }
}
