//module rules;
#![allow(dead_code)]

// my imports
use super::cells::{CellType, Cell};
use crate::world_manager::coordinates::*;
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
    #[serde(with = "BigArray")]
    pub cells: [Cell; CHUNK_LENGTH_USIZE],
}

impl Default for Chunk {
    fn default() -> Self {
        Chunk { cells: [Cell::default(); CHUNK_LENGTH_USIZE] }
    }
}


impl Chunk {

    /// # Functionality:
    /// Returns a filled `Chunk` with the given `CellType` and `ChunkCoords`.
    pub fn new_from_cell_type(cell_type: CellType) -> Chunk {
        let cell: Cell = Cell::build_cell(cell_type);
        Chunk { cells: [cell; CHUNK_LENGTH_USIZE],}
    }

    /// # Functionality:
    /// Returns a filled `Chunk` with the given `Cell` and `ChunkCoords`.
    pub fn new_from_cell(cell: Cell) -> Chunk {
        Chunk { cells: [cell; CHUNK_LENGTH_USIZE],}
    }

    /// # Functionality:
    /// Gets the save `path` of the `Chunk` with the given coordinates. This uses the `.ron` fileformat.
    pub fn get_save_path(coords: &ChunkCoords) -> String {
        format!("chunks/{}_{}.ron",&coords.x, &coords.y )
    }

    /// # Functionality:
    /// Writes the given `Chunk` to a file with the appropriate file `path`.
    pub fn save_chunk(&self, chunk_coords: &ChunkCoords) -> Result<(), CellError> {

        // get the save path
        let file_path: String = Chunk::get_save_path(chunk_coords);

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
    pub fn load_chunk(coords: &ChunkCoords) -> Option<Chunk> {
        // get the save path
        let file_path: &String = &Chunk::get_save_path(coords);

        Chunk::get_from_file(file_path).ok()
    }
}