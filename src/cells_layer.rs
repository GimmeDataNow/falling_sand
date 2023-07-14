// imports
use crate::config::CHUNK_SIZE;

use rand::Rng;

use fnv::FnvHashMap;

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
/// The materials are: ```Air```, ```Rock```, ```Wood```, ```Sand```, ```Gunpowder```, ```Water```, ```Oil```, ```Fire```, ```Lava```, ```Acid``` ....
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
    AntiVoid
}
impl Default for CellType {
    fn default() -> Self {
        CellType::AntiVoid
    }
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
/// This enum dictates what shape the ```paint_brush()``` function should assume
/// # Options:
/// The options of ```BrushType``` are: ```Square```, ```Circle```
#[allow(dead_code)]
pub enum BrushType {
    Square,
    Circle
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
static CELL_PROPERTIES: [CellTypeProperties; 12] = [    
    CellTypeProperties { name: "Air",       cell_type: CellType::Air,       state: StateOfAggregation::Replaceable,     density: 0.0,   temp_coefficient: 1.0,      flammable: false, base_temp: 298,   base_color: [0,   0,    0, 0] },
    CellTypeProperties { name: "Rock",      cell_type: CellType::Rock,      state: StateOfAggregation::ImmovableSolid,  density: 9.0,   temp_coefficient: 0.1,      flammable: false, base_temp: 298,   base_color: [119, 136,  153, 255] },
    CellTypeProperties { name: "Water",     cell_type: CellType::Water,     state: StateOfAggregation::Liquid,          density: 1.0,   temp_coefficient: 0.1,      flammable: false, base_temp: 298,   base_color: [0, 0,  255, 255] },
    CellTypeProperties { name: "Sand",      cell_type: CellType::Sand,      state: StateOfAggregation::Granular,        density: 1.5,   temp_coefficient: 0.1,      flammable: false, base_temp: 298,   base_color: [250, 250,  210, 255] },
    CellTypeProperties { name: "Gravel",    cell_type: CellType::Gravel,    state: StateOfAggregation::Granular,        density: 3.1,   temp_coefficient: 0.1,      flammable: false, base_temp: 298,   base_color: [112, 128,  144, 255] },
    CellTypeProperties { name: "Wood",      cell_type: CellType::Wood,      state: StateOfAggregation::ImmovableSolid,  density: 1.2,   temp_coefficient: 0.1,      flammable: true,  base_temp: 298,   base_color: [139, 69,   19, 255] },
    CellTypeProperties { name: "Steam",     cell_type: CellType::Steam,     state: StateOfAggregation::Gas,             density: 0.1,   temp_coefficient: 0.1,      flammable: false, base_temp: 298,   base_color: [206, 206,  209, 255] },
    CellTypeProperties { name: "Gunpowder", cell_type: CellType::Gunpowder, state: StateOfAggregation::Granular,        density: 1.7,   temp_coefficient: 0.1,      flammable: true,  base_temp: 298,   base_color: [70,  70,   80,  255] },
    CellTypeProperties { name: "Oil",       cell_type: CellType::Oil,       state: StateOfAggregation::Liquid,          density: 0.9,   temp_coefficient: 0.1,      flammable: true,  base_temp: 298,   base_color: [55,  58,   54,  255] },
    CellTypeProperties { name: "Lava",      cell_type: CellType::Lava,      state: StateOfAggregation::Liquid,          density: 3.1,   temp_coefficient: 100.0,    flammable: false, base_temp: 298,   base_color: [255, 0,    0,   255] },
    CellTypeProperties { name: "Acid",      cell_type: CellType::Acid,      state: StateOfAggregation::Liquid,          density: 1.4,   temp_coefficient: 0.1,      flammable: false, base_temp: 298,   base_color: [0,   255,  0,   255] },
    CellTypeProperties { name: "Anti-Void", cell_type: CellType::AntiVoid,  state: StateOfAggregation::ImmovableSolid,  density: 9.9,   temp_coefficient: 0.1,      flammable: false, base_temp: 298,   base_color: [255, 255,  255, 255] }
];


impl CellTypeProperties {

    /// # Functionality:
    /// This function returns a random element of the ```CELL_PROPERTIES``` array. It is not dependant on the size of ```CELL_PROPERTIES```.
    /// # Panic behaviour:
    /// Panics if the length of ```CELL_PROPERTIES < 1```.
    #[allow(dead_code)]
    pub fn rand_cell_properties() -> CellTypeProperties{ CELL_PROPERTIES[rand::thread_rng().gen_range(0..CELL_PROPERTIES.len())] }

    /// # Functionality:
    /// This is the highly imortant function that returns the ```&CellTypeProperties``` that other functions rely on.
    /// # Panic behaviour:
    /// Panics if the length of ```CELL_PROPERTIES < 1```
    pub fn get_cell_properties<'a>(cell_type: CellType) -> &'a CellTypeProperties { &CELL_PROPERTIES[cell_type as usize] }

    /// # Functionality:
    /// This function returns the ```CellType``` of the cell based on the ```selection``` counter.
    pub fn get_celltype_by_number(selection: &usize) -> CellType { CELL_PROPERTIES[selection % CELL_PROPERTIES.len()].cell_type }

    /// # Functionality:
    /// This function returns the ```CellTypeProperties``` of the cell based on the ```selection``` counter.
    pub fn get_cell_properties_by_number(selection: &usize) -> CellTypeProperties { *Self::get_cell_properties(Self::get_celltype_by_number(selection)) }
}

/// # Functionality:
/// This general cell struct that stores cell specific data, that can vary from cell to cell.
/// # Structure:
/// ```
/// pub struct Cell {
///     pub cell_type: CellType,
///     pub color: [u8; 4],
///     pub generation: u32,
///     pub temp: f32,
///}
/// ```
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Cell {
    pub cell_type: CellType,
    pub color: [u8; 4],
    pub generation: u32,
    pub temp: u16,
}
impl Default for Cell {
    fn default() -> Self {
        let c = CellTypeProperties::get_cell_properties(CellType::default());
        Cell { cell_type: c.cell_type, color: c.base_color, generation: 0, temp: 295 }
    }
}

impl Cell {

    /// # Functionality:
    /// returns the ```CellTypeProperties``` struct with respect to the ```CellType```
    pub fn get_cell_properties<'a>(&self) -> &'a CellTypeProperties { CellTypeProperties::get_cell_properties(self.cell_type) }

    /// # Functionality:
    /// returns the ```Cell``` based on the given ```CellType```
    pub fn build_cell(cell_type: CellType) -> Cell {

        //this is the cell properties that will be used to build the cell
        let ref_cell_properties = CellTypeProperties::get_cell_properties(cell_type);
        
        //this is the cell that will be returned
        Cell { 
            cell_type: ref_cell_properties.cell_type,
            generation: 0, 
            color: if ref_cell_properties.state != StateOfAggregation::Liquid && ref_cell_properties.state != StateOfAggregation::Gas { randomise_color(ref_cell_properties.base_color, 0.1)} else { ref_cell_properties.base_color }, 
            temp: ref_cell_properties.base_temp,  
        }
    }

    /// # Functionality:
    /// sets the cell to be air.
    pub fn set_air() -> Cell { Self::build_cell(CellType::Air) }
}

pub fn randomise_color(color: [u8; 4], range: f32) -> [u8; 4] {
    if range < 0.0 || range > 1.0 { return color; }
    let brightness_adjust: f32 = rand::thread_rng().gen_range(0.0..range);
    [
        (color[0] as f32 * (1.0 - brightness_adjust)).trunc() as u8,
        (color[1] as f32 * (1.0 - brightness_adjust)).trunc() as u8,
        (color[2] as f32 * (1.0 - brightness_adjust)).trunc() as u8,
        (color[3] as f32 * (1.0 - brightness_adjust)).trunc() as u8
    ]
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Chunk {
    pub chunk_coordinates: (i32, i32),
    pub cells: [[Cell; CHUNK_SIZE]; CHUNK_SIZE],
}

impl Default for Chunk {
    fn default() -> Self {
        Chunk { cells: [[Cell::default(); CHUNK_SIZE]; CHUNK_SIZE], chunk_coordinates: (0,0) }
    }
}

impl From<std::io::Error> for CustomErrors {
    fn from(_: std::io::Error) -> Self {
        CustomErrors::CouldNotComplete
    }
}

impl Chunk {

    /// # Functionality:
    /// returns a filled ```Chunk``` with the given ```CellType``` and ```(x,y)``` chunk-coordinates.
    pub fn new_with_fill(cell_type: CellType, chunk_coords: (i32, i32)) -> Chunk {
        let cell = Cell::build_cell(cell_type);
        Chunk { cells: [[cell; CHUNK_SIZE]; CHUNK_SIZE], chunk_coordinates: chunk_coords}
    }

    /// # Functionality:
    /// gets the save/load file path for the given `(x,y)` chunk-coordinates.
    fn get_chunk_path(coords: (i32, i32)) -> String {
        todo!()
    }

    /// # Functionality:
    /// returns the chunk with the given `(x,y)` chunk-coordinates or returns an Error.
    fn load_chunk_from_file(coords: (i32, i32)) -> Result<Chunk, CustomErrors> {
        let path = Chunk::get_chunk_path(coords);
        todo!()
    }

    /// # Functionality:
    /// returns the chunk with the given `(x,y)` chunk-coordinates or returns a default Chunk.
    /// This is simply a wrapper around `load_chunk_from_file()`.
    fn get_chunk_from_file(coords: (i32, i32)) -> Chunk {
        match Chunk::load_chunk_from_file(coords) {
            Ok(c) => c,
            Err(_) => Chunk::new_with_fill(CellType::default(), (0,0))
        }
    }

    /// # Functionality:
    /// `TODO`
    fn save_chunk_to_file(&self) -> Result<(), CustomErrors> {
        todo!()
    }

    /// # Functionality:
    /// `TODO`
    fn unload_chunk(&mut self) -> Result<(), CustomErrors> {
        todo!()
    }

    // should implement some error handling but this works for now
    fn get_cell_local_coords(&self, coords: (i32, i32)) -> Cell {

        // to usize coordinates
        let pos = (coords.0 as usize, coords.1 as usize);

        // retun the cell
        self.cells[pos.1][pos.0]
    }

    fn set_cell_local_coords(&self, coords: (i32, i32), cell: Cell) -> Option<()> {

        // assumes that the index is within the bounds of the chunk
        let pos = (coords.0 as usize, coords.1 as usize);

        // changes the cell
        self.cells[pos.1][pos.0] = cell;

        // a return value i guess?
        Some(())
    }

    
}

// should prob only have one of these active at any given time, but I don't quite understand Singletons yet
pub struct ChunkManager {
    pub is_empty: bool,
    pub map: fnv::FnvHashMap<(i32, i32), Chunk>
}

impl ChunkManager {
    fn new() -> Self {

        // probably a better way to do this but I don't know how
        // I really dislike the implicit type declaration that is made by default_coords and val
        let mut map = fnv::FnvHashMap::default();

        // player spawns here
        let default_coords = (0, 0);

        // here the loading of the default chunk is done
        let val = Chunk::get_chunk_from_file(default_coords);

        // dump the loaded chunk into the map as a default
        map.insert(default_coords, val);

        // return
        ChunkManager { is_empty: false, map: map }
    }

    fn global_pos_to_chunk_pos(&self, coords: (i32, i32)) -> (i32, i32) {

        // this should return the position as (i32, i32), but no guarantee with this
        (coords.0 / CHUNK_SIZE as i32 , coords.1 / CHUNK_SIZE as i32)
    }

    fn global_pos_decompose(&self, coords: (i32, i32)) -> ((i32, i32), (i32, i32)) {

        // this should return the position as (i32, i32), but no guarantee with this
        // this should also return the remainder as (i32, i32), but no guarantee with this
        ( (coords.0 / CHUNK_SIZE as i32 , coords.1 / CHUNK_SIZE as i32), (coords.0 % CHUNK_SIZE as i32 , coords.1 % CHUNK_SIZE as i32) )
    }

    fn get_cell(&self, coords: (i32, i32)) -> Option<Cell> {

        // split the coordinates into chunk and local coordinates
        let k = self.global_pos_decompose(coords);

        // use the chunk coordinates to get the chunk
        let chunk = self.map.get(&k.0)?;

        // index into the chunk and retrieve the cell
        Some(chunk.get_cell_local_coords(k.1))
    }

    fn set_cell(&mut self, coords: (i32, i32), cell: Cell) -> Option<()> {

        // decompose the coordinates
        let k = self.global_pos_decompose(coords);

        // pray this is actually a mutable reference
        let chunk = self.map.get_mut(&k.0)?;

        // replace the cell at the given coordinates with the given cell    
        // assumes chunk is acutally mutable (not sure if this is the case though)
        chunk.set_cell_local_coords(coords, cell);

        // a return value i guess?
        Some(())
    }

    // maybe invert the return value? this would be nice cause i can just use the ? operator to exit the other functions if necessary
    fn simulate_vertical(&mut self, coords: (i32, i32)) -> Option<()> {
        None
    }

    //
    fn simulate_granular(&mut self) -> Option<()> {
        Some(())
    }

    fn match_behaviour(&mut self, cell: Cell, coords: (i32, i32)) {
        match cell.get_cell_properties().state {
            StateOfAggregation::Granular => (),
            _ => (),
        }
    }

    pub fn simulate_area(&mut self, p1: (i32, i32), p2: (i32, i32)) -> Result<(), CustomErrors> {

        // get the lower left and upper right coordinates
        let upperleft = (p1.0.min(p2.0), p1.1.min(p2.1));
        let lowerright = (p1.0.max(p2.0), p1.1.max(p2.1));

        // iterate over the coordinates
        for x in upperleft.0..lowerright.0 {
            for y in upperleft.1..lowerright.1 {

                match self.get_cell((x,y)) {
                    // add the sim cell funtion here
                    Some(cell) => (),
                    None => (),
                }
            }
        }
        todo!()
    }

}