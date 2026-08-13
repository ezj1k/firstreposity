#[derive(Debug, PartialEq)]
pub enum Levels {
    Level1,
    Level2,
    Level3,
}

impl Levels {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.trim() {
            "1" => Some(Levels::Level1),
            "2" => Some(Levels::Level2),
            "3" => Some(Levels::Level3),
            _ => None,
        }
    }
}