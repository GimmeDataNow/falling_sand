//module rules;
#![allow(dead_code)]

// my imports
pub mod cells;
pub mod chunks;

use super::coordinates::*;
use cells::{StateOfAggregation, CellTypeProperties, Cell, CellType};
use super::chunk_manager::chunks::Chunk;
use crate::custom_error::*;
use crate::config::{DEFAULT_PLAYER_SPAWN_COORDINATES, SCREEN_HEIGHT, SCREEN_WIDTH};


use rand::Rng;


/// # Functionality:
/// This contain the `HashMap` containing the chunks with the `chunk-coordinates` as the key.
/// # Notice:
/// This should probably implement `Singletons` to ensure that there is only one instance of the `ChunkManager`.
pub struct ChunkManager {
    pub generation: u16,
    pub chunk_cache: ((Option<ChunkCoords>, Option<Chunk>), (Option<ChunkCoords>, Option<Chunk>)),
    pub map: fnv::FnvHashMap<ChunkCoords, chunks::Chunk>,
}

impl ChunkManager {
    pub fn new() -> Self {
        Self {
            generation: 0,
            chunk_cache: ((None, None), (None, None)),
            map: fnv::FnvHashMap::default(),
        }
    }

    pub fn is_cached(&self, chunk_coords: &ChunkCoords) -> Option<bool> {
        if self.chunk_cache.0.0 == Some(*chunk_coords) {
            return Some(true);
        }
        if self.chunk_cache.1.0 == Some(*chunk_coords) {
            return Some(false);
        }
        None
    }

    /// # Functionality:
    /// Return a reference to a chunk from the chunk map
    pub fn get_chunk_from_map(&self, chunk_coords: &ChunkCoords) -> Option<&Chunk> {
        // this is just to hide the .map call
        self.map.get(chunk_coords)
    }

    /// # Functionality:
    /// Return a mutable reference to a chunk from the chunk map
    pub fn get_chunk_from_map_mut(&mut self, chunk_coords: &ChunkCoords) -> Option<&mut Chunk> {
        // this is just to hide the .map call
        self.map.get_mut(chunk_coords)
    }

    /// # Functionality:
    /// Try to return a reference to a chunk from the chunk cache. If the retreival fails, revert to getting it from the chunk map
    pub fn get_chunk_from_cache(&self, chunk_coords: &ChunkCoords) -> Option<&Chunk> {

        // check if it is cached
        match self.is_cached(chunk_coords) {

            // chunk_cache.0.1 is the chunk in the first slot of the ((Option<ChunkCoords>, Option<Chunk>), (Option<ChunkCoords>, Option<Chunk>))
            Some(true) => self.chunk_cache.0.1.as_ref(),

            // chunk_cache.1.1 is the chunk in the second slot of the ((Option<ChunkCoords>, Option<Chunk>), (Option<ChunkCoords>, Option<Chunk>))
            Some(false) => self.chunk_cache.1.1.as_ref(),

            // default to the chunk map
            None => self.get_chunk_from_map(chunk_coords),
        }
    }

    /// # Functionality:
    /// Try to return a mutable reference to a chunk from the chunk cache. If the retreival fails, revert to getting it from the chunk map
    pub fn get_chunk_from_cache_mut(&mut self, chunk_coords: &ChunkCoords) -> Option<&mut Chunk> {

        // check if it is cached
        match self.is_cached(chunk_coords) {

            // chunk_cache.0.1 is the chunk in the first slot of the ((Option<ChunkCoords>, Option<Chunk>), (Option<ChunkCoords>, Option<Chunk>))
            Some(true) => self.chunk_cache.0.1.as_mut(),

            // chunk_cache.1.1 is the chunk in the second slot of the ((Option<ChunkCoords>, Option<Chunk>), (Option<ChunkCoords>, Option<Chunk>))
            Some(false) => self.chunk_cache.1.1.as_mut(),

            // default to the chunk map
            None => self.get_chunk_from_map_mut(chunk_coords),
        }
    }

    /// # Functionality:
    /// Tries to set the chunk to a specific fill.
    fn set_chunk_from_cell_type(&mut self, global_coords: &GlobalCoords, fill: CellType) -> Option<()> {

        // convert to ChunkCoords
        let chunk_coords: ChunkCoords = ChunkCoords::from(*global_coords);

        // check if the chunk exists
        if let Some(chunk) = self.get_chunk_from_cache_mut(&chunk_coords) {

            // set the chunk
            chunk.cells = chunks::Chunk::new_from_cell_type(fill).cells;

            // return
            return Some(());
        }

        // return
        None
    }

    /// # Functionality:
    /// Tries to set the chunk to a specific fill.
    fn set_chunk_from_cell(&mut self, global_coords: &GlobalCoords, fill: Cell) -> Option<()> {

        // convert to ChunkCoords
        let chunk_coords: ChunkCoords = ChunkCoords::from(*global_coords);

        // check if the chunk exists
        if let Some(chunk) = self.get_chunk_from_cache_mut(&chunk_coords) {

            // set the chunk
            chunk.cells = chunks::Chunk::new_from_cell(fill).cells;

            // return
            return Some(());
        }

        // return
        None
    }

    /// # Functionality:
    /// Tries to set the chunk to a specific fill.
    fn set_chunk_from_chunk(&mut self, global_coords: &GlobalCoords, fill: Chunk) -> Option<()> {

        // convert to ChunkCoords
        let chunk_coords: ChunkCoords = ChunkCoords::from(*global_coords);

        // check if the chunk exists
        if let Some(chunk) = self.get_chunk_from_cache_mut(&chunk_coords) {

            // set the chunk
            chunk.cells = fill.cells;

            // return
            return Some(());
        }

        // return
        None
    }

    /// # Functionality:
    /// Load the chunk into the cache. Does not check if the chunk is already loaded or if the Option<Chunk> is None.
    pub fn load_chunk_into_cache(&mut self, chunk_coords: &ChunkCoords, preferred_slot: bool) {

        // get the chunk from the map
        let chunk = self.get_chunk_from_map(chunk_coords).cloned();

        // match the slot
        match preferred_slot {
            true => {self.chunk_cache.0 = (Some(*chunk_coords), chunk)},
            false => {self.chunk_cache.1 = (Some(*chunk_coords), chunk)},
        }
    }

    /// # Functionality:
    /// Load the chunk into the cache. Does not check if the chunk is already loaded or if the Option<Chunk> is None.
    pub fn try_load_chunk_into_cache(&mut self, chunk_coords: &ChunkCoords, preferred_slot: bool) -> Result<(), ChunkError> {

        // is the chunk loaded
        match self.get_chunk_from_map(chunk_coords) {

            Some(chunk) => {

                // clone the chunk
                let chunk_option = Some(*chunk);

                // match slot
                match preferred_slot {
                    true => {self.chunk_cache.0 = (Some(*chunk_coords), chunk_option)},
                    false => {self.chunk_cache.1 = (Some(*chunk_coords), chunk_option)},
                }
            },

            // error
            None => return Err(ChunkError::TargtNotLoaded),
        }
        Ok(())
    }

    /// # Functionality:
    /// Save the chunk to a file
    pub fn save_chunk(&self, chunk_coords: &ChunkCoords) -> Result<(), ChunkError> {
        Ok(
            self.get_chunk_from_cache(chunk_coords)
                .ok_or(ChunkError::OutOfBounds)?
                .save_chunk(chunk_coords)?
        )
    }

    /// # Functionality:
    /// Save the chunk to a file and then unload it from memory
    pub fn unload_chunk_at_coords(&mut self, chunk_coords: &ChunkCoords) -> Result<(), ChunkError> {

        // check if the chunk is even loaded
        match self.get_chunk_from_cache(chunk_coords) {

            // is loaded
            Some(chunk) => {

                // this will error if the chunk fails to save
                // the error is ignored here which may lead to errors
                let _ = chunk.save_chunk(chunk_coords);

                // to delete from the cache the position in the cache is needed
                match self.is_cached(chunk_coords) {

                    // slot 0
                    Some(true) => self.chunk_cache.0 = (None, None),

                    // slot 2
                    Some(false) => self.chunk_cache.1 = (None, None),

                    // not in cache
                    None => (),
                }
                self.map.remove(chunk_coords);
                return Ok(());
            },

            // not loaded
            None => return Err(ChunkError::TargtNotLoaded)
        };
    }

    /// # Functionality:
    /// Loads a chunk into the hashmap. Overrides the chunk if it is already loaded
    pub fn insert_chunk_into_map(&mut self, chunk_coords: ChunkCoords) -> Option<Chunk> {
        self.map.insert(chunk_coords, chunks::Chunk::default())
    }

    /// # Functionality:
    /// Loads a chunk into the hashmap. Overrides the chunk if it is already loaded
    pub fn load_chunk_into_map(&mut self, chunk_coords: &ChunkCoords) -> Option<Chunk> {
        self.insert_chunk_into_map(*chunk_coords)
    }

    /// # Functionality:
    /// Tries to load a chunk into the hashmap. Errors if the chunk is already loaded
    pub fn try_insert_chunk_into_map(&mut self, chunk_coords: ChunkCoords) -> Result<(), ChunkError> {
        match self.map.contains_key(&chunk_coords) {
            true => return Err(ChunkError::TargtAlreadyLoaded),
            false => Ok(()),
        }
    }

    /// # Functionality:
    /// Check if the `Chunk` is loaded in the `chunk-map`. If not, it inserts the loaded chunk into the `chunk-map` and returns it.
    pub fn get_and_force_load_chunk(&mut self, chunk_coords: &ChunkCoords) -> Option<&mut Chunk> {
        if !self.map.contains_key(chunk_coords) {
            self.map.insert(*chunk_coords, Chunk::default());
        }
        return self.map.get_mut(chunk_coords);
    }

    /// # Functionality:
    /// Tries to set the cell in either the cache or in the chunk map. Errors if self.get_chunk_from_cache_mut() returns None
    pub fn set_cell_cache(&mut self, coords: &GlobalCoords, cell: Cell) -> Result<(), ChunkError> {
        
        match self.get_chunk_from_cache_mut(&ChunkCoords::from(coords)) {
            Some(chunk_mut) => {
                let index = Index::from(coords);
                chunk_mut.cells[index.i] = cell;
                Ok(())
            },
            None => Err(ChunkError::TargtNotLoaded),
        }
    }

    /// # Functionality:
    /// Tries to set the cell in either the cache or in the chunk map. Errors if self.get_chunk_from_cache_mut() returns None
    pub fn set_cell(&mut self, coords: &GlobalCoords, cell: Cell) -> Result<(), ChunkError> {
        
        match self.get_chunk_from_cache_mut(&ChunkCoords::from(coords)) {
            Some(chunk_mut) => {
                let index = Index::from(coords);
                chunk_mut.cells[index.i] = cell;
                Ok(())
            },
            None => Err(ChunkError::TargtNotLoaded),
        }
    }

    /// # Functionality:
    /// Swaps two cells
    /// # Warning:
    /// It is the responsibility of the caller to check if it is better to cache the chunks first
    pub fn swap_cells(&mut self, cell_1_coords: &GlobalCoords, cell_2_coords: &GlobalCoords) -> Result<(), CellError> {

        let cell_1 = self
            .get_chunk_from_cache(&ChunkCoords::from(cell_1_coords))
            .ok_or(CellError::CouldNotComplete)?
            .cells[Index::from(cell_1_coords).i]
            .clone();

        let cell_2 = self
            .get_chunk_from_cache(&ChunkCoords::from(cell_2_coords))
            .ok_or(CellError::CouldNotComplete)?
            .cells[Index::from(cell_2_coords).i]
            .clone();

        self.set_cell(cell_2_coords, cell_1)?;
        self.set_cell(cell_1_coords, cell_2)?;

        Ok(())
    }

    /// # Functionality:
    /// Swaps two cells
    /// # Warning:
    /// This function loads the chunks into the cache first
    pub fn swap_cells_with_cache(&mut self, cell_1_coords: &GlobalCoords, cell_2_coords: &GlobalCoords) -> Result<(), CellError> {

        let cell_1_chunk_coords = &ChunkCoords::from(cell_1_coords);
        let cell_2_chunk_coords = &ChunkCoords::from(cell_2_coords);

        self.load_chunk_into_cache(cell_1_chunk_coords, true);
        self.load_chunk_into_cache(cell_2_chunk_coords, false);

        let cell_1 = self
            .get_chunk_from_cache(cell_1_chunk_coords)
            .ok_or(CellError::CouldNotComplete)?
            .cells[Index::from(cell_1_coords).i]
            .clone();

        let cell_2 = self
            .get_chunk_from_cache(cell_2_chunk_coords)
            .ok_or(CellError::CouldNotComplete)?
            .cells[Index::from(cell_2_coords).i]
            .clone();

        self.set_cell(cell_2_coords, cell_1)?;
        self.set_cell(cell_1_coords, cell_2)?;

        Ok(())
    }

    /// # Functionality:
    /// Swaps two cells
    /// # Warning:
    /// It is the responsibility of the caller to ensure that the proper chunks are cached for optimal performance
    pub fn simulate_gravity(&mut self, coords_1: &GlobalCoords) -> Option<()> {

        let coords_2: &GlobalCoords = &(*coords_1 + (0, 1));

        let cell_1_chunk_coords = ChunkCoords::from(coords_1);
        let cell_2_chunk_coords = ChunkCoords::from(coords_2);

        let cell_type_1 = self.get_chunk_from_cache(&cell_1_chunk_coords)?.cells[Index::from(coords_1).i].cell_type;
        let cell_type_2 = self.get_chunk_from_cache(&cell_2_chunk_coords)?.cells[Index::from(coords_2).i].cell_type;

        let cell_1_properties = CellTypeProperties::from(cell_type_1);
        let cell_2_properties = CellTypeProperties::from(cell_type_2);

        match (cell_1_properties.state, cell_2_properties.state) {
            // eliminate the ImmovableSolids immediately
            (StateOfAggregation::ImmovableSolid, _) => None,
            (_, StateOfAggregation::ImmovableSolid) => None,

            // if the target is of a granular type then skip it
            (_, StateOfAggregation::Granular) => None,

            // no more checks are needed because the first match (the one above) prevents this
            (StateOfAggregation::Granular, _) => {

                // swap
                self.swap_cells(coords_1, coords_2).ok()?;

                Some(())
            },

            // liquids
            (StateOfAggregation::Liquid, _) => {

                // density check
                if cell_1_properties.density > cell_2_properties.density {  self.swap_cells( coords_1, coords_2).ok()? };

                Some(())
            },

            // discard all else
            (_, _) => None,
        }
    }

    // more stuff here
    // get cell
    // set cell
    // swap cell
    // simulate gravity

    pub fn draw_cells(&self, screen: &mut [u8]) {

        // this is horribly slow
        // split the iterator into chunks of 4
        // meaning that this makes an iterator over screen with screen.chunks_mut(4)[index] => len() = 4
        screen.chunks_mut(4).enumerate().for_each(|(index, color)| {

            // this conversion seems to be alright
            let (x, y) = (index as i32 % SCREEN_WIDTH as i32, index as i32 / SCREEN_WIDTH as i32);
            //let index = Index::from(GlobalCoords::from((x, y))).i;

            // println
            // println!("-> x:{} y:{}", x, y);
            // println!("index: {}", index);
            // println!("computed index: {}", Index::from(GlobalCoords::from((x,y))).i);


            let cell = self
                .get_chunk_from_cache(&ChunkCoords::from(GlobalCoords::from((x, y))))
                .unwrap_or(&Chunk::new_from_cell_type(cells::CellType::Pink))
                // holy moly this is fucked - overflow without the min()
                .cells[std::cmp::min(Index::from(GlobalCoords::from((x, y))).i, 63)];
            color.copy_from_slice(&cell.color[0..]);
            //a.iter_mut().for_each(|p| *p = 200 );
        });

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

            // soooo turns out this get_cell() call was always erroring and returning the default
            let cell = self.get_cell(&GlobalCoords::from((x,y))).unwrap_or((Cell::new(cells::CellType::Pink), false)).0;
            color.copy_from_slice(&cell.color[0..]);
            //a.iter_mut().for_each(|p| *p = 200 );
        });

    }
}