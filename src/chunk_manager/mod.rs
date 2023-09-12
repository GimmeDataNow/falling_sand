//module rules;
#![allow(dead_code)]

// my imports
pub mod chunks;
pub mod world_tools;
mod renderer;
use crate::custom_errors;
use crate::config;
use self::chunks::cells::{CellType, Cell, CellTypeProperties, StateOfAggregation};
use rand::Rng;
use rand::thread_rng;


/// # Functionality:
/// This contain the `HashMap` containing the chunks with the `chunk-coordinates` as the key.
/// # Notice:
/// This should probably implement `Singletons` to ensure that there is only one instance of the `ChunkManager`.
pub struct ChunkManager {
    pub map: fnv::FnvHashMap<(i32, i32), chunks::Chunk>,
    pub generation: u16
}

// should prob only have one of these active at any given time, but I don't quite understand Singletons yet
impl ChunkManager {

    /// # Functionality:
    /// This creates a new instance of `ChunkManager` and initializes an empty chunk.
    pub fn new() -> Self {

        // probably a better way to do this but I don't know how
        // I really dislike the implicit type declaration that is made by default_coords and val
        let mut map = fnv::FnvHashMap::default();

        // player spawns here
        let default_coords: (i32, i32) = (0, 0);

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
            if global_coords.0 >= 0 { global_coords.0 / config::CHUNK_SIZE_I32 + 1 } else { global_coords.0 / config::CHUNK_SIZE_I32 },
            if global_coords.1 >= 0 { global_coords.1 / config::CHUNK_SIZE_I32 + 1 } else { global_coords.1 / config::CHUNK_SIZE_I32 }
        )
        //(global_coords.0 / config::CHUNK_SIZE_I32, global_coords.1 / config::CHUNK_SIZE_I32)
    }

    /// # Functionality:
    /// Cenvert the coordinates to local in-chunk coordinates for further processing.
    fn to_local(global_coords: (i32, i32)) -> (i32, i32) {
        (global_coords.0.rem_euclid(config::CHUNK_SIZE_I32), global_coords.1.rem_euclid(config::CHUNK_SIZE_I32))
    }

    fn to_index(local_coords: (i32, i32)) -> usize {
        (local_coords.0 + local_coords.1 * config::CHUNK_SIZE_I32) as usize
    }

    /// # Functionality:
    /// Check if the `Chunk` is loaded in the `chunk-map`. If not, it inserts the loaded chunk into the `chunk-map` and returns it.
    fn get_or_load_chunk(&mut self, chunk_coords: (i32, i32)) -> &mut chunks::Chunk {
        if !self.map.contains_key(&chunk_coords) {
            self.map.insert(chunk_coords, chunks::Chunk::load_chunk(chunk_coords));
        }
        self.map.get_mut(&chunk_coords).unwrap()
    }

    pub fn set_chunk(&mut self, global_coords: (i32, i32), cell_type: CellType) -> Option<()> {
        let chunk_coords = ChunkManager::to_chunk_coords(global_coords);
        
        if let Some(chunk) = self.map.get_mut(&chunk_coords) {
            chunk.cells = chunks::Chunk::new_with_fill(cell_type, chunk_coords).cells;
            return Some(());
        }
        None
    }

    /// # Functionality:
    /// Saves the chunk.
    /// # TODO: NOT WOKING || REDO
    pub fn simple_save(&mut self, chunk_coords: &(i32, i32)) -> Result<(), custom_errors::CellError> {
        // check if the chunk is loaded
        if self.map.contains_key(&chunk_coords) {
            
            // Save the chunk to disk before unloading if needed
            self.map.get(&chunk_coords).ok_or(custom_errors::CellError::FailedToSave)?.save_chunk()?;
            
            Ok(())
        } else {
            // error if the chunk is not loaded
            Err(custom_errors::CellError::TargtNotLoaded)
        }
    }

    /// # Functionality:
    /// Check if the `Chunk` is loaded in the `chunk-map`. Then it will try to save the `Chunk` and then it removes it from the `chunk-map`.
    pub fn unload_chunk_at_coords(&mut self, chunk_coords: &(i32, i32)) -> Result<(), custom_errors::CellError> {

        // check if the chunk is loaded
        if self.map.contains_key(&chunk_coords) {

            // Save the chunk to disk before unloading if needed
            self.map.get(&chunk_coords).ok_or(custom_errors::CellError::CouldNotComplete)?.save_chunk()?;

            // Remove the chunk from the hashmap to unload it
            self.map.remove(&chunk_coords);
            Ok(())
        } else {
            // error if the chunk is not loaded
            Err(custom_errors::CellError::FailedToUnload)
        }
    }

    /// # Functionality:
    /// Retreives the `Cell` with the given coordinates and returns an `Option<Cell>`. Returns `None` if the cell is not loaded.
    pub fn get_cell_at_global_coords(&self, coords: (i32, i32)) -> Option<Cell> {

        // Step 1: Convert global coordinates to chunk coordinates
        let chunk_coords: (i32, i32) = ChunkManager::to_chunk_coords(coords);

        // Step 2: Check if the ChunkManager contains the chunk
        if let Some(chunk) = self.map.get(&chunk_coords) {

            // Step 3: Convert local chunk coordinates to cell coordinates
            let local_coords: (i32, i32) = ChunkManager::to_local(coords);

            // Step 4: Access the cell in the chunk
            let cell_index = (local_coords.0 + local_coords.1 * config::CHUNK_SIZE_I32) as usize;

            // return the cell
            return Some(chunk.cells[cell_index]);
        }

        // Return None if the chunk is not found
        None
    }

    pub fn get_celltype_at_global_coords(&self, coords: (i32, i32)) -> Option<&str> {

        // Step 1: Convert global coordinates to chunk coordinates
        let chunk_coords: (i32, i32) = ChunkManager::to_chunk_coords(coords);

        // Step 2: Check if the ChunkManager contains the chunk
        if let Some(chunk) = self.map.get(&chunk_coords) {

            // Step 3: Convert local chunk coordinates to cell coordinates
            let local_coords: (i32, i32) = ChunkManager::to_local(coords);

            // Step 4: Access the cell in the chunk
            let cell_index = (local_coords.0 + local_coords.1 * config::CHUNK_SIZE_I32) as usize;

            // return the cell
            return Some(chunk.cells[cell_index].get_cell_properties().name);
        }

        // Return None if the chunk is not found
        None
    }

    /// # Functionality:
    /// Set a `Cell` at a given coordinate, even if the `Chunk` is not loaded.
    pub fn get_cell_at_global_coords_force_load(&mut self, coords: (i32, i32)) -> Option<Cell> {

        // Step 1: Convert global coordinates to chunk coordinates
        let chunk_coords: (i32, i32) = ChunkManager::to_chunk_coords(coords);

        if !self.map.contains_key(&chunk_coords) {
            ChunkManager::get_or_load_chunk(self, chunk_coords);
        }
        // Step 2: Check if the ChunkManager contains the chunk
        if let Some(chunk) = self.map.get(&chunk_coords) {

            // Step 3: Convert local chunk coordinates to cell coordinates
            let local_coords: (i32, i32) = ChunkManager::to_local(coords);

            // Step 4: Access the cell in the chunk
            let cell_index = (local_coords.0 + local_coords.1 * config::CHUNK_SIZE_I32) as usize;

            // return the cell
            return Some(chunk.cells[cell_index]);
        }

        // Return None if the chunk is not found
        None
    }

    pub fn update_generation(&mut self, coords: (i32, i32)) {
        if let Some(chunk) = self.map.get_mut(&ChunkManager::to_chunk_coords(coords)) {
            let local_coords = ChunkManager::to_local(coords);
            chunk.cells[(local_coords.0 + local_coords.1 * config::CHUNK_SIZE_I32) as usize].generation = self.generation
        }
    }

    /// true if it needs updating
    fn needs_updating(&self, coords: (i32, i32)) -> Option<bool> {
        let chunk = self.map.get(&ChunkManager::to_chunk_coords(coords))?;
        let local_coords = ChunkManager::to_local(coords);
        Some(chunk.cells[(local_coords.0 + local_coords.1 * config::CHUNK_SIZE_I32) as usize].generation != self.generation)
    }

    /// # Functionality:
    /// Set a `Cell` at a given coordinate, given that the `Chunk` is loaded.
    pub fn set_cell_at_global_coords(&mut self, coords: (i32, i32), cell: Cell) -> Option<()> {

        // Step 1: Convert global coordinates to chunk coordinates
        let chunk_coords: (i32, i32) = ChunkManager::to_chunk_coords(coords);

        // Step 2: Check if the ChunkManager contains the chunk
        if let Some(chunk) = self.map.get_mut(&chunk_coords) {

            // Step 3: Convert local chunk coordinates to cell coordinates
            let local_coords: (i32, i32) = ChunkManager::to_local(coords);

            // Step 4: Access the cell in the chunk
            let cell_index = (local_coords.0 + local_coords.1 * config::CHUNK_SIZE_I32) as usize;

            // replace cell
            chunk.cells[cell_index] = cell;

            // mark as updated
            self.update_generation(coords);

            // return the cell
            return Some(());
        }

        // Return None if the chunk is not found
        None
    }

    /// # Functionality:
    /// Checks if a cell is solid.
    pub fn is_solid(&self, coords: (i32, i32)) -> Option<()> {
        
        // get the cell state
        let cell_state = self.get_cell_at_global_coords(coords)?.get_cell_properties().state;

        // check if cell is not an `ImmovableSolid` or a `Granular` material
        (cell_state == StateOfAggregation::ImmovableSolid || cell_state == StateOfAggregation::Granular).then(|| ())
    }



    /// # Functionality:
    /// Swaps the cells
    fn swap_cells_at_global_coords(&mut self, coords_1: (i32, i32), coords_2: (i32, i32)) -> Option<()> {

        // store the cells
        let cell_1_state = self.get_cell_at_global_coords_force_load(coords_1)?;
        let cell_2_state = self.get_cell_at_global_coords_force_load(coords_2)?;

        // swap the cells
        self.set_cell_at_global_coords(coords_1, cell_2_state);
        self.set_cell_at_global_coords(coords_2, cell_1_state);

        // return if ok
        Some(())
    }
    
    

    fn get_neighboring_cells(&self, _coords: (i32, i32)) -> [Cell; 8] {

        // the previous implementation was bad. need to REDO
        todo!()
    }
    
    /// # Functionality:
    /// Checks if the density of the cell at `coords_1` is greater `>` than `coords_2`. Retuns none if the cell properties could not be retreived
    fn compare_density(&self, coords_1: (i32, i32), coords_2: (i32, i32)) -> Option<()> {
        (self.get_cell_at_global_coords(coords_1)?.get_cell_properties().density > self.get_cell_at_global_coords(coords_2)?.get_cell_properties().density).then(|| ())
    }
    
    // will have to be remade for gasses to work properly || REDO
    /// # Functionality:
    /// Swaps the cells
    #[inline]
    fn vertical(&mut self, coords: (i32, i32), density_based: bool, normal_gravity: bool) -> Option<()> {

        // convert the bool into a usable variable
        let dy: i32 = if normal_gravity { -1 } else { 1 };

        if density_based {
            // checks if the cell it is trying to swap to is not solid and checks if it has a greater density than the target cell
            if !self.is_solid(coords).is_some() && !self.is_solid((coords.0, coords.1 + dy)).is_some() && self.compare_density(coords, (coords.0, coords.1 + dy)).is_some() {

                // swap
                self.swap_cells_at_global_coords(coords, (coords.0, coords.1 + dy));

                // return an `Ok`
                return Some(());
            }
        }
        // checks if the cell it is trying to swap to is not solid
        else if !self.is_solid((coords.0, coords.1 + dy)).is_some() {

            // swap
            self.swap_cells_at_global_coords(coords, (coords.0, coords.1 + dy));

            // return an `Ok`
            return Some(());
        }
        None
    }


    fn check_sides(&self, ref_coords: (i32, i32), coords: (i32, i32), density_based: bool) -> [bool; 2] {

        let left_coords = (coords.0 - 1, coords.1);
        let right_coords = (coords.0 + 1, coords.1);

        if density_based {
            [self.is_solid(left_coords).is_none() && self.compare_density(ref_coords, left_coords).is_some(), self.is_solid(right_coords).is_none() && self.compare_density(ref_coords, right_coords).is_some()]
        }
        else {
            [self.is_solid(left_coords).is_none(), self.is_solid(right_coords).is_none()]
        }
    }
    
    fn diagonal(&mut self, coords: (i32, i32), density_based: bool, normal_gravity: bool) -> Option<()> {
        let offset = if normal_gravity { -1 } else { 1 };

        let same_layer = self.check_sides(coords, coords, density_based);
        let offset_layer = self.check_sides(coords, (coords.0, coords.1 + offset), density_based);

        let paths = [same_layer[0] && offset_layer[0], same_layer[1] && offset_layer[1]];
        let rand_bool = thread_rng().gen_bool(0.5);

        if rand_bool && paths[0] {
            if self.swap_cells_at_global_coords(coords, (coords.0 - 1, coords.1 + offset))
            .or_else( || self.swap_cells_at_global_coords(coords, (coords.0 + 1, coords.1 - offset)) )
            .is_some() {
                return Some(());
            }
        }
        else if !rand_bool && paths[1] {
            if self.swap_cells_at_global_coords(coords, (coords.0 + 1, coords.1 + offset))
            .or_else( || self.swap_cells_at_global_coords(coords, (coords.0 - 1, coords.1 + offset)) )
            .is_some() {
                return Some(());
            }
        }

        None
    }

    fn horizontal(&mut self, coords: (i32, i32), density_based: bool) -> Option<()> {

        let paths = self.check_sides(coords, coords, density_based);
        let rand_bool = thread_rng().gen_bool(0.5);

        if rand_bool && paths[0] {
            if self.swap_cells_at_global_coords(coords, (coords.0 - 1, coords.1))
            .or_else( || self.swap_cells_at_global_coords(coords, (coords.0 + 1, coords.1)) )
            .is_some() {
                return Some(());
            }
        }
        else if !rand_bool && paths[1] {
            if self.swap_cells_at_global_coords(coords, (coords.0 + 1, coords.1))
            .or_else( || self.swap_cells_at_global_coords(coords, (coords.0 - 1, coords.1)) )
            .is_some() {
                return Some(());
            }
        }

        None
    }

    fn granular(&mut self, coords: (i32, i32), density_based: bool, normal_gravity: bool) -> Option<()> {
        self.vertical(coords, density_based, normal_gravity)
        .or_else(|| self.diagonal(coords, density_based, normal_gravity))?;
        Some(())
    }

    fn liquid(&mut self, coords: (i32, i32), density_based: bool, normal_gravity: bool) -> Option<()> {
        self.vertical(coords, density_based, normal_gravity)
        .or_else(|| self.diagonal(coords, density_based, normal_gravity))
        .or_else(|| self.horizontal(coords, density_based))?;
        Some(())
    }
    fn gas(&mut self, coords: (i32, i32), density_based: bool, normal_gravity: bool) -> Option<()> {
        self.vertical(coords, density_based, normal_gravity)
        .or_else(|| self.diagonal(coords, density_based, normal_gravity))
        .or_else(|| self.horizontal(coords, density_based))?;
        Some(())
    }

    pub fn iterate_area_around_coordinate(&mut self, x: i32, y: i32) {

        // Loop through the cells within the area
        for dx in 0..=config::SIMULATION_HEIGHT_I32 {
            for dy in 0..=config::SIMULATION_HEIGHT_I32 {

                // convert dx and dy such that the iteration is centered around the provided coordinate
                let coords = (x + dx - (config::SIMULATION_WIDTH_I32 / 2), y - dy + (config::SIMULATION_WIDTH_I32 / 2));

                // Get the cell at the current coordinates without errors
                if let Some(cell) = self.get_cell_at_global_coords(coords) {
                    
                    if !self.needs_updating(coords).unwrap_or(false) {
                        
                        // Retrieve the state of aggregation based on the cell's type
                        let state_of_aggregation = CellTypeProperties::get_cell_properties(cell.cell_type).state;
                        
                        // Match the state of aggregation to perform appropriate actions
                        match state_of_aggregation {
                            StateOfAggregation::Granular => {
                                // Handle granular cells
                                self.granular(coords, false, true);
                            }
                            StateOfAggregation::Liquid => {
                                // Handle liquid cells
                                self.liquid(coords, true, true);
                            }
                            StateOfAggregation::Gas => {
                                // Handle gas cells
                                self.gas(coords, true, false);
                            }
                            _ => ()
                        }
                    }
                }
            }
        }
    }
}