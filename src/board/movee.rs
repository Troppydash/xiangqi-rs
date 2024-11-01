#[derive(Hash, Debug, Clone)]
pub struct Move {
    pub startx: i8,
    pub starty: i8,
    pub endx: i8,
    pub endy: i8,
    pub captured: i8,
    pub last_capture: i32,
}

impl Move {
    pub fn new(starty: i8, startx: i8, endy: i8, endx: i8) -> Self {
        Self { startx, starty, endx, endy, captured: 0, last_capture: 0 }
    }

    pub fn display(&self) -> String {
        let cols: Vec<char> = "ABCDEFGHIJK".chars().collect();
        let rows: Vec<char> = "X987654321".chars().collect();

        format!("Move(start={}{}, end={}{})", cols[self.startx as usize], rows[self.starty as usize], cols[self.endx as usize], rows[self.endy as usize])
    }


    pub fn equals(&self, other: &Move) -> bool {
        self.compute_hash() == other.compute_hash()
    }

    pub fn compute_hash(&self) -> u64 {
        let prime = 23;
        vec![self.startx, self.starty, self.endx, self.endy].iter().fold(0, |acc, &val| acc * prime + (val as u64) + 1)
    }
}