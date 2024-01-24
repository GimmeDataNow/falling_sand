//module rules;
#![allow(dead_code)]

// my imports
pub mod cells;
pub mod chunks;

use super::coordinates::*;
use cells::{StateOfAggregation, CellTypeProperties, Cell};
use super::chunk_manager::chunks::Chunk;
use crate::custom_error::ChunkError;
use crate::config::{DEFAULT_PLAYER_SPAWN_COORDINATES, SCREEN_HEIGHT, SCREEN_WIDTH};


use rand::Rng;


/// # Functionality:
/// This contain the `HashMap` containing the chunks with the `chunk-coordinates` as the key.
/// # Notice:
/// This should probably implement `Singletons` to ensure that there is only one instance of the `ChunkManager`.
pub struct ChunkManager {
    pub generation: u16,
    pub map: fnv::FnvHashMap<ChunkCoords, chunks::Chunk>,
}

impl ChunkManager {
    pub fn new() -> Self {

        // probably a better way to do this but I don't know how
        // I really dislike the implicit type declaration that is made by default_coords and val
        let mut map = fnv::FnvHashMap::default();

        // player/camera spawns here
        let default_coords: ChunkCoords = ChunkCoords::from( GlobalCoords::from(DEFAULT_PLAYER_SPAWN_COORDINATES) );

        // here the loading of the default chunk is done
        let val: chunks::Chunk = chunks::Chunk::default();

        // dump the loaded chunk into the map as a default
        map.insert(default_coords, val);

        // return
        ChunkManager { map, generation: 0 }
    }

    /// # Functionality:
    /// Return the chunk in the hash map
    pub fn get_chunk(&mut self, chunk_coords: &ChunkCoords) -> Option<&mut Chunk> {
        self.map.get_mut(chunk_coords)
    }

    /// # Functionality:
    /// Convert the coordinates to the index for further processing.
    pub fn set_chunk(&mut self, global_coords: &GlobalCoords, cell_type: cells::CellType) -> Option<()> {
        //let chunk_coords: (i32, i32) = ChunkManager::to_chunk_coords(global_coords);
        let chunk_coords: ChunkCoords = ChunkCoords::from(*global_coords);
        if let Some(chunk) = self.map.get_mut(&chunk_coords) {
            chunk.cells = chunks::Chunk::new_with_fill(cell_type, chunk_coords).cells;
            return Some(());
        }
        None
    }

    /// # Functionality:
    /// Save the chunk to a file
    pub fn save_chunk(&self, coords: &ChunkCoords) -> Result<(), ChunkError> {
        Ok(
            self.map
                .get(coords)
                .ok_or(ChunkError::OutOfBounds)?
                .save_chunk()?
        )
    }

    /// # Functionality:
    /// Save the chunk to a file and then unload it from memory
    pub fn unload_chunk_at_coords(&mut self, chunk_coords: &ChunkCoords) -> Result<(), ChunkError> {

        // check if the chunk is loaded
        if self.map.contains_key(chunk_coords) {

            // Save the chunk to disk before unloading if needed
            self.map.get(chunk_coords).ok_or(ChunkError::TargtNotLoaded)?.save_chunk()?;

            // Remove the chunk from the hashmap to unload it
            self.map.remove(chunk_coords);
            Ok(())
        } else {

            // error if the chunk is not loaded
            Err(ChunkError::FailedToUnload)
        }
    }

    /// # Functionality:
    /// Loads a chunk into memory / into the hashmap
    pub fn load_chunk(&mut self, chunk_coords: ChunkCoords) -> Option<Chunk> {
        self.map.insert(chunk_coords, chunks::Chunk::load_chunk(&chunk_coords)?)
    }

    /// # Functionality:
    /// Check if the `Chunk` is loaded in the `chunk-map`. If not, it inserts the loaded chunk into the `chunk-map` and returns it.
    fn get_and_force_load_chunk(&mut self, chunk_coords: &ChunkCoords) -> Option<&mut Chunk> {
        if !self.map.contains_key(chunk_coords) {
            self.map.insert(*chunk_coords, Chunk::default());
        }
        return self.map.get_mut(chunk_coords);
    }

}

/// # Functionality:
/// This is the Chunk cache. It is meant to be accessed before the actual hashmap to reduce hashmap lookups and improve performance.
/// # Notice:
/// This should probably implement `Singletons` to ensure that there is only one instance of the `ChunkManager`.
/// I should also eventually use a tuple: (Option<Chunk>, Option<Chunk>). This should improve the swap_cells() function a lot.
/// # TODO: USE TUPLE
pub struct ChunkCache {
    pub chunk_coords: ChunkCoords,
    pub chunk: Option<Chunk>,
    pub chunk_coords_tuple: (ChunkCoords, ChunkCoords),
    pub chunk_tuple: (Option<Chunk>, Option<Chunk>)
}

impl ChunkCache {

    /// # Functionality:
    /// creates the buffer
    pub fn new() -> Self {
        ChunkCache { 
            // used DEFAULT_PLAYER_SPAWN_COORDINATES
            // center chunk may not spawn correctly 
            // # TODO: FIX THIS
            chunk_coords: ChunkCoords::from(DEFAULT_PLAYER_SPAWN_COORDINATES), 
            chunk: None,
            chunk_coords_tuple: (ChunkCoords::from(DEFAULT_PLAYER_SPAWN_COORDINATES), ChunkCoords::from(DEFAULT_PLAYER_SPAWN_COORDINATES)),
            chunk_tuple: (None, None),
        }
    }

    /// # Functionality:
    /// Check if the ChunkCoords are already inside the chunk_coords_tuple
    fn is_loaded(&mut self,  chunk_coords: &ChunkCoords) -> bool {

        // check if the ChunkCoords are already inside the chunk_coords_tuple
        &self.chunk_coords_tuple.0 == chunk_coords || &self.chunk_coords_tuple.1 == chunk_coords
    }

    /// # Functionality:
    /// loads the chunk into the buffer 
    /// 
    /// the bool represents the tuple index, that means that the return type indicates the position of the Chunk that was loaded into the tuple
    pub fn load_chunk(&mut self, world_map: &mut ChunkManager, chunk_coords: &ChunkCoords, preffered_chunk: bool) -> Option<bool> {

        // retun early if the coordinates are already loaded
        // check if the target is loaded into the first tuple slot
        if &self.chunk_coords_tuple.0 == chunk_coords {

            // return true if it is
            return Some(true);

        // check if the target is loaded into the second tuple slot
        } else if &self.chunk_coords_tuple.1 == chunk_coords {

            // return false if it is
            return Some(false);
        }

        // match the tuple index
        match preffered_chunk {

            true => {

                // save the current Chunk into the hashmap
                world_map.map.insert(self.chunk_coords_tuple.0, self.chunk_tuple.0?);

                // load the wanted chunk into the preffered tuple index
                self.chunk_tuple.0 = world_map.map.get_mut(&self.chunk_coords_tuple.0).cloned();

                // update the coordinates
                self.chunk_coords_tuple.0 = *chunk_coords;

                // return the tuple index
                return Some(preffered_chunk);
            },

            false => {

                // save the current Chunk into the hashmap
                world_map.map.insert(self.chunk_coords_tuple.1, self.chunk_tuple.1?);

                // load the wanted chunk into the preffered tuple index
                self.chunk_tuple.1 = world_map.map.get_mut(&self.chunk_coords_tuple.1).cloned();

                // update the coordinates
                self.chunk_coords_tuple.1 = *chunk_coords;

                // return the tuple index
                return Some(preffered_chunk);
            },
        }
        // code cannot reach this part, so no need for a return
    }

    /// # Functionality:
    /// Gets the Cell inside the buffer.
    /// # Warning: 
    /// It is the responsibility of the caller to ensure that the correct Chunk is loaded!
    fn get_cell(&self, coords: &GlobalCoords) -> Option<(Cell, bool)> {

        // check if the chunk is in the first slot
        let preffered_chunk = &self.chunk_coords_tuple.0 == &ChunkCoords::from(coords);

        // match to the correct chunk tuple index
        match preffered_chunk {

            // clone the cell and attach the tuple index then return
            true => Some((self.chunk_tuple.0?.cells[Index::from(*coords).i], preffered_chunk)),

            // clone the cell and attach the tuple index then return
            false => Some((self.chunk_tuple.1?.cells[Index::from(*coords).i], preffered_chunk)),
        }
    }

    /// # Functionality:
    /// Sets the Cell inside the buffer.
    /// # Warning: 
    /// It is the responsibility of the caller to ensure that the correct Chunk is loaded!
    pub fn set_cell(&mut self, coords: &GlobalCoords, cell: Cell, preffered_chunk: bool) -> Option<()> {

        // match to the correct chunk tuple index
        match preffered_chunk {
            true => {

                // set the cell
                self.chunk_tuple.0?.cells[Index::from(*coords).i] = cell;

                Some(())
            },
            false => {

                // set the cell
                self.chunk_tuple.1?.cells[Index::from(*coords).i] = cell;
                Some(())
            },
        }

        // code cannot reach this part, so no need for a return
    }

    /// # Functionality:
    /// Sets the Cell inside the buffer.
    pub fn set_cell_force_load(&mut self, world_map: &mut ChunkManager, coords: &GlobalCoords, cell: Cell, preffered_chunk: bool) -> Option<()> {

        self.load_chunk(world_map, &ChunkCoords::from(coords), preffered_chunk);

        // match to the correct chunk tuple index
        match preffered_chunk {
            true => {

                // set the cell
                self.chunk_tuple.0?.cells[Index::from(*coords).i] = cell;

                Some(())
            },
            false => {

                // set the cell
                self.chunk_tuple.1?.cells[Index::from(*coords).i] = cell;
                Some(())
            },
        }

        // code cannot reach this part, so no need for a return
    }

    /// # Functionality:
    /// sets the buffer and sets the Cell inside the ChunkManager
    /// 
    /// loads the chunks if necessary
    fn swap_cells(&mut self, world_map: &mut ChunkManager, coords_1: &GlobalCoords, coords_2: &GlobalCoords) -> Option<()> {

        // get the chunk coordinates
        let cell_1_chunk_coords = ChunkCoords::from(coords_1);
        let cell_2_chunk_coords = ChunkCoords::from(coords_2);

        // check if both cells are in the same chunk
        match cell_1_chunk_coords == cell_2_chunk_coords {

            // if it is
            true => {

                // check if the chunk is actually loaded and if not load it
                // this deliberately sets the tuple index to be one, because both cells are in the same chunk and more likely to be accessed again
                if !self.is_loaded(&cell_1_chunk_coords) { self.load_chunk(world_map, &cell_1_chunk_coords, true);}
            },

            // if it's not
            false => {

                // check if the chunks are actually loaded and if not load them
                // does not have any preferance to either index because it's impossible to determine here
                if !self.is_loaded(&cell_1_chunk_coords) { self.load_chunk(world_map, &cell_1_chunk_coords, true); }
                if !self.is_loaded(&cell_2_chunk_coords) { self.load_chunk(world_map, &cell_2_chunk_coords, false); }
            },
        }
        // get and save the cells
        let cell1_tuple = self.get_cell(coords_1)?;
        let cell2_tuple = self.get_cell(coords_2)?;

        // set the cells
        self.set_cell(coords_2, cell1_tuple.0, true);

        // cell_1_chunk_coords == cell_2_chunk_coords is false when the cells are in different chunks and thus the cell will be redirected to the second index
        self.set_cell(coords_1, cell2_tuple.0, cell_1_chunk_coords == cell_2_chunk_coords);

        // the return
        Some(())
    }

    /// # Functionality:
    /// tries to simulate gravity for the given Cell. Returns `None` if it fails and `Some if it succeeds`
    /// 
    /// loads the chunks if necessary
    fn simulate_gravity(&mut self, world_map: &mut ChunkManager, coords_1: &GlobalCoords, cell1_properties: CellTypeProperties) -> Option<()> {

        // get the coordiantes
        let coords_2: &GlobalCoords = &(*coords_1 + (0, 1));

        let cell2 =  self.get_cell(coords_2)?;

        let cell2_properties: CellTypeProperties = Into::<CellTypeProperties>::into(cell2.0);

        match (cell1_properties.state, cell2_properties.state) {

            // eliminate the ImmovableSolids immediately
            (StateOfAggregation::ImmovableSolid, _) => None,
            (_, StateOfAggregation::ImmovableSolid) => None,

            // if the target is of a granular type then skip it
            (_, StateOfAggregation::Granular) => None,

            // no more checks are needed because the first match (the one above) prevents this
            (StateOfAggregation::Granular, _) => {

                // swap
                self.swap_cells(world_map, coords_1, coords_2)?;

                Some(())
            },

            // liquids
            (StateOfAggregation::Liquid, _) => {

                // density check
                if cell1_properties.density > cell2_properties.density {  self.swap_cells(world_map, coords_1, coords_2)? };

                Some(())
            },

            // discard all else
            (_, _) => None,
        }
    }

    /// # Functionality:
    /// tries to simulate gravity for the given Cell. Returns `None` if it fails and `Some if it succeeds`
    /// 
    /// loads the chunks if necessary
    fn simulate_diagonal(&mut self, _world_map: &mut ChunkManager, coords_1: &GlobalCoords, _cell1_properties: CellTypeProperties) -> Option<()> {
        let rand_bool = rand::thread_rng().gen_bool(0.5);
        let coords_2: &GlobalCoords = &(if rand_bool { *coords_1 + (0, 1) } else { *coords_1 + (0, -1) });

        let _cell2 =  self.get_cell(coords_2)?;
        todo!()
    }

    fn simulate_cell(&mut self, world_map: &mut ChunkManager, coords: &GlobalCoords) -> Option<()> {

        // ensure that the correct chunk is loaded
        // and place it into the first slot because this fuction will be called a lot
        self.load_chunk(world_map, &ChunkCoords::from(coords), true);

        // load the CellTypeProperties because they will be repeatedly passed on into the other functions
        let cell_properties_cell1: CellTypeProperties = Into::<CellTypeProperties>::into(self.chunk?.cells[Index::from(*coords).i].cell_type);

        // match the StateOfAggregation to the correct cell behavior
        match cell_properties_cell1.state {
            StateOfAggregation::ImmovableSolid => Some(()),
            _ => Some(())
        }
    }



    pub fn draw_cells(&self, screen: &mut [u8]) {

        // this is horribly slow
        screen.chunks_mut(4).enumerate().for_each(|(index, color)| {

            let (x, y) = (index as i32 / SCREEN_HEIGHT as i32, index as i32 / SCREEN_WIDTH as i32);
            let cell = self.get_cell(&GlobalCoords::from((x,y))).unwrap_or((Cell::new(cells::CellType::Pink), false)).0;
            color.copy_from_slice(&cell.color[0..]);
            //a.iter_mut().for_each(|p| *p = 200 );
        });

    }
}