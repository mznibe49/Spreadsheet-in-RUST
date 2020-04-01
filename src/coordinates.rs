use std::hash::{Hash, Hasher};
use std::cmp::Ordering;

#[derive(Default, Debug, Clone, Copy)]
/// Structure defining the coordinates of a cell
pub struct Coordinates {
    pub row: u32,
    pub col: u32
}

impl Coordinates {
    pub fn new() -> Coordinates {
        Coordinates {
            row: 0,
            col: 0
        }
    }

    pub fn from(row: u32, col: u32) -> Coordinates {
        Coordinates {
            row,
            col
        }
    }

}

impl PartialEq for Coordinates {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.col == other.col
    }
}

impl Hash for Coordinates {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.row.hash(state);
        self.col.hash(state);
    }
}

impl Eq for Coordinates {}


impl Ord for Coordinates {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.row == other.row {
            self.col.cmp(&other.col)
        } else {
            self.row.cmp(&other.row)
        }
    }
}

impl PartialOrd for Coordinates {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


