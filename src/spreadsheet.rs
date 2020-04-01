extern crate petgraph;
use crate::cell::*;
use crate::coordinates::Coordinates;
use std::io::{BufReader, BufRead, Write, Error};
use std::fs::{File, OpenOptions};
use self::petgraph::{Directed, Graph};
use std::collections::{HashMap, BTreeMap};
use petgraph::graph::NodeIndex;
use self::petgraph::algo::{toposort};
use crate::cell::Category::FaultyCell;
use crate::rules::Rules;
use std::ops::Add;
use self::petgraph::Direction::Incoming;


/// Spreadsheet contain :
/// graph :  where to store the cells
/// evals : hashmap to store only the cell to evaluate, which mean OccurCells
/// changes : to store the changes affected by a user
pub struct SpreadSheet {
    pub graph: Graph<Cell, Cell, Directed>,
    pub evals: HashMap<Coordinates, Cell>,
    pub changes: BTreeMap<Coordinates, String>,
    pub col_max: u32,
    pub row_max: u32
}

impl SpreadSheet {

    pub fn new() -> SpreadSheet{
        SpreadSheet{
            graph: Graph::new(),
            evals: HashMap::new(),
            changes: BTreeMap::new(),
            col_max: 0,
            row_max: 0
        }
    }

    /// Function that browse "data.csv" file input in fill the graph
    pub fn browse_data(&mut self, path: &String) {
        let stream = BufReader::new(File::open(path).unwrap());
        let rule = Rules::new();

        // Index on the csv file
        let mut coordinates = Coordinates::new();
        for line in stream.lines() {

            for value in line.unwrap().split(';'){

                // Parse the data and build a brand new cell
                let mut new_cell = Cell::new();
                let new_category = Cell::from_formula(&value.to_string(), &rule);

                //println!("cat 2: {:?}",new_category);
                new_cell.set_category(new_category);
                new_cell.set_coordinates(coordinates);

                // add the cell into the graphe as a node
                let index  = self.graph.add_node(new_cell);
                new_cell.set_index(index);
                self.graph[index].set_index(index);

                // If the new cell need an evaluation, add it into the evaluation set
                if let Category::OccurCell(_cell) = new_category {
                    self.evals.insert(coordinates, new_cell);
                }

                // Move the cursor to the next column
                coordinates.col += 1;
            }
            self.col_max = coordinates.col - 1;
            coordinates.col = 0;
            // End of line, move the cursor to the next row
            coordinates.row += 1;
        }
        self.row_max = coordinates.row - 1;
    }

    /// Function that give us an index node from coordinates
    /// For exemple in a file with max_col = 4
    /// If we want to have the index node of the cell with coordinate (2, 2)
    /// It will be ((4 + 1)*2 + 2) = 12
    pub fn get_index_node_from_crd(&mut self, coordinates: &Coordinates) -> NodeIndex<u32>{
        return if coordinates.row == 0 {
            NodeIndex::new(coordinates.col as usize)
        } else {
            let res = ((self.col_max + 1) * coordinates.row) + coordinates.col;
            NodeIndex::new(res as usize)
        }
    }

    /// Function that link nodes between them
    /// An OccurCell is a father of every another cell in it rectangle (area)
    /// Every node containing an OccurCell will build an outgoing edges to it children
    pub fn link_nodes(&mut self){
        let clone = self.evals.clone();
        for (coordinates,  mut cell) in clone {
            let first_index = cell.get_index();
            let category = cell.category;
            if let Category::OccurCell(mut cat) = category {
                if ! cat.rectangle.rect_respecting_max(self.row_max, self.col_max) {
                    let index = cell.clone().get_index();
                    self.graph[index].set_category(Category::FaultyCell);
                    self.evals.remove(&coordinates);
                } else {
                    for row in cat.rectangle.begin.row..(cat.rectangle.end.row + 1) {
                        for col in cat.rectangle.begin.col..(cat.rectangle.end.col + 1){
                            let crd = Coordinates::from(row,col);
                            let second_index = self.get_index_node_from_crd(&crd);
                            self.graph.extend_with_edges(&[
                                (first_index, second_index)
                            ]);
                        }
                    }
                }
            }
        }
    }

    /// Return the family of a node from it index
    pub fn get_family(&mut self, node_index: NodeIndex<u32>, memo: &mut Vec<NodeIndex<u32>>) {
        if let Category::StaticCell(_cell) = self.graph[node_index].category{
            return;
        }
        if memo.contains(&node_index) {
            return;
        }
        memo.push(node_index);

        let clone = self.graph.clone();
        let list = clone.neighbors_undirected(node_index);
        for element in list {
            self.get_family(element,memo);
        }
    }

    /// If the graph was acyclic, return a vector of nodes in topological order: each node is ordered before its successors.
    /// Otherwise, it will return a Cycle error. Self loops are also cycles.
    pub fn check_cycle(&mut self, cloned_graph: &mut Graph<Cell, Cell, Directed>) -> Option<Vec<NodeIndex<u32>>> {
        let potential_cycle = toposort(&cloned_graph.clone(), None);
        match potential_cycle {
            Ok(_) => None, // no cycle found
            Err(e) => {
                let node_id = e.node_id(); // Return a node id that participates in the cycle
                let mut family: Vec<NodeIndex<u32>> = Vec::new();
                self.get_family(node_id,&mut family);
                Some(family)
            },
        }
    }

    /// Printing the cells from the graph
    pub fn print_cells(&mut self) {

        for index in 0..(self.graph.node_count()) {
            let mut cell =  self.graph[NodeIndex::new(index)];

            if let Category::OccurCell(_cell) = cell.category {

                println!("[OCCUR ] node with index : {} is {:?}", index , self.graph[cell.get_index()]);
                println!();

                let neighbors = self.graph.neighbors(cell.get_index());
                println!("\tCHILDREN OF INDEX NUMBER {} ARE : ",index);
                for element in neighbors {
                    println!("node with index : {} is {:?}", element.index() , self.graph[element]);
                }
                println!();

            } else {
                println!("[STATIC] node with index : {} is {:?}", index , self.graph[cell.get_index()]);
                println!();

            }
        }
    }


    /// Update the category of cells who are part of a cycle.
    /// A cell in a cycle is a FaultyCell
    /// # Arguments
    /// * 'cycle_cells' A NodeIndex vector of cell who are part of a cycle
    pub fn update_cells(&mut self, cycle_cells: Vec<NodeIndex<u32>>) {
        for key in cycle_cells.clone() {
            self.graph[key].set_category(FaultyCell);
            let coordinates = self.graph[key].coordinates;
            self.evals.remove(&coordinates);
        }

    }

    /// Browse evals hashmap and evaluates all the cells inside.
    ///
    /// First, evaluate all cells without childs into the list.
    /// Then, delete those cells from the list.
    /// Repeat until there is no cells into the list.
    ///
    /// # Return value
    /// Nothing if everything was alright, else Error.
    pub fn evaluate_all(&mut self)  {
        while !self.evals.is_empty() {

            for (crd, mut cell) in self.evals.clone().iter() {

                if let Category::OccurCell(mut cat) = cell.category {
                    if ! cat.rectangle.rect_respecting_max(self.row_max, self.col_max) {
                        let index = cell.clone().get_index();
                        self.graph[index].set_category(Category::FaultyCell);
                        self.evals.remove(&crd);
                    }
                }

                let mut occ_child = false;
                let children = self.graph.neighbors(cell.clone().get_index());
                for child_index in children {
                    let another_cell = self.graph[child_index];
                    if self.evals.contains_key(&another_cell.coordinates) {
                        occ_child = true;
                    }
                }
                if !occ_child {
                    self.evaluate_cell(&crd);
                    self.evals.remove(&crd);
                }
            }
        }
    }

    /// evaluate one cell
    pub fn evaluate_cell(&mut self, coordinates: &Coordinates)  {

        if let Some(cell) = self.evals.get_mut(coordinates) {
            // dangerous block
            // if cell block rectangle is not valid it s a faulty cell
            let children = self.graph.neighbors(cell.get_index());
            let mut occurrence = 0;
            for child in children {
                let mut child_cell = self.graph[child];
                if cell.get_value() == child_cell.get_special() {
                    occurrence += 1;
                }
            }
            if occurrence > 255 { occurrence = 255; }
            self.graph[cell.get_index()].set_occurrence(occurrence);
        }
    }

    /// Print the nodes graph in a csv file.
    ///
    /// # Arguments
    /// * 'path' - Path to the file where the data are written.
    ///
    /// # Return value
    /// Nothing if everything was alright, else Error.
    pub fn print_view(&self, path: &String) -> Result<(), Error> {
        // Open the file where all the data will be write
        let mut stream = OpenOptions::new()
            .truncate(true)
            .write(true)
            .create(true)
            .open(path)?;

        for index in 0..(self.graph.node_count()) {

            let mut cell =  self.graph[NodeIndex::new(index)];
            match cell.category {
                Category::OccurCell(_) => stream.write(&(cell.get_occurrence().unwrap() as u8)
                    .to_string()
                    .as_bytes())?,
                Category::StaticCell(_) => stream.write(&(cell.get_value().unwrap() as u8)
                    .to_string()
                    .as_bytes())?,
                _ => stream.write(&"P".as_bytes())?
            };
            if cell.coordinates.col < self.col_max {
                stream.write(&";".as_bytes())?;
            } else if cell.coordinates.col == self.col_max {
                stream.write(&"\n".as_bytes())?;
            }
        }
        Ok(())
    }


    /// Spread the informations of a new value through the spreadsheet.
    ///
    /// # Arguments
    /// * 'cell' - Reference to the new cell.
    /// * 'old_val' - Old value in the current coordinates of the new cell.
    ///
    /// # Return value
    /// Nothing if everything was alright, else Error.
    pub fn spread_information(&mut self, new_cell: &mut Cell,  old_cell_value: Option<u32>) -> Result<(), Error> {
        // If the new value equals the old value, spread the information is useless
        if new_cell.get_special() == old_cell_value {
            return Ok(());
        }

        // Insert the new value in the changes collection
        // All changes are going to be written in a file
        self.changes.insert(new_cell.coordinates, new_cell.get_value_string());


        // The changes variable stores all cells affected by the new value
        let mut local_changes: Vec<(NodeIndex<u32>, Option<u32>)> = Vec::new();

        let cloned_graph = self.graph.clone();

        // Browse all cell's parents
        // If the new value affect one on them, add it into changes
        for father in cloned_graph.neighbors_directed(new_cell.get_index(), Incoming) {
            // Instead of looking for the parent value, look for the "v" value
            // The v value is the occurrence value sought, in (r1, c1, r2, c2, v)
            // Here the v value is called ref_occur
            let mut father_cell = self.graph[father];
            //println!("A FATHER is {:?}",father_cell.clone());
            let father_value = father_cell.get_value();

            // If the new cell is a None all of it fathers should be to
            if new_cell.get_special() == None {

                let value = self.graph[father].get_special();
                local_changes.push((father, value));
                self.graph[father].set_category(Category::FaultyCell);

            } else if new_cell.get_special() == father_value {

                // If the new cell equals the ref_occur value, it's a new occurence
                // The parent's value need to be incremented
                let value = self.graph[father].get_special();
                local_changes.push((father, value));
                self.graph[father].increment_occ();

            } else if old_cell_value == father_value {

                // If the new cell is different, but the old value was equal to new value
                // It was an occurence, but now it's not
                // The parent's value need to be decremented
                let value = self.graph[father].get_special();
                local_changes.push((father, value));
                self.graph[father].decrement_occ();

            }
        }

        // Browse all changed parents and spread information for each one
        for change in local_changes {
            // here we recall with the father and

            let mut father_cell = self.graph[change.0];
            self.spread_information( &mut father_cell, change.1)?;
        }

        Ok(())
    }

    /// Insert a new cell into the spreadsheet.
    /// Then, spread the information of this new arrival.
    ///
    /// # Arguments
    /// * 'cell' - Reference to the new cell.
    /// * 'crd' - A tuple reference to the cell coordinates.
    ///
    /// # Return value
    /// Nothing if everything was alright, else Error.
    pub fn insert_spread_cell(&mut self, mut new_cell: Cell) -> Result<(), Error> {

        // Index of the node that will store the new cell
        let cell_index = self.get_index_node_from_crd(&new_cell.coordinates);
        let mut old_cell = self.graph[cell_index];

        // Add the cell into the node to erase the old one
        self.graph[cell_index].set_category(new_cell.category);
        self.graph[cell_index].set_index(old_cell.get_index());
        new_cell.set_index(cell_index);

        //println!("new cell : {:?}",new_cell);
        // Delete all outgoing edges from old node
        for child_index in self.graph.clone().neighbors(cell_index) {
            let edge = self.graph.find_edge(cell_index,child_index);
            if let Some(edge_index) = edge {
                self.graph.remove_edge(edge_index);
            }
        }

        // If the new cell is a dynamic one, we have to add it into eval
        // Then check if it create a cycle
        if let Category::OccurCell(_) = new_cell.category {

            self.evals.insert(new_cell.coordinates,new_cell);
            self.link_nodes();

            // Handling new inserted cycle by user
            let mut cloned_graph = self.graph.clone();
            let mut cell_to_update = self.check_cycle(&mut cloned_graph);

            if let Some(cycle) = cell_to_update {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(new_cell.get_index());
                // Update cell that have to be changed into FaultyCell
                self.update_cells(tmp_vec);
            } else {
                // Evaluate the new cell
                self.evaluate_cell(&new_cell.coordinates);
            }

        }

        // Spread the information of this new presence
        let mut cloned_cell = self.graph[new_cell.get_index()].clone();
        self.spread_information(&mut cloned_cell,  old_cell.get_special())?;

        Ok(())
    }

    /// Browse a "user.txt" file.
    /// Read an change, then apply it immediatly on the spreadsheet.
    /// Repeat until there is no more changes.
    ///
    /// # Arguments
    /// * 'in_path' - File path where all user changes are written
    /// * 'out_path' - File path where all spreadsheet changes are written
    ///
    /// # Return value
    /// Nothing if everything was alright, else Error.
    pub fn browse_user(&mut self, in_path: &String, out_path: &String) -> Result<(), Error> {

        let stream = BufReader::new(File::open(in_path)?);
        let rule = Rules::new();
        // Browse and parse the file
        for line in stream.lines() {


            // Extracting 'c', 'r' and 'd' from the current line
            let unwrapped_line = &line.unwrap();
            let cap = rule.user_cmd_regex.captures(unwrapped_line).unwrap();
            let row: u32 = cap.get(1).unwrap().as_str().parse().unwrap();
            let column: u32 = cap.get(2).unwrap().as_str().parse().unwrap();
            let datum = cap.get(3).unwrap().as_str().to_string();

            // Creating the new cell to insert
            let coordinates = Coordinates::from(row,column);
            let mut new_cell = Cell::new();
            let new_category = Cell::from_formula(&datum.to_string(), &rule);

            new_cell.set_category(new_category);
            new_cell.set_coordinates(coordinates);

            // Insert the new cell in the spreadsheet, then spread the information
            self.insert_spread_cell(new_cell).unwrap();

            // Print the all the changes
            self.print_changes(unwrapped_line, out_path)?;
            self.changes.clear();

        }
        Ok(())
    }

    /// Print the changes collection in a file.
    ///
    /// # Arguments
    /// * 'after' - Last user action.
    /// * 'path' - Path to the file where the changes are written.
    ///
    /// # Return value
    /// Nothing if everything was alright, else Error.
    pub fn print_changes(&self, after: &String, path: &String) -> Result<(), Error> {
        // Open the file where all changes will be write
        let mut stream = OpenOptions::new()
            .append(true)
            .write(true)
            .create(true)
            .open(path)?;

        // Print the user action before changes
        // The user action have the form "after "x y v":"
        // Where (x, y) are coordinates in the spreadsheet and v the new value
        stream.write(String::from("after \"")
            .add(after.as_ref())
            .add(&"\":\n")
            .as_bytes())?;

        // Browse the changes collection and print all changes in lexicographic order
        // All changes have the form "x y v"
        // Where (x, y) are coordinates in the spreadsheet and v the new value
        for (coordinates, val) in self.changes.clone() {
            stream.write(String::from(coordinates.row.to_string())
                .add(&" ")
                .add(&coordinates.col.to_string())
                .add(&" ")
                .add(&val.to_string())
                .add(&"\n")
                .as_bytes())?;
        }

        Ok(())
    }

    pub fn handle_cycles(&mut self, cloned_graph: &mut Graph<Cell, Cell, Directed>, memo: &mut Vec<NodeIndex<u32>>){
        let vec_index = self.check_cycle(cloned_graph);
        match vec_index {
            Some(vec) => {
                memo.extend(vec.iter().cloned());
                for elt in vec {
                    cloned_graph.remove_node(elt);
                }
                self.handle_cycles(cloned_graph,memo);
            },
            None => ()
        }
    }

    pub fn process(args: &Vec<String>)  -> Result<(), Error> {
        if args.len() != 5 {
            println!("ERROR - Wrong number of arguments");
            return Ok(());
        }

        /*if !(check::check_user_file(&args[2])?) {
            println!("ERROR - User file format incrorrect");
            return Ok(());
        }*/
        let _stream = OpenOptions::new()
            .truncate(true)
            .write(true)
            .create(true)
            .open(&args[4])?;

        let mut sheet = SpreadSheet::new();

        // Browse the data file and fill the main cells map
        sheet.browse_data(&args[1]);

        sheet.print_cells();


        // link between the graph nodes
        sheet.link_nodes();

        // The idea is to clone the main graph
        // Remove every node participating in a cycle from the cloned graph
        // We gather index of the removed nodes
        // We change them into FaultyCell in the original graph
        // That way we are handling all cycles without removing anything from our main graph :)
        let mut cloned_graph = sheet.graph.clone();
        let mut cell_to_update = Vec::new();
        sheet.handle_cycles(&mut cloned_graph, &mut cell_to_update);

        // Update cells that have to be changed into FaultyCell
        sheet.update_cells(cell_to_update);

        // Evaluate all cells
        sheet.evaluate_all();

        // Print all cell from graph
        sheet.print_view(&args[3]).unwrap();

        //sheet.print_cells();

        // Browser a file with changes, apply them on the spread sheet
        sheet.browse_user(&args[2], &args[4])?;

        Ok(())

    }


}
