use crate::coordinates::Coordinates;
//use std::collections::HashSet;

#[derive(PartialOrd, PartialEq, Default, Debug, Clone, Copy)]
/// a structure defining a the area pointed by a dynamic cell
pub struct Rectangle {
    pub begin: Coordinates,
    pub end: Coordinates,
}

impl Rectangle {

    pub fn new() -> Rectangle {
        Rectangle {
            begin: Coordinates::new(),
            end: Coordinates::new(),
        }
    }

    pub fn from(begin: Coordinates, end: Coordinates) -> Rectangle {
        Rectangle {
            begin,
            end,
        }
    }


    pub fn rect_respecting_max(&mut self, row_max: u32, col_max: u32) -> bool{
        return self.end.col <= col_max && self.end.row <= row_max
    }

    pub fn is_valid(&self) -> bool {
        //println!("b_row {} e_row {} b_col {} e_col {}", self.begin.row, self.end.row, self.begin.col, self.end.col);
        (self.begin.row >= 0 && self.begin.col >= 0)
            && (self.begin.row <= self.end.row && self.begin.col <= self.end.col)
    }

}