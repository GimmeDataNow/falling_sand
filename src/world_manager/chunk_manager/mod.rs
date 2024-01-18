//module rules;
#![allow(dead_code)]

// my imports
pub mod cells;
pub mod chunks;

use crate::custom_error::ChunkError;
use crate::config::{DEFAULT_PLAYER_SPAWN_COORDINATES, CHUNK_SIZE_I32};


/// # Functionality:
/// This contain the `HashMap` containing the chunks with the `chunk-coordinates` as the key.
/// # Notice:
/// This should probably implement `Singletons` to ensure that there is only one instance of the `ChunkManager`.
pub struct ChunkManager {
    pub generation: u16,
    pub map: fnv::FnvHashMap<(i32, i32), chunks::Chunk>,
}

impl ChunkManager {
    pub fn new() -> Self {

        // probably a better way to do this but I don't know how
        // I really dislike the implicit type declaration that is made by default_coords and val
        let mut map = fnv::FnvHashMap::default();

        // player/camera spawns here
        let default_coords: (i32, i32) = DEFAULT_PLAYER_SPAWN_COORDINATES;

        // here the loading of the default chunk is done
        let val: chunks::Chunk = chunks::Chunk::default();

        // dump the loaded chunk into the map as a default
        map.insert(default_coords, val);

        // return
        ChunkManager { map, generation: 0 }
    }

    /// # Functionality:
    /// Convert the coordinates to `chunk-coordinates` for further processing.
    /// //THIS IS THE BUG
    fn to_chunk_coords(global_coords: (i32, i32)) -> (i32, i32) {
        (
            if global_coords.0 >= 0 { global_coords.0 / CHUNK_SIZE_I32 + 1 } else { global_coords.0 / CHUNK_SIZE_I32 },
            if global_coords.1 >= 0 { global_coords.1 / CHUNK_SIZE_I32 + 1 } else { global_coords.1 / CHUNK_SIZE_I32 }
        )
    }

    /// # Functionality:
    /// Convert the coordinates to local in-chunk coordinates for further processing.
    fn to_local(global_coords: (i32, i32)) -> (i32, i32) {
        (global_coords.0.rem_euclid(CHUNK_SIZE_I32), global_coords.1.rem_euclid(CHUNK_SIZE_I32))
    }

    /// # Functionality:
    /// Convert the coordinates to the index for further processing.
    fn to_index(local_coords: (i32, i32)) -> usize {
        (local_coords.0 + local_coords.1 * CHUNK_SIZE_I32) as usize
    }

    /// # Functionality:
    /// Convert the coordinates to the index for further processing.
    pub fn set_chunk(&mut self, global_coords: (i32, i32), cell_type: cells::CellType) -> Option<()> {
        let chunk_coords: (i32, i32) = ChunkManager::to_chunk_coords(global_coords);
        
        if let Some(chunk) = self.map.get_mut(&chunk_coords) {
            chunk.cells = chunks::Chunk::new_with_fill(cell_type, chunk_coords).cells;
            return Some(());
        }
        None
    }

    pub fn save_chunk(&self, coords: &(i32, i32)) -> Result<(), ChunkError> {
        Ok(
            self.map
                .get(coords)
                .ok_or(ChunkError::OutOfBounds)?
                .save_chunk()?
        )
    }

    pub fn unload_chunk_at_coords(&mut self, chunk_coords: &(i32, i32)) -> Result<(), ChunkError> {

        // check if the chunk is loaded
        if self.map.contains_key(&chunk_coords) {

            // Save the chunk to disk before unloading if needed
            self.map.get(&chunk_coords).ok_or(ChunkError::TargtNotLoaded)?.save_chunk()?;

            // Remove the chunk from the hashmap to unload it
            self.map.remove(&chunk_coords);
            Ok(())
        } else {
            // error if the chunk is not loaded
            Err(ChunkError::FailedToUnload)
        }
    }

}