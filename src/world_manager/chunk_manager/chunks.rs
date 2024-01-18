//module rules;
#![allow(dead_code)]

// my imports
use super::cells::{CellType, Cell};
use crate::custom_error::CellError;
use crate::config::CHUNK_LENGTH_USIZE;

// foreign imports
use serde::{Serialize, Deserialize};
use serde_big_array::BigArray;
use std::fs;

/// # Functionality:
/// This struct contains both the chunk coordinates and the actual cells. 
/// 
/// This has to use `serde_big_array` crate to derive the serialization and deserialization functions since they are not serializable by default (due to their size).
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


impl Chunk {

    /// # Functionality:
    /// returns a filled `Chunk` with the given `CellType` and `(x,y)` chunk-coordinates.
    pub fn new_with_fill(cell_type: CellType, chunk_pos: (i32, i32)) -> Chunk {
        let cell: Cell = Cell::build_cell(cell_type);
        Chunk { chunk_coordinates: chunk_pos, cells: [cell; CHUNK_LENGTH_USIZE],}
    }

    /// # Functionality:
    /// gets the save `path` of the `Chunk` with the given coordinates. This uses the `.ron` fileformat.
    pub fn get_save_path(coords: &(i32, i32)) -> String {
        "chunks/".to_owned() + &coords.0.to_string() + "_" + &coords.1.to_string() + ".ron"
    }

    /// # Functionality:
    /// Writes the given `Chunk` to a file with the appropriate file `path`.
    pub fn save_chunk(&self) -> Result<(), CellError> {

        // get the save path
        let file_path: String = Chunk::get_save_path(&self.chunk_coordinates);

        // to ron format
        let chunk_ron: String = ron::ser::to_string_pretty(self, Default::default()).map_err(|_| CellError::CouldNotComplete)?;

        // dump to the file or error
        fs::write(file_path, chunk_ron).map_err(|_| CellError::CouldNotComplete)?;

        Ok(())
    }

    /// # Functionality:
    /// Reads a `Chunk` form a file `path` and returns either a `Chunk` or a `CustomErrors`.
    pub fn get_from_file(file_path: &str) -> Result<Chunk, CellError> {

        // from file to ron format or escape
        let chunk_ron: String = fs::read_to_string(file_path).map_err(|_| CellError::CouldNotComplete)?;

        // get the chunk from the string or escape
        let chunk: Chunk = ron::de::from_str(&chunk_ron).map_err(|_| CellError::CouldNotComplete)?;

        // return
        Ok(chunk)
    }

    /// # Functionality:
    /// Load a `Chunk` from a file or returns a default `Chunk`.
    pub fn load_chunk(coords: (i32, i32)) -> Option<Chunk> {
        // get the save path
        let file_path: &String = &Chunk::get_save_path(&coords);

        Chunk::get_from_file(file_path).ok()
    }
}