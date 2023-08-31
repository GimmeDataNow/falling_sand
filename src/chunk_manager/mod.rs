pub mod chunks;
mod custom_error;

use crate::config;
use self::chunks::cells::{CellType, Cell, CellTypeProperties, StateOfAggregation};

// should prob only have one of these active at any given time, but I don't quite understand Singletons yet

/// # Functionality:
/// This contain the `HashMap` containing the chunks with the chunk coordinates as the keys.
/// # Notice:
/// This should probably implement `Singletons` to ensure that there is only one instance of the `ChunkManager`.
pub struct ChunkManager {
    pub map: fnv::FnvHashMap<(i32, i32), chunks::Chunk>
}

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
        ChunkManager { map }
    }

    /// # Functionality:
    /// Check if the chunk is loaded in the chunk map. If not, it inserts the loaded chunk into the chunk map and returns it.
    fn get_or_load_chunk(&mut self, chunk_coords: (i32, i32)) -> &mut chunks::Chunk {
        if !self.map.contains_key(&chunk_coords) {
            self.map.insert(chunk_coords, chunks::Chunk::load_chunk(chunk_coords));
        }
        self.map.get_mut(&chunk_coords).unwrap()
    }

    pub fn simple_save(&mut self, chunk_coords: &(i32, i32)) -> Result<(), custom_error::CustomErrors> {
        // check if the chunk is loaded
        if self.map.contains_key(&chunk_coords) {
            
            // Save the chunk to disk before unloading if needed
            self.map.get(&chunk_coords).ok_or(custom_error::CustomErrors::CouldNotComplete)?.save_chunk()?;
            
            Ok(())
        } else {
            // error if the chunk is not loaded
            Err(custom_error::CustomErrors::CouldNotComplete)
        }
    }
    /// # Functionality:
    /// Check if the chunk is loaded in the chunk map. Then it will try to save the chunk and then it removes it from the chunk map.
    fn unload_chunk_at_coords(&mut self, chunk_coords: &(i32, i32)) -> Result<(), custom_error::CustomErrors> {

        // check if the chunk is loaded
        if self.map.contains_key(&chunk_coords) {

            // Save the chunk to disk before unloading if needed
            self.map.get(&chunk_coords).ok_or(custom_error::CustomErrors::CouldNotComplete)?.save_chunk()?;

            // Remove the chunk from the hashmap to unload it
            self.map.remove(&chunk_coords);
            Ok(())
        } else {
            // error if the chunk is not loaded
            Err(custom_error::CustomErrors::CouldNotComplete)
        }
    }

    /// # Functionality:
    /// Retreives the cell with the given coordinates and returns an Option<Cell>. Returns None if the cell is not loaded.
    pub fn get_cell_at_global_coords(&self, coords: (i32, i32)) -> Option<Cell> {

        // Step 1: Convert global coordinates to chunk coordinates
        let chunk_x = coords.0 / config::CHUNK_SIZE_I32;
        let chunk_y = coords.1 / config::CHUNK_SIZE_I32;

        // Step 2: Check if the ChunkManager contains the chunk
        if let Some(chunk) = self.map.get(&(chunk_x, chunk_y)) {

            // Step 3: Convert local chunk coordinates to cell coordinates
            let local_x = coords.0.rem_euclid(config::CHUNK_SIZE_I32);
            let local_y = coords.1.rem_euclid(config::CHUNK_SIZE_I32);

            // Step 4: Access the cell in the chunk
            let cell_index = (local_x + local_y * config::CHUNK_SIZE_I32) as usize;

            // return the cell
            return Some(chunk.cells[cell_index]);
        }

        // Return None if the chunk is not found
        None
    }
    pub fn get_cell_at_global_coords_force_load(&mut self, coords: (i32, i32)) -> Option<Cell> {

        // Step 1: Convert global coordinates to chunk coordinates
        let chunk_x = coords.0 / config::CHUNK_SIZE_I32;
        let chunk_y = coords.1 / config::CHUNK_SIZE_I32;

        if !self.map.contains_key(&(chunk_x, chunk_y)) {
            ChunkManager::get_or_load_chunk(self, (chunk_x, chunk_y));
        }
        // Step 2: Check if the ChunkManager contains the chunk
        if let Some(chunk) = self.map.get(&(chunk_x, chunk_y)) {

            // Step 3: Convert local chunk coordinates to cell coordinates
            let local_x = coords.0.rem_euclid(config::CHUNK_SIZE_I32);
            let local_y = coords.1.rem_euclid(config::CHUNK_SIZE_I32);

            // Step 4: Access the cell in the chunk
            let cell_index = (local_x + local_y * config::CHUNK_SIZE_I32) as usize;

            // return the cell
            return Some(chunk.cells[cell_index]);
        }

        // Return None if the chunk is not found
        None
    }

    pub fn set_cell_at_global_coords(&mut self, coords: (i32, i32), cell: Cell) -> Option<()> {

        // Step 1: Convert global coordinates to chunk coordinates
        let chunk_x = coords.0 / config::CHUNK_SIZE_I32;
        let chunk_y = coords.1 / config::CHUNK_SIZE_I32;

        // Step 2: Check if the ChunkManager contains the chunk
        if let Some(chunk) = self.map.get_mut(&(chunk_x, chunk_y)) {

            // Step 3: Convert local chunk coordinates to cell coordinates
            let local_x = coords.0.rem_euclid(config::CHUNK_SIZE_I32);
            let local_y = coords.1.rem_euclid(config::CHUNK_SIZE_I32);

            // Step 4: Access the cell in the chunk
            let cell_index = (local_x + local_y * config::CHUNK_SIZE_I32) as usize;

            chunk.cells[cell_index] = cell;

            // return the cell
            return Some(());
        }

        // Return None if the chunk is not found
        None
    }

    pub fn is_solid(&self, coords: (i32, i32)) -> Option<()> {
        (self.get_cell_at_global_coords(coords)?.get_cell_properties().state == StateOfAggregation::ImmovableSolid
        || self.get_cell_at_global_coords(coords)?.get_cell_properties().state == StateOfAggregation::Granular).then(|| ())
    }

    fn compare_density(&self, coords_1: (i32, i32), coords_2: (i32, i32)) -> Option<()> {
        (self.get_cell_at_global_coords(coords_1)?.get_cell_properties().density > self.get_cell_at_global_coords(coords_2)?.get_cell_properties().density).then(|| ())
    }

    /// # Functionality:
    /// Swaps the cells
    fn swap_cells_at_global_coords(&mut self, coords_1: (i32, i32), coords_2: (i32, i32)) {
        // Step 1: Convert global coordinates to chunk coordinates
        let chunk_x1 = coords_1.0 / config::CHUNK_SIZE_I32;
        let chunk_y1 = coords_1.1 / config::CHUNK_SIZE_I32;
        let chunk_x2 = coords_2.0 / config::CHUNK_SIZE_I32;
        let chunk_y2 = coords_2.1 / config::CHUNK_SIZE_I32;
    
        // Step 2: Check if the cells are in different chunks
        if chunk_x1 == chunk_x2 && chunk_y1 == chunk_y2 {
            // Cells are in the same chunk
            let chunk_coords = (chunk_x1, chunk_y1);
    
            // Step 3: Convert local chunk coordinates to cell coordinates
            let local_x1 = coords_1.0.rem_euclid(config::CHUNK_SIZE_I32);
            let local_y1 = coords_1.1.rem_euclid(config::CHUNK_SIZE_I32);
            let local_x2 = coords_2.0.rem_euclid(config::CHUNK_SIZE_I32);
            let local_y2 = coords_2.1.rem_euclid(config::CHUNK_SIZE_I32);
    
            // Step 4: Access and Swap the Cells within the same chunk
            if let Some(chunk) = self.map.get_mut(&chunk_coords) {
                let cell_index1 = (local_x1 + local_y1 * config::CHUNK_SIZE_I32) as usize;
                let cell_index2 = (local_x2 + local_y2 * config::CHUNK_SIZE_I32) as usize;
    
                if cell_index1 < config::CHUNK_LENGTH_USIZE && cell_index2 < config::CHUNK_LENGTH_USIZE {
                    let temp_cell = chunk.cells[cell_index1];
                    chunk.cells[cell_index1] = chunk.cells[cell_index2];
                    chunk.cells[cell_index2] = temp_cell;
                }
            }
        } else {
            // Cells are in different chunks
            let chunk1_coords = (chunk_x1, chunk_y1);
            let chunk2_coords = (chunk_x2, chunk_y2);
    
            // Step 5: Check if both chunks are present in the ChunkManager
            if let (Some(mut chunk1), Some(mut chunk2)) = (self.map.get(&chunk1_coords).cloned(), self.map.get(&chunk2_coords).cloned()) {
                // Step 6: Convert local chunk coordinates to cell coordinates
                let local_x1 = coords_1.0.rem_euclid(config::CHUNK_SIZE_I32);
                let local_y1 = coords_1.1.rem_euclid(config::CHUNK_SIZE_I32);
                let local_x2 = coords_2.0.rem_euclid(config::CHUNK_SIZE_I32);
                let local_y2 = coords_2.1.rem_euclid(config::CHUNK_SIZE_I32);
    
                // Step 7: Access and Swap the Cells between the two chunks
                let cell_index1 = (local_x1 + local_y1 * config::CHUNK_SIZE_I32) as usize;
                let cell_index2 = (local_x2 + local_y2 * config::CHUNK_SIZE_I32) as usize;
    
                if cell_index1 < config::CHUNK_LENGTH_USIZE && cell_index2 < config::CHUNK_LENGTH_USIZE {
                    std::mem::swap(&mut chunk1.cells[cell_index1], &mut chunk2.cells[cell_index2]);
                }
            }
        }
    }
    
    

    fn get_neighboring_cells(chunk_manager: &ChunkManager, x: i32, y: i32) -> [Cell; 8] {
        let mut neighbors: [Cell; 8] = Default::default(); // Initialize the array with default values
    
        let mut index = 0;
        // Iterate over neighboring coordinates (relative to the cell at (x, y))
        for dx in -1..=1 {
            for dy in -1..=1 {
                // Skip the cell itself (dx = 0, dy = 0)
                if dx == 0 && dy == 0 {
                    continue;
                }
    
                let neighbor_x = x + dx;
                let neighbor_y = y + dy;
    
                // Get the neighboring cell if it exists and add it to the array
                if let Some(neighbor_cell) = chunk_manager.get_cell_at_global_coords((neighbor_x, neighbor_y)) {
                    neighbors[index] = neighbor_cell;
                }
    
                index += 1;
            }
        }
    
        neighbors
    }
    
    // will have to be remade for gasses to work properly
    #[inline]
    fn vertical(&mut self, coords: (i32, i32), density_based: bool, normal_gravity: bool) -> Option<()> {
        let dy: i32 = if normal_gravity { -1 } else { 1 };
        if density_based {
            if !self.is_solid(coords).is_some() && !self.is_solid((coords.0, coords.1 + dy)).is_some() && self.compare_density(coords, (coords.0, coords.1 + dy)).is_some() {
                self.swap_cells_at_global_coords(coords, (coords.0, coords.1 + dy));
                return Some(());
            }
        }
        else if !self.is_solid(coords).is_some() && !self.is_solid((coords.0, coords.1 + dy)).is_some() {
            self.swap_cells_at_global_coords(coords, (coords.0, coords.1 + dy));
            return Some(());
        }
        None
    }

    fn diagonal(chunk_manager: &mut ChunkManager, x: i32, y: i32, density_based: bool) -> Option<()> {
        todo!()
    }
    fn sides_check(&self, coords: (i32, i32), density_based: bool, celltype: CellType) -> [bool; 2] {
        todo!()
    }
    

    fn iterate_area_around_coordinate(&mut self, x: i32, y: i32) {
        // Calculate the half width and half height of the area
        let half_width = config::SIMULATION_WIDTH_I32 / 2;
        let half_height = config::SIMULATION_HEIGHT_I32 / 2;
    
        // Loop through the cells within the area
        for dx in -half_width..=half_width {
            for dy in -half_height..=half_height {
                let cell_x = x + dx;
                let cell_y = y + dy;

                let coords = (cell_x, cell_y);
    
                // Get the cell at the current coordinates
                if let Some(cell) = self.get_cell_at_global_coords((cell_x, cell_y)) {
                    // Retrieve the state of aggregation based on the cell's type
                    let state_of_aggregation = CellTypeProperties::get_cell_properties(cell.cell_type).state;
    
                    // Match the state of aggregation to perform appropriate actions
                    match state_of_aggregation {
                        StateOfAggregation::Granular => {
                            // Handle granular cells
                            // Example: Perform granular behavior
                            
                            self.vertical(coords, false, false);
                        }
                        StateOfAggregation::Liquid => {
                            // Handle liquid cells
                            // Example: Perform liquid behavior
                        }
                        StateOfAggregation::Gas => {
                            // Handle gas cells
                            // Example: Perform gas behavior
                        }
                        _ => ()
                    }
                }
            }
        }
    }
}