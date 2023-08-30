pub mod cells;

use crate::chunk_manager::custom_error as Err;
use cells::Cell;
use cells::CellType;

use serde_big_array::BigArray;

use std::fs;

use crate::config::CHUNK_LENGTH_USIZE;
use crate::config::CHUNK_SIZE_I32;

/// # Functionality:
/// This struct contains both the chunk coordinates and the actual chunk containing the cells. 
/// 
/// This has to use serde_big_array crate to derive the serialization and deserialization functions since they are not serializable by default (due to their size).
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Chunk {
    pub chunk_coordinates: (i32, i32),
    #[serde(with = "BigArray")]
    pub cells: [Cell; CHUNK_LENGTH_USIZE],
}

impl Default for Chunk {
    fn default() -> Self {
        Chunk { cells: [Cell::default(); CHUNK_LENGTH_USIZE], chunk_coordinates: (0,0) }
    }
}

impl From<std::io::Error> for Err::CustomErrors {
    fn from(_: std::io::Error) -> Self {
        Err::CustomErrors::CouldNotComplete
    }
}


impl Chunk {

    /// # Functionality:
    /// returns a filled ```Chunk``` with the given ```CellType``` and ```(x,y)``` chunk-coordinates.
    pub fn new_with_fill(cell_type: CellType, chunk_pos: (i32, i32)) -> Chunk {
        let cell: Cell = Cell::build_cell(cell_type);
        Chunk { chunk_coordinates: chunk_pos, cells: [cell; CHUNK_LENGTH_USIZE],}
    }

    /// # Functionality:
    /// gets the save path of the chunk with the given coordinates. This uses the ron fileformat.
    pub fn get_save_path(coords: (i32, i32)) -> String {
        coords.0.to_string() + "|" + &coords.1.to_string() + ".ron"
    }

    /// # Functionality:
    /// Writes the given chunk to a file with the appropriate file path.
    pub fn save_chunk(&self) -> Result<(), Err::CustomErrors> {

        // get the save path
        let file_path = Chunk::get_save_path(self.chunk_coordinates);

        // to ron format
        let chunk_ron: String = ron::ser::to_string_pretty(self, Default::default()).map_err(|_| Err::CustomErrors::CouldNotComplete)?;

        // dump to the file or error
        fs::write(file_path, chunk_ron).map_err(|_| Err::CustomErrors::CouldNotComplete)?;

        Ok(())
    }

    /// # Functionality:
    /// Reads a chunk form a file path and returns either a chunk or an error.
    pub fn get_from_file(file_path: &str) -> Result<Chunk, Err::CustomErrors> {

        // from file to ron format or escape
        let chunk_ron: String = fs::read_to_string(file_path).map_err(|_| Err::CustomErrors::CouldNotComplete)?;

        // get the chunk from the string or escape
        let chunk: Chunk = ron::de::from_str(&chunk_ron).map_err(|_| Err::CustomErrors::CouldNotComplete)?;

        // return
        Ok(chunk)
    }

    /// # Functionality:
    /// Load a chunk from a file or returns a default chunk.
    pub fn load_chunk(coords: (i32, i32)) -> Chunk {
        let file_path: &String = &Chunk::get_save_path(coords);
        Chunk::get_from_file(file_path).unwrap_or(Chunk::default())
    }

    /// # Functionality:
    /// checks whether the given coordinates are within the the bounds of the chunk.
    fn is_inbounds(&self, coords: (i32, i32)) -> Option<()> {

        // turns the boolean return into an optional where the SOME option will represent the true boolean
        // this will hopefully make it a hell of a lot easier to implement in other functions
        (coords.0 > 0 || coords.0 < CHUNK_SIZE_I32 || coords.1 > 0 || coords.1 < CHUNK_SIZE_I32).then(|| ())
    }

    /// # Functionality:
    /// Returns the cell index of the chunk based on the global coordinates.
    fn get_cell_coordinate(coords: (i32, i32)) -> usize {

        // not sure how this works but gippy gave me this
        let new_coords: (i32, i32) = (
            (coords.0.rem_euclid(CHUNK_SIZE_I32) + CHUNK_SIZE_I32) % CHUNK_SIZE_I32,
            (coords.1.rem_euclid(CHUNK_SIZE_I32) + CHUNK_SIZE_I32) % CHUNK_SIZE_I32,
        );
        (new_coords.0 + new_coords.1 * CHUNK_SIZE_I32) as usize
    }

    /// # Functionality:
    /// Returns an `Option<cell>` given the global coordinates. Returns `None` if the global coordinate are not inbounds.
    fn get_cell_local(&self, coords: (i32, i32)) -> Option<Cell> {

        // Bounds checking
        self.is_inbounds(coords)?;

        // retun the cell
        Some(self.cells[Chunk::get_cell_coordinate(coords)])
    }

    /// # Functionality:
    /// Replaces the cell at the given coordinate with the given cell.
    fn set_cell_local(&mut self, coords: (i32, i32), cell: Cell) -> Option<()> {

        // Bounds checking
        self.is_inbounds(coords)?;

        // changes the cell
        self.cells[Chunk::get_cell_coordinate(coords)] = cell;

        // a return value i guess?
        Some(())
    }
}


