use std::{usize, isize};

// imports:
use rand::Rng;

/// The ```CellType``` is the material of a cell
/// # Options:
/// The materials are: ```Air```, ```Rock```, ```Wood```, ```Sand```, ```Gunpowder```, ```Water```, ```Oil```, ```Fire```, ```Lava```, ```Acid```
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CellType {
    Air,
    Rock,
    Water,
    Sand,
    Gravel,
    Wood,
    Steam,
    Gunpowder,
    Oil,
    Lava,
    Acid,
}

/// # Functionality:
/// This enum dictates how the material is processed in the function that is responsible for updating the world
/// # Options:
/// The states of aggregation are: ```ImmovableSolid```, ```Granular```, ```Liquid```, ```Gas```, ```Replaceable```
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum StateOfAggregation {
    ImmovableSolid,
    Granular,
    Liquid,
    Gas,
    Replaceable
}

/// # Functionality:
/// This struct dictates the structure and information of the look-up-array ```CELL_PROPERTIES```, which inturn dictates material behavior. ```base_temp``` is measured in ```Kelvin```
/// # Structure:
/// ```
/// pub struct CellTypeProperties {
///     pub name: &'static str,
///     pub cell_type: CellType,
///     pub state: StateOfAggregation,
///     pub density: f64,
///     pub temp_coefficient: f32,
///     pub flammable: bool,
///     pub base_color: [u8; 4]
/// }
/// ```
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CellTypeProperties {
    pub name: &'static str,
    pub cell_type: CellType,
    pub state: StateOfAggregation,
    pub density: f32,
    pub temp_coefficient: f32,
    pub flammable: bool,
    pub base_temp: u16,
    pub base_color: [u8; 4]
}

/// # Functionality:
/// This is the look-up-array for other functions to rely on
/// # Structure:
/// It's an static array of ```CellTypeProperties``` with fixed lenght
static CELL_PROPERTIES: [CellTypeProperties; 11] = [    
    CellTypeProperties { name: "Air",       cell_type: CellType::Air,       state: StateOfAggregation::Replaceable,     density: 0.0,   temp_coefficient: 1.0,      flammable: false, base_temp: 298,   base_color: [0,   0,    0, 0] },
    CellTypeProperties { name: "Rock",      cell_type: CellType::Rock,      state: StateOfAggregation::ImmovableSolid,  density: 9.0,   temp_coefficient: 0.1,      flammable: false, base_temp: 298,   base_color: [119, 136,  153, 255] },
    CellTypeProperties { name: "Water",     cell_type: CellType::Water,     state: StateOfAggregation::Liquid,          density: 1.0,   temp_coefficient: 0.1,      flammable: false, base_temp: 298,   base_color: [0, 0,  255, 255] },
    CellTypeProperties { name: "Sand",      cell_type: CellType::Sand,      state: StateOfAggregation::Granular,        density: 1.5,   temp_coefficient: 0.1,      flammable: false, base_temp: 298,   base_color: [250, 250,  210, 255] },
    CellTypeProperties { name: "Gravel",    cell_type: CellType::Gravel,    state: StateOfAggregation::Granular,        density: 3.1,   temp_coefficient: 0.1,      flammable: false, base_temp: 298,   base_color: [112, 128,  144, 255] },
    CellTypeProperties { name: "Wood",      cell_type: CellType::Wood,      state: StateOfAggregation::ImmovableSolid,  density: 1.2,   temp_coefficient: 0.1,      flammable: true,  base_temp: 298,   base_color: [139, 69,   19, 255] },
    CellTypeProperties { name: "Steam",     cell_type: CellType::Steam,     state: StateOfAggregation::Gas,             density: 0.1,   temp_coefficient: 0.1,      flammable: false, base_temp: 298,   base_color: [206, 206,  209, 255] },
    CellTypeProperties { name: "Gunpowder", cell_type: CellType::Gunpowder, state: StateOfAggregation::Granular,        density: 1.7,   temp_coefficient: 0.1,      flammable: true,  base_temp: 298,   base_color: [70, 70,    80, 255] },
    CellTypeProperties { name: "Oil",       cell_type: CellType::Oil,       state: StateOfAggregation::Liquid,          density: 0.9,   temp_coefficient: 0.1,      flammable: true,  base_temp: 298,   base_color: [55, 58,    54, 255] },
    CellTypeProperties { name: "Lava",      cell_type: CellType::Lava,      state: StateOfAggregation::Liquid,          density: 3.1,   temp_coefficient: 100.0,    flammable: false, base_temp: 298,   base_color: [255, 0,    0, 255] },
    CellTypeProperties { name: "Acid",      cell_type: CellType::Acid,      state: StateOfAggregation::Liquid,          density: 1.4,  temp_coefficient: 0.1,      flammable: false, base_temp: 298,   base_color: [0,   255,  0, 255] },
];

impl CellTypeProperties {

    /// # Functionality:
    /// This function returns a random element of the ```CELL_PROPERTIES``` array. It is not dependant on the size of ```CELL_PROPERTIES```
    /// # Panic behaviour:
    /// Panics if the length of ```CELL_PROPERTIES < 1```
    #[allow(dead_code)]
    pub fn rand_cell_properties() -> CellTypeProperties{ CELL_PROPERTIES[rand::thread_rng().gen_range(0..CELL_PROPERTIES.len())] }

    /// # Functionality:
    /// This is the highly imortant function that returns 
    /// # Panic behaviour:
    /// Panics if the length of ```CELL_PROPERTIES < 1```
    pub fn get_cell_properties<'a>(cell_type: CellType) -> &'a CellTypeProperties { &CELL_PROPERTIES[cell_type as usize] }

    pub fn get_cell_by_number(selection: &usize) -> (CellType, &'static str) {
        let a = selection % CELL_PROPERTIES.len();
        (CELL_PROPERTIES[a].cell_type, &CELL_PROPERTIES[a].name)
    }
}

/// # Functionality:
/// This general cell struct that stores cell specific data, that can vary from cell to cell
/// # Structure:
/// ```
/// pub struct Cell {
///     pub cell_type: CellType,
///     pub generation: u32,
///     pub temp: f32,
///}
/// ```
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Cell {
    pub cell_type: CellType,
    pub color: [u8; 4],
    pub generation: u32,
    pub temp: f32,
}

impl Cell {

    /// # Functionality:
    /// sets the cell to be air
    pub fn set_air() -> Cell { Cell { cell_type: CellType::Air, generation: 0, color: [0; 4], temp: 28.0 } }

    /// # Functionality:
    /// returns the CellTypeProperties struct with respect to the CellType
    pub fn get_properties<'a>(&self) -> &'a CellTypeProperties { CellTypeProperties::get_cell_properties(self.cell_type) }
}

/// # Functionality:
/// This is the general World space in which the simulation occurs
/// # Structure:
/// ```
/// pub struct Space {
///     pub width: u32,
///     pub height: u32,
///     pub lenght: i32,
///     pub generation: u32,
///     pub cells: Vec<Cell>,
/// }
/// ```
pub struct Space {
    pub width: i32,
    pub height: i32,
    pub lenght: i32,
    pub generation: u32,
    pub cells: Vec<Cell>,
}

impl Space {
    /// # Functionality:
    /// Creates a new simulation space with dimensions ```width * height``` and thus an ```index i``` of ```width * height = lenght```
    /// # Panic behaviour:
    /// panics if ```width < 0``` or ```height < 0```
    pub fn new(width: i32, height: i32) -> Self {

        // creates the cell 'list'
        let length = width * height;
        let mut cells = Vec::with_capacity(length as usize);

        // fills the list with empty cells / air
        for _ in 0..length {
            cells.push(Cell::set_air());
        }
        Space { width, height, lenght: width * height, generation: 0, cells }
    }

    /// # Functionality:
    /// Increments the simulation space's generation by one
    /// # Panic behaviour:
    /// This function will sometimes error due to ```arithmetic_overflow```, this is dependant on the unsigned intiger size.
    /// 
    /// If ```Space``` has a genation counter with the size of u8, then the application will error very quickly.
    #[allow(arithmetic_overflow)]
    pub fn increment_generation(&mut self) { self.generation += 1 }

    /// # Functionality:
    /// Sets the cell's generation to the of ```Space```
    /// # Panic behaviour:
    /// This function will sometimes error if ```i > self.lenght```
    pub fn update_cell_generation(&mut self, i: usize) { self.cells[i].generation = self.generation }

    /// # Functionality:
    /// Checks if the cell's generation is equal to the current generation of ```Space```.
    /// # Panic behaviour:
    /// This function will sometimes error if ```i > self.lenght```
    pub fn cell_needs_updating(&self, i: usize) -> bool { self.generation != self.cells[i].generation }

    /// # Functionality:
    /// Calculates the coordinates based on the index
    /// # Formula:
    /// ```
    /// let x = i as i32 % self.width;
    /// let y = (i as i32 - x) / self.width;
    /// (x, y)
    /// ```
    /// # Undefined behaviour:
    /// If ```i < 0 || i > self.lenght``` then it might cause unwanted behavior. Not really 'undefined', but usually unwanted
    pub fn get_coordinates(&self, i:isize) -> (i32, i32) {
        let x = i as i32 % self.width;
        let y = (i as i32 - x) / self.width;
        (x, y)
    }

    /// # Functionality:
    /// Returns the index based on the x and y coordinates as well as self.width
    /// # Formula:
    /// ```
    /// (x + (y * self.width));
    /// ```
    /// # Undefined behaviour:
    /// if ```i < 0 || i > self.lenght``` then it might cause unwanted behavior. Not really 'undefined', but usually unwanted
    pub fn get_index(&self, x: i32, y: i32) -> isize { (x + (y * self.width)) as isize }

    /// # Functionality:
    /// Returns the ```Option<isize>``` where ```Some()``` is the checked index ```i```
    pub fn get_index_checked(&self, x: i32, y: i32) -> Result<isize, bool> {
        if x > self.width || y > self.height || x < 0 || y < 0 {
            Err(false)
        } else {
            Ok(self.get_index(x, y))
        }
    }

    /// # Functionality:
    /// Checks if the index is within bounds. returns ```false``` if the index is not inbounds
    /// # Formula:
    /// ```
    /// if i < 0 || i > self.lenght as isize - 1 { return false }
    /// return true;
    /// ```
    pub fn index_inbounds(&mut self, i: isize) -> bool{
        i > 0 && i < self.lenght as isize - 1
    }

    /// # Functionality:
    /// Swaps two cells with the index i and j
    /// # Panic behaviour:
    /// Panics if ```self.index_inbounds(i) == false``` or ```self.index_inbounds(j) == false```
    pub fn swap_cells(&mut self, i: usize, j: usize) {

        // the swap
        let i_cell = self.cells[i];
        self.cells[i] = self.cells[j];
        self.cells[j] = i_cell;

        //mark it as updated
        self.cells[j].generation = self.generation;
        self.cells[i].generation = self.generation;
    }

    /// # Functionality:
    /// Sets a cell to a specific type
    /// # Behaviour:
    /// May cause a cell to wait too long to update, due to ```self.cells[i].generation = self.generation```
    /// # Panic behaviour:
    /// Panics if ```self.index_inbounds(i) == false``` or ```self.index_inbounds(j) == false```
    #[allow(dead_code)]
    pub fn set_cell(&mut self, i: usize, cell: &Cell) {
        
        // replace the cell
        self.cells[i] = *cell;

        //mark it as updated
        self.cells[i].generation = self.generation;
    }
    
    /// # Functionality:
    /// Sets a cell to a specific type, checks for 
    /// # Behaviour:
    /// May cause a cell to wait too long to update, due to ```self.cells[i].generation = self.generation```
    /// # Panic behaviour:
    /// Panics if ```i < 0```
    pub fn set_cell_checked(&mut self, i: usize, cell: &Cell) {

        // check the index
        if !self.index_inbounds(i as isize) { return }

        // replace the cell
        self.cells[i] = *cell;

        //mark it as updated
        self.cells[i].generation = self.generation;
    }

    /// # Functionality:
    /// Simulates all cells in ```Space```
    /// # Behaviour:
    /// Matches behavior to ```CellType``` and executes matching functions. If the cell is of the StateOfAggregation::ImmovableSolid then it is skipped
    /// # Panic behaviour:
    /// Inherits the panic behavior of all used functions and thus can make it hard to track down errors
    pub fn update_cell_behaviour(&mut self) {

        // iterate trough all elements of the Vec
        for i in 0..(self.lenght - 1) as usize {

            // needs to check if the cell needs updating
            if self.cell_needs_updating(i) {
                
                // match the cell type of index i to it's behavior
                match self.cells[i].get_properties().state {

                    // skip as there is nothing to do
                    StateOfAggregation::ImmovableSolid => (),
                    
                    // uses move_granular() to simulate sand/gravel like materials
                    StateOfAggregation::Granular => self.move_granular(i as isize, true, false),

                    // move_granular() exept it can move left and right
                    StateOfAggregation::Liquid => self.move_liquid(i as isize, true, true),

                    // baisically just reverse liquid
                    StateOfAggregation::Gas => self.move_gas(i as isize, false, true),

                    // discard all else
                    _ => (),
                }
                self.update_cell_generation(i);   
            }
        }
        self.increment_generation()
    }

    /// # Functionality:
    /// Tries to move a cell vertically. Returns a sucess bool
    /// # Behaviour:
    /// Depending on the ```gravity_normal``` bool it moves it up or down. If ```gravity_normal``` is set to true gravity is normal
    /// # Undefined behaviour:
    /// If ```i < 0 || i > self.lenght``` then it might cause unwanted behavior. Not really 'undefined', but usually unwanted
    pub fn try_move_vert(&mut self, i: isize, gravity_normal: bool) -> bool {
        
        //gets coordinates for later usage
        let xy = self.get_coordinates(i);

        // turns the gravity_normal bool into something more usable
        let dy:i32 = if gravity_normal {1} else {-1};

        match self.get_index_checked(xy.0, xy.1 + dy) {
            Ok(j) =>{

                    //check if both coordinates are inbounds
                    // once used: if !self.index_inbounds(i) { return false }
                    if !self.index_inbounds(j) { return false }
                    
                    // assign vars that are the converted indexes
                    let ii = i as usize;
                    let jj = j as usize;

                    // compares the densites of the materials and checks if its an immovable solid
                    if self.cells[ii].get_properties().density > self.cells[jj].get_properties().density && self.cells[jj].get_properties().state != StateOfAggregation::ImmovableSolid && self.cells[jj].get_properties().state != StateOfAggregation::Granular {
                        self.swap_cells(ii, jj);
                    }
                    else {
                        return false;
                    }
                    
                true
                },
            Err(_) => false,
        }
    }
    
    /// # Functionality:
    /// Checks if a cell is air or if the cell is not an ```ImmovableSolid```, and then it compares ```densities```
    /// # Structure:
    /// test for ```index_inbounds()``` && ```boundary detetection``` && ```non-ImmovableSolid```. 
    /// Then test for ```index_inbounds``` && ```density difference```
    pub fn check_sides(&mut self, i: isize) -> [bool; 4] {

        // create mutable variables that will be returned in the [bool; 4] array
        let mut left = false;
        let mut right = false;
        let mut left_less_dense = false;
        let mut right_less_dense = false;

        // test for index_inbounds() && boundary detetection && ```non-ImmovableSolid ```
        // then test for index_inbounds && density difference
        //if self.index_inbounds(i - 1) && i % self.width as isize != 0 && self.cells[(i - 1) as usize].get_properties().state != StateOfAggregation::ImmovableSolid && self.cells[(i - 1) as usize].get_properties().state != StateOfAggregation::Granular {
        //    left = true;
        //     
        //    if self.index_inbounds(i - (self.height * offset) as isize) && self.cells[i as usize - (self.height * offset) as usize].get_properties().density > self.cells[(i - 1) as usize].get_properties().density { left_less_dense = true }
        //}
        //if self.index_inbounds(i + 1) && i % self.width as isize != self.width as isize - 1 && self.cells[(i + 1) as usize].get_properties().state != StateOfAggregation::ImmovableSolid && self.cells[(i + 1) as usize].get_properties().state != StateOfAggregation::Granular {
        //    right = true;
        //     
        //    if self.index_inbounds(i - (self.height * offset) as isize) && self.cells[i as usize - (self.height * offset) as usize].get_properties().density > self.cells[(i + 1) as usize].get_properties().density { right_less_dense = true }
        //}

        if self.index_inbounds(i - 1) && self.cells[(i - 1) as usize].get_properties().state != StateOfAggregation::ImmovableSolid && self.cells[(i - 1) as usize].get_properties().state != StateOfAggregation::Granular {
            left = true;
            if self.cells[(i - 1) as usize].get_properties().density <= self.cells[i as usize].get_properties().density {
                left_less_dense = true;
            }
        }
        if self.index_inbounds(i + 1) && self.cells[(i + 1) as usize].get_properties().state != StateOfAggregation::ImmovableSolid && self.cells[(i + 1) as usize].get_properties().state != StateOfAggregation::Granular {
            right = true;
            if self.cells[(i + 1) as usize].get_properties().density <= self.cells[i as usize].get_properties().density {
                right_less_dense = true;
            }
        }
        [left, left_less_dense, right, right_less_dense]
    }
    
    /// # Functionality:
    /// Checks if a cell is air or if the cell is not an ```ImmovableSolid```, and then it compares densities
    /// # Structure:
    /// first checks if cells to the left and right and the ones below and afterwards checks for density if ```density_based == true```
    pub fn try_move_diagonally(&mut self, i: isize, gravity_normal: bool, density_based: bool) -> bool {

        // turns the gravity_normal bool into something more usable
        let dy:i32 = if gravity_normal { 1 } else { -1 };

        // handle index i and j. Check self.index_inbounds(i) and self.index_inbounds(j)
        let j = i + (dy * self.height) as isize;
        if !self.index_inbounds(i) || !self.index_inbounds(j) { return false }
        
        // ii and jj are just i and j converted to usize
        let ii = i as usize;
        let jj = j as usize;
        
        // make a new rand_bool that decides bias of swap order
        let rand_bool = rand::random::<bool>();

        
        // left, density, right, density
        let same_level_array = self.check_sides(i);
        let offset_level_array = self.check_sides(j);

        let mut can_move:[bool;2] = [false; 2];
        //let can_move:[bool; 4] = [same_level_array[0] && offset_level_array[0], same_level_array[1] && offset_level_array[1], same_level_array[2] && offset_level_array[2], same_level_array[3] && offset_level_array[3]];
        if density_based { can_move = [same_level_array[0] && offset_level_array[0] && same_level_array[1] && offset_level_array[1], same_level_array[2] && offset_level_array[2] && same_level_array[3] && offset_level_array[3]];}
        else { can_move = [same_level_array[0] && offset_level_array[0], same_level_array[2] && offset_level_array[2]];}

        if rand_bool {
            if can_move[0] {
                self.swap_cells(ii, jj - 1);
                return true;
            }
            else if can_move[1] {
                self.swap_cells(ii, jj + 1);
                return true;
            }
        }
        else if can_move[1] {
            self.swap_cells(ii, jj + 1);
            return true;
        }
        else if can_move[0] {
            self.swap_cells(ii, jj - 1);
            return true;
        }
        false
    }

    /// # Functionality:
    /// Checks if a cell is air or if the cell is not an ```ImmovableSolid```, and then it compares densities
    /// # Structure:
    /// checks if cells to the left and right and randomly selects a order of execution
    pub fn try_move_sideways(&mut self, i: isize, density_based: bool) -> bool {

        // create variables for further processing
        let same_level = self.check_sides(i);
        let rand_bool = rand::random::<bool>();

        // accounts for the density_based bool
        let can_move_left_and_right = if density_based {
            ( same_level[0] && same_level[1], same_level[2] && same_level[3] )
        }
        else { 
            (same_level[0], same_level[2]) 
        };

        // the comment below can limit the 'jiggle' that particles can experience, but makes the liquid less 'flowy'
        // if self.generation % 3 != 0 { return false }

        // select order of execution and swap if possible
        if rand_bool {
            if can_move_left_and_right.0 {
                self.swap_cells(i as usize, (i - 1) as usize );
                return true;
            }
            else if can_move_left_and_right.1 {
                self.swap_cells(i as usize, (i + 1) as usize );
                return true;
            }
        }
        else if can_move_left_and_right.1 {
            self.swap_cells(i as usize, (i + 1) as usize );
            return true;
        }
        else if can_move_left_and_right.0 {
            self.swap_cells(i as usize, (i - 1) as usize );
            return true;
        }

        false
    }

    /// # Functionality:
    /// Checks if a cell is air or if the cell is not an ImmovableSolid, and then it compares densities
    /// # Behaviour:
    /// Tries to mimik movement of granular materials by first checking below itself. And only if it can't move down it will try to move diagonally
    /// # Structure:
    /// first checks self.try_move_vert() and then self.try_move_diagonally()
    pub fn move_granular(&mut self, i: isize, gravity_normal: bool, density_based: bool) {
        if self.try_move_vert(i, gravity_normal) { return }
        self.try_move_diagonally(i, gravity_normal, density_based);
    }

    pub fn move_liquid(&mut self, i: isize, gravity_normal: bool, density_based: bool) {
        if self.try_move_vert(i, gravity_normal) { return }
        if self.try_move_diagonally(i, gravity_normal, density_based) { return }
        self.try_move_sideways(i, density_based);
    }

    pub fn move_gas(&mut self, i: isize, gravity_normal: bool, density_based: bool) {
        if self.try_move_vert(i, gravity_normal) { return }
        if self.try_move_diagonally(i, gravity_normal, density_based) { return }
        self.try_move_sideways(i, density_based);
    }

    pub fn is_solid(&self, position: (i32, i32)) -> bool {
        match self.get_index_checked(position.0 , position.1 ){
            Ok(i) => {
                if self.cells[i as usize].get_properties().state != StateOfAggregation::ImmovableSolid && self.cells[i as usize].get_properties().state != StateOfAggregation::Granular{
                    return true;
                }
                false
            },
            Err(_) => false,
        }
    }
    pub fn update_cell_alchemy(&mut self) {
        for i in 0..(self.lenght - 1) as usize {
            match self.cells[i].cell_type {
                // change the rng range for different probabilities
                CellType::Steam => if rand::thread_rng().gen_range(1..=1250) < 2 { self.set_cell_checked(i, &Cell { cell_type: CellType::Water, color: CellTypeProperties::get_cell_properties(CellType::Water).base_color, generation: 0, temp: 20.0 })},
                _ => (),
            }
        }
    }
}