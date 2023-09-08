//module rules;
#![allow(dead_code)]
// imports
use rand::Rng;
use serde::{Serialize, Deserialize};


/// # Functionality:
/// The `CellType` is the material of a cell.
/// # Options:
/// The materials are: ```Air```, ```Rock```, etc.
#[derive(Default, Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum CellType {
    #[default]
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

/// # Functionality:
/// This enum dictates how the material is processed in the function that is responsible for updating the world.
/// # Options:
/// The states of aggregation are: ```ImmovableSolid```, ```Granular```, ```Liquid```, ```Gas```, ```Replaceable```.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum StateOfAggregation {
    ImmovableSolid,
    Granular,
    Liquid,
    Gas,
    Replaceable
}

/// # Functionality:
/// This enum dictates what shape the `paint_brush()` function should assume.
/// # Options:
/// The options of ```BrushType``` are: ```Square``` and ```Circle```.
#[allow(dead_code)]
pub enum BrushType {
    Square,
    Circle
}

/// # Functionality:
/// This struct dictates the structure and information of the look-up-array `CELL_PROPERTIES`, which inturn dictates material behavior. `base_temp` is measured in `Kelvin`.
/// # Structure:
/// ```
/// pub struct CellTypeProperties {
///     pub name: &'static str,
///     pub cell_type: CellType,
///     pub state: StateOfAggregation,
///     pub density: f32,
///     pub temp_coefficient: f32,
///     pub flammable: bool,
///     pub base_temp: u16,
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
/// This is the look-up-array for other functions to rely on.
/// # Structure:
/// It's an static array of `CellTypeProperties` with fixed length.
pub static CELL_PROPERTIES: [CellTypeProperties; 12] = [    
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
    CellTypeProperties { name: "Anti-Void", cell_type: CellType::AntiVoid,  state: StateOfAggregation::ImmovableSolid,  density: 9.9,   temp_coefficient: 0.1,      flammable: false, base_temp: 298,   base_color: [0, 0,  0, 255] }
];


impl CellTypeProperties {

    /// # Functionality:
    /// This function returns a random element of the `CELL_PROPERTIES` array. It is not dependant on the size of `CELL_PROPERTIES`.
    pub fn rand_cell_properties() -> CellTypeProperties{ CELL_PROPERTIES[rand::thread_rng().gen_range(0..CELL_PROPERTIES.len())] }

    /// # Functionality:
    /// This is the highly imortant function that returns the `&CellTypeProperties` that other functions rely on.
    pub fn get_cell_properties<'a>(cell_type: CellType) -> &'a CellTypeProperties { &CELL_PROPERTIES[cell_type as usize] }

    /// # Functionality:
    /// This function that returns the `&CellTypeProperties` of the corresponding index. This effectively does the same as `get_cell_properties()`, but rust restricts the types that a function can accept.
    pub fn get_cell_properties_by_index<'a>(index: usize) -> &'a CellTypeProperties { &CELL_PROPERTIES[index] }

}

/// # Functionality:
/// This general cell struct that stores unique cell specific data.
/// # Structure:
/// ```
/// pub struct Cell {
///     pub cell_type: CellType,
///     pub color: [u8; 4],
///     pub generation: u32,
///     pub temp: f32,
///}
/// ```
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Cell {
    pub cell_type: CellType,
    pub color: [u8; 4],
    pub generation: u32,
    pub temp: u16,
}

impl Default for Cell {
    fn default() -> Self {
        let c: &CellTypeProperties = CellTypeProperties::get_cell_properties(CellType::default());
        Cell { cell_type: c.cell_type, color: c.base_color, generation: 0, temp: 295 }
    }
}

impl Cell {

    /// # Functionality:
    /// returns the `CellTypeProperties` struct with respect to the `CellType`.
    pub fn get_cell_properties<'a>(&self) -> &'a CellTypeProperties { CellTypeProperties::get_cell_properties(self.cell_type) }

    /// # Functionality:
    /// returns the `Cell` based on the given `CellType`.
    pub fn build_cell(cell_type: CellType) -> Cell {

        //this is the cell properties that will be used to build the cell
        let ref_cell_properties: &CellTypeProperties = CellTypeProperties::get_cell_properties(cell_type);
        
        //this is the cell that will be returned
        Cell { 
            cell_type: ref_cell_properties.cell_type,
            generation: 0, 
            color: if ref_cell_properties.state != StateOfAggregation::Liquid && ref_cell_properties.state != StateOfAggregation::Gas { randomise_color(ref_cell_properties.base_color, 0.1)} else { ref_cell_properties.base_color }, 
            temp: ref_cell_properties.base_temp,  
        }
    }
}


/// # Functionality:
/// This function is supposed to randomise the cells colors a little bit to add some extra texturing.
/// # Behavior:
/// Will return the base color of the cell if the range is not within the area of [0.0 -  1.0].
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

