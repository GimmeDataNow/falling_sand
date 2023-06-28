use std::{usize, isize};

// imports:
use rand::Rng;

#[allow(dead_code)]
#[derive(Debug)]
pub enum CustomErrors {
    None,
    OutOfBounds,
    UndefinedBehavior,
    CouldNotComplete
}

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
    pub fn get_cell_properties<'a>(&self) -> &'a CellTypeProperties { CellTypeProperties::get_cell_properties(self.cell_type) }
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
    /// Creates a new simulation space with dimentions ```width * height``` and thus an ```index i``` of ```width * height = lenght```
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
    /// This function will sometimes error due to ```arithmetic_overflow```, this is dependant on the maximum size of ```self.generation```.
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
    pub fn get_index_checked(&self, x: i32, y: i32) -> Result<isize, CustomErrors> {
        if x > self.width || y > self.height || x < 0 || y < 0 {
            Err(CustomErrors::OutOfBounds)
        } else {
            Ok(self.get_index(x, y))
        }
    }

    /// # Functionality:
    /// Checks if the index is within bounds. returns ```false``` if the index is not inbounds
    /// # Formula:
    /// ```
    /// i > 0 && i < self.lenght as isize - 1
    /// return true;
    /// ```
    pub fn index_inbounds(&self, i: isize) -> bool {
        i > 0 && i < self.lenght as isize
    }

    /// # Functionality:
    /// Returns the ```CellTypeProperties``` after validating if the index is inbounds
    /// # Panic behaviour:
    /// Return ```CustomErrors::OutOfBounds``` if the function fails
    pub fn get_properties_checked(&self, i: isize) -> Result<&CellTypeProperties, CustomErrors> {
        if !self.index_inbounds(i) { return Err(CustomErrors::OutOfBounds) }
        Ok(&self.cells[i as usize].get_cell_properties())
    }

    /// # Functionality:
    /// Checks if the density of i is greater than j
    /// # Panic behaviour:
    /// Return ```CustomErrors::OutOfBounds``` if the function fails
    pub fn compare_density(&self, i:isize , j: isize) -> Result<bool, CustomErrors> {
        Ok(self.get_properties_checked(i)?.density > self.get_properties_checked(j)?.density)
    }
    
    /// # Functionality:
    /// Checks if i is ```not``` ```StateOfAggregation::ImmovableSolid``` and ```StateOfAggregation::Granular```
    /// # Panic behaviour:
    /// Return ```CustomErrors::OutOfBounds``` if the function fails
    pub fn is_solid(&self, i: isize) -> Result<bool, CustomErrors> {
        Ok(self.get_properties_checked(i)?.state == StateOfAggregation::Granular || self.get_properties_checked(i)?.state == StateOfAggregation::ImmovableSolid)
    }

    /// # Functionality:
    /// Swaps two cells with the index i and j
    /// # Panic behaviour:
    /// Panics if ```self.index_inbounds(i) == false``` or ```self.index_inbounds(j) == false```
    pub fn swap_cells(&mut self, i: isize, j: isize) {

        // convert from isize to usize
        let ii = i as usize;
        let jj = j as usize;

        //simplified the swap using std::mem::swap / Vec::swap
        self.cells.swap(ii, jj);

        //mark all cells as updated
        self.cells[jj].generation = self.generation;
        self.cells[ii].generation = self.generation;
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
    /// Sets a cell to a specific type, checks for ```self.index_inbounds()```
    /// # Behaviour:
    /// May cause a cell to wait too long to update, due to ```self.cells[i].generation = self.generation```
    /// # Panic behaviour:
    /// Panics if ```i < 0```
    pub fn set_cell_checked(&mut self, i: usize, cell: &Cell) -> Result<bool, CustomErrors> {

        // check the index
        if !self.index_inbounds(i as isize) { return Err(CustomErrors::OutOfBounds) }

        // replace the cell
        self.cells[i] = *cell;

        //mark it as updated
        self.cells[i].generation = self.generation;

        // mark it as done
        Ok(true)
    }

    /// # Functionality:
    /// Simulates the movement of all cells in ```Space```
    /// # Behaviour:
    /// Matches behavior to ```CellType``` and executes matching functions. If the cell is of the ```StateOfAggregation::ImmovableSolid``` or ```StateOfAggregation::Replaceable``` type, then it will be skipped
    /// # Panic behaviour:
    /// Inherits the panic behavior of all used functions and thus can make it hard to track down errors
    pub fn update_cell_behaviour(&mut self) {

        // iterate trough all elements of the Vec
        for i in 0..(self.lenght) as usize {

            // needs to check if the cell needs updating
            if self.cell_needs_updating(i) {
                
                // match the cell type of index i to it's behavior
                match self.cells[i].get_cell_properties().state {

                    // uses move_granular() to simulate sand/gravel like materials
                    StateOfAggregation::Granular => self.move_granular(i as isize, true, false),

                    // move_granular() exept it can move left and right
                    StateOfAggregation::Liquid => self.move_liquid(i as isize, true, true),

                    // baisically just reverse liquid
                    StateOfAggregation::Gas => self.move_gas(i as isize, false, true),

                    // discard all else
                    _ => false,
                };

                // mark the cell as updated
                self.update_cell_generation(i);   
            }
        }
        // mark the space as updated and allow it to be updated again in the next iteration
        self.increment_generation()
    }

    /// # Functionality:
    /// Tries to move a cell vertically. Returns a sucess bool
    /// # Behaviour:
    /// Depending on the ```gravity_normal``` bool it moves it up or down. If ```gravity_normal``` is set to true gravity is normal
    /// # Undefined behaviour:
    /// If ```i < 0 || i > self.lenght``` then it might cause unwanted behavior. Not really 'undefined', but usually unwanted
    pub fn try_move_vert(&mut self, i: isize, gravity_normal: bool, density_based: bool) -> Result<bool, CustomErrors> {

        // turns the gravity_normal bool into something more usable
        let j = if gravity_normal { i + self.width as isize } else { i - self.width as isize };

        if !self.index_inbounds(j) { return Err(CustomErrors::OutOfBounds) }
        
        // println!("{}", !self.is_solid(j).unwrap_or(false));
        if !self.is_solid(j)? {

            if density_based && self.compare_density(i, j)? {
                self.swap_cells(i, j);
                return Ok(true);
            }
            if !density_based {
                self.swap_cells(i, j);
                return Ok(true);
            }
        }
       Err(CustomErrors::UndefinedBehavior)
    }
    
    /// # Functionality:
    /// Checks if a cell is air or if the cell is not an ```ImmovableSolid```, and then it compares ```densities```
    /// # Structure:
    /// test for ```index_inbounds()``` && ```boundary detetection``` && ```non-ImmovableSolid```. 
    /// Then test for ```index_inbounds``` && ```density difference```
    pub fn compare_sides(&mut self, ref_cell: isize, i: isize) -> [bool; 4] {

        let left_pos = i - 1;
        let right_pos = i + 1;

        let left = !self.is_solid(left_pos).unwrap_or(true);
        let right = !self.is_solid(right_pos).unwrap_or(true);
        let left_less_dense = self.compare_density(ref_cell, left_pos).unwrap_or(false);
        let right_less_dense = self.compare_density(ref_cell, right_pos).unwrap_or(false);

        // return
        [left, left_less_dense, right, right_less_dense]
    }
    
    pub fn random_move_side(&mut self, can_move: [bool; 2], i: isize, j: isize) -> Result<bool, CustomErrors> {
        // random bool to decide the direction of movement
        let rand_bool = rand::random::<bool>();

        //series of checks to move the cell
        // TODO: make this more robust and more efficient (i.e. reduce complexity)
        if rand_bool {
            if can_move[0] {
                self.swap_cells(i, j - 1);
                return Ok(true);
            }
            else if can_move[1] {
                self.swap_cells(i, j + 1);
                return Ok(true);
            }
        }
        else if can_move[1] {
            self.swap_cells(i, j + 1);
            return Ok(true);
        }
        else if can_move[0] {
            self.swap_cells(i, j - 1);
            return Ok(true);
        }

        //throw an error if this code is reached
        Err(CustomErrors::OutOfBounds)
    }
    /// # Functionality:
    /// Checks if a cell is air or if the cell is not an ```ImmovableSolid```, and then it compares densities
    /// # Structure:
    /// first checks if cells to the left and right and the ones below and afterwards checks for density if ```density_based == true```
    pub fn try_move_diagonally(&mut self, i: isize, gravity_normal: bool, density_based: bool) -> Result<bool, CustomErrors> {

        // turns the gravity_normal bool into something more usable
        let j = if gravity_normal { i + self.width as isize } else { i - self.width as isize };

        // handle index i and j. Check self.index_inbounds(i) and self.index_inbounds(j)
        if !self.index_inbounds(i) || !self.index_inbounds(j) { return Err(CustomErrors::OutOfBounds) }

        // some logic processing
        let same_level_array = self.compare_sides(i, i);
        let offset_level_array = self.compare_sides(i, j);
        let a = compare_arrays_4(same_level_array, offset_level_array);
        
        let can_move = if density_based { [a[0] && a[1], a[2] && a[3]]} else {[a[0], a[2]]};

        // swap based on rand_bool to randomise the resulting swap
        self.random_move_side(can_move, i, j)?;
        
        //if anyting errors then this is undefined behavior
        Err(CustomErrors::UndefinedBehavior)
    }

    /// # Functionality:
    /// Checks if a cell is air or if the cell is not an ```ImmovableSolid```, and then it compares densities
    /// # Structure:
    /// checks if cells to the left and right and randomly selects a order of execution
    pub fn try_move_sideways(&mut self, i: isize, density_based: bool) -> Result<bool, CustomErrors> {

        // create variables for further processing
        let same_level = self.compare_sides(i, i);

        // accounts for the density_based bool
        let can_move = if density_based {
            [same_level[0] && same_level[1], same_level[2] && same_level[3]]
        }
        else { 
            [same_level[0], same_level[2]]
        };

        // swap based on rand_bool to randomise the resulting swap
        self.random_move_side(can_move, i, i)?;

        //if anyting errors then this is undefined behavior
        Err(CustomErrors::UndefinedBehavior)
    }

    /// # Functionality:
    /// First the fuction tries to move a cell vertically. Then the fuction tries to move a cell diagonally. Should the function fail it returns false
    /// # Behaviour:
    /// Tries to mimik movement of granular materials by first checking below itself. And only if it can't move down it will try to move diagonally
    /// # Structure:
    /// first checks ```self.try_move_vert()``` and then ```self.try_move_diagonally()```
    pub fn move_granular(&mut self, i: isize, gravity_normal: bool, density_based: bool) -> bool {
        if self.try_move_vert(i, gravity_normal, density_based).unwrap_or(false) { return true }
        self.try_move_diagonally(i, gravity_normal, density_based).unwrap_or(false)
    }

    /// # Functionality:
    /// Checks if a cell is air or if the cell is not an ImmovableSolid, and then it compares densities
    /// # Behaviour:
    /// Tries to mimik movement of granular materials by first checking below itself. And only if it can't move down it will try to move diagonally
    /// # Structure:
    /// first checks self.try_move_vert() and then self.try_move_diagonally()
    pub fn move_liquid(&mut self, i: isize, gravity_normal: bool, density_based: bool) -> bool {
        if self.try_move_vert(i, gravity_normal, density_based).unwrap_or(false) { return true }
        if self.try_move_diagonally(i, gravity_normal, density_based).unwrap_or(false) { return true }
        self.try_move_sideways(i, density_based).unwrap_or(false)
    }

    pub fn move_gas(&mut self, i: isize, gravity_normal: bool, density_based: bool) -> bool {
        if self.try_move_vert(i, gravity_normal, density_based).unwrap_or(false) { return true }
        if self.try_move_diagonally(i, gravity_normal, density_based).unwrap_or(false) { return true }
        self.try_move_sideways(i, density_based).unwrap_or(false)
    }

    /// # Functionality:
    /// This function is the backbone for all alchemical reactions
    /// # Behaviour:
    /// It matches the cell type of index i to it's corresponding behavior
    pub fn update_cell_alchemy(&mut self) {
        for i in 0..(self.lenght - 1) as usize {
            match self.cells[i].cell_type {
                // change the rng range for different probabilities
                // ignore the safety checks since i is already in range
                CellType::Steam => if rand::thread_rng().gen_range(1..=1250) < 2 { self.set_cell(i, &Cell { cell_type: CellType::Water, color: CellTypeProperties::get_cell_properties(CellType::Water).base_color, generation: 0, temp: 20.0 });},
                _ => (),
            }
        }
    }
}

/// # Functionality:
/// made to reduce code duplication
/// # Structure:
/// compares the values individually and returns a boolean array based on the resulting values
pub fn compare_arrays_4(a: [bool; 4], b: [bool; 4]) -> [bool; 4] {
    [
        a[0] && b[0],
        a[1] && b[1],
        a[2] && b[2],
        a[3] && b[3]
    ]
}
