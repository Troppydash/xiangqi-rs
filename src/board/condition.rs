
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Condition {
    RED = 0,
    BLACK = 1,
    NONE = 2,
    DRAW = 3,
}

impl Condition {
    pub fn inverse(&self) -> Self {
        match self {
            Condition::RED => Condition::BLACK,
            Condition::BLACK => Condition::RED,
            _ => Condition::NONE
        }
    }

    pub fn display(&self) -> String {
        (match self {
            Condition::RED => "Red",
            Condition::BLACK => "Black",
            Condition::NONE => "None",
            Condition::DRAW => "Draw"
        }).to_string()
    }
    
    pub fn into(&self) -> u8 {
        *self as u8
    } 
}
