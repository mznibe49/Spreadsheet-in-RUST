use crate::coordinates::Coordinates;
use crate::rectangle::Rectangle;
use petgraph::graph::NodeIndex;
use crate::rules::Rules;

/// Structure of a cell
/// Index : representing the index of node where the cell will be residing
/// Coordinates : reprente the coordinates of the cell by (row, collumn)
/// Category : the cell type
#[derive(PartialOrd, PartialEq, Default, Debug, Clone, Copy)]
pub struct Cell {
    pub index: NodeIndex<u32>,
    pub coordinates: Coordinates,
    pub category: Category
}

/// Enum of cell's categories
/// A cell is either
/// StaticCell which containt only a value
/// OccurCell which is a cell that will count the occurence of it value in a certain area
/// FaultyCell
#[derive(PartialOrd, PartialEq, Debug, Clone, Copy)]
pub enum Category {
    StaticCell(StaticCell),
    OccurCell(OccurCell),
    FaultyCell,
}

impl Default for Category {
    fn default() -> Category {
        Category::FaultyCell
    }
}

/// Structure of a static cell
#[derive(PartialOrd, PartialEq, Debug, Clone, Copy)]
pub struct StaticCell {
    pub value: u32,
}

/// Structure of a dynamic cell
/// OccurCell contain a rectangle which represent the area where this cell will be counting it occurrence
#[derive(PartialOrd, PartialEq, Default, Debug, Clone, Copy)]
pub struct OccurCell {
    pub occurrence: u32,
    pub rectangle: Rectangle,
    pub value: u32,
}

impl Cell {

    pub fn new() -> Cell{
        Cell {
            index: NodeIndex::new(0),
            coordinates: Coordinates::new(),
            category: Category::FaultyCell,
        }
    }

    pub fn from(index: NodeIndex, coordinates: Coordinates, category: Category ) -> Cell {
        Cell {
            index,
            coordinates,
            category
        }
    }

    /// Return a build cell from formula
    pub fn from_formula(formula: &String, rule: &Rules) -> Category {
        if OccurCell::check_formula(formula, rule).unwrap() {
            let mut cell = OccurCell::new();
            cell.parse_formula(formula).unwrap();
            return Category::OccurCell(cell);
        } else if StaticCell::check_number(formula, rule).unwrap() {
            let mut cell = StaticCell::new();
            cell.value = formula.parse().unwrap();
            return Category::StaticCell(cell);
        }
        return Category::FaultyCell;
    }

    pub fn set_coordinates(&mut self, coordinates: Coordinates){
        self.coordinates = coordinates;
    }

    pub fn set_category(&mut self, new_category: Category){

        if let Category::OccurCell(occ_cell) = new_category {
            if ! occ_cell.rectangle.is_valid() {
                self.category = Category::FaultyCell;
            } else {
                self.category = new_category;
            }
        } else {
            self.category = new_category;
        }
    }

    pub fn set_index(&mut self, index: NodeIndex<u32>){
        self.index = index;
    }

    pub fn get_index(&mut self) -> NodeIndex<u32> { return  self.index;}

    pub fn get_value(&mut self) -> Option<u32> {
        match self.category {
            Category::OccurCell(occ) => Some(occ.value),
            Category::StaticCell(stat) => Some(stat.value),
            _ => None
        }
    }

    pub fn set_occurrence(&mut self, occurrence: u32) {
        if let Category::OccurCell(mut occ) = self.category {
            occ.occurrence = occurrence;
            self.category = Category::OccurCell(occ);
        }
    }

    pub fn get_occurrence(&mut self) -> Option<u32>{
        match self.category {
            Category::OccurCell(occ) => Some(occ.occurrence),
            _ => None
        }
    }

    pub fn get_coordinates(&mut self) -> Coordinates {
        return self.coordinates;
    }

    pub fn decrement_occ(&mut self) {
        match self.category {
            Category::OccurCell(mut cell) => {
                cell.occurrence -= 1;
                self.category = Category::OccurCell(cell);
            },
            _ => {},
        }
    }

    pub fn increment_occ(&mut self) {
        match self.category {
            Category::OccurCell(mut cell) => {
                cell.occurrence += 1;
                self.category = Category::OccurCell(cell);
            },
            _ => {},
        }
    }

    pub fn get_value_string(&mut self) -> String {
        match self.get_special() {
            Some(value) => value.to_string(),
            None => "P".to_string()
        }
    }

    pub fn get_special(&self) -> Option<u32> {
        match self.category {
            Category::StaticCell(cell) => Some(cell.value),
            Category::OccurCell(cell) => Some(cell.occurrence),
            Category::FaultyCell => None,
        }
    }


}

impl StaticCell {

    pub fn new() -> StaticCell {
        StaticCell::from(0)
    }

    pub fn from(value: u32) -> StaticCell {
        StaticCell {
            value
        }
    }

    pub fn check_number(number: &String, rule: &Rules) -> std::io::Result<bool> {
        let mut trimmed_str = String::from(number);
        trimmed_str.retain(|c| !c.is_whitespace());
        Ok(rule.value_regex.is_match(&trimmed_str))
    }
}

impl OccurCell {

    pub fn new() -> OccurCell {
        OccurCell {
            occurrence: 0,
            rectangle: Rectangle::new(),
            value: 0
        }
    }

    pub fn from(rectangle: Rectangle) -> OccurCell {
        OccurCell {
            occurrence: 0,
            rectangle,
            value: 0
        }
    }

    /// check if the formula respect the good syntax
    pub fn check_formula(formula: &String, rule: &Rules) -> std::io::Result<bool> {
        let mut trimmed_str = String::from(formula);
        trimmed_str.retain(|c| !c.is_whitespace());
        Ok(rule.occur_regex.is_match(&trimmed_str))
    }

    /// parse the cell from a formula
    pub fn parse_formula(&mut self, formula: &String) -> std::io::Result<bool> {
        let mut trimmed_str = String::from(formula);
        trimmed_str.retain(|c| !c.is_whitespace());

        let slice = &trimmed_str[3..trimmed_str.len() - 1];
        let splited: Vec<&str> = slice.split(",").collect();

        let c1 = (splited[0].parse::<u32>().unwrap(), splited[1].parse::<u32>().unwrap());
        let c2  = (splited[2].parse::<u32>().unwrap(), splited[3].parse::<u32>().unwrap());
        self.rectangle = Rectangle::from(Coordinates::from(c1.0,c1.1),
                                         Coordinates::from(c2.0,c2.1));
        self.value = splited[4].parse::<u32>().unwrap();

        Ok(true)
    }
}
