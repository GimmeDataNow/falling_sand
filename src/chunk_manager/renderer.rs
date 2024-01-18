use crate::chunk_manager::ChunkManager;
use crate::config::{SCREEN_WIDTH_USIZE, SCREEN_HEIGHT, SCREEN_WIDTH, SCREEN_ARRAY_LENGTH};
use crate::Cell;

impl ChunkManager {

    pub fn force_load_area(&mut self, cam_pos: (i32, i32)) {
        for dy in 0..SCREEN_HEIGHT {
            for dx in 0..SCREEN_WIDTH {
        
                let cell_x = cam_pos.0 + dx - SCREEN_WIDTH/2;
                let cell_y = cam_pos.1 - dy + SCREEN_HEIGHT/2;
        
                let coords = (cell_x, cell_y);
                self.get_cell_at_global_coords_force_load(coords);
            }
        }
    }
    fn draw_precursor_v2(self, cam_pos: &(i32, i32)) -> Box<[[u8;4]; SCREEN_ARRAY_LENGTH]> {

        // this holds all of the color data
        let mut color_map = Box::new([[0;4]; SCREEN_ARRAY_LENGTH]);

        // Loop through the cells within the area
        for dy in 0..SCREEN_HEIGHT {
            for dx in 0..SCREEN_WIDTH {
        
                let cell_x = cam_pos.0 + dx - SCREEN_WIDTH/2;
                let cell_y = cam_pos.1 - dy + SCREEN_HEIGHT/2;
        
                let coords = (cell_x, cell_y);
                if dx == SCREEN_WIDTH/2 && dy == SCREEN_HEIGHT/2 {
                    color_map[dx as usize + dy as usize * SCREEN_WIDTH_USIZE] = [207, 47, 116, 255]
                } else {
                    color_map[dx as usize + dy as usize * SCREEN_WIDTH_USIZE] = self.get_cell_at_global_coords(coords).unwrap_or(Cell::build_cell(super::chunks::cells::CellType::Acid)).color
                }
            }
        }
        color_map
    }

    /// # Functionality:
    /// Generates the array containing all of `the cell-color` data.
    fn draw_precursor(&mut self, cam_pos: (i32, i32)) -> Box<[[u8;4]; SCREEN_ARRAY_LENGTH]> {

        // this holds all of the color data
        let mut color_map = Box::new([[0;4]; SCREEN_ARRAY_LENGTH]);

        // Loop through the cells within the area
        for dy in 0..SCREEN_HEIGHT {
            for dx in 0..SCREEN_WIDTH {
        
                let cell_x = cam_pos.0 + dx - SCREEN_WIDTH/2;
                let cell_y = cam_pos.1 - dy + SCREEN_HEIGHT/2;
        
                let coords = (cell_x, cell_y);
                if dx == SCREEN_WIDTH/2 && dy == SCREEN_HEIGHT/2 {
                    color_map[dx as usize + dy as usize * SCREEN_WIDTH_USIZE] = [207, 47, 116, 255]
                } else {
                    color_map[dx as usize + dy as usize * SCREEN_WIDTH_USIZE] = self.get_cell_at_global_coords_force_load(coords).unwrap_or(Cell::default()).color
                }
            }
        }
        color_map
    }

    /// # Functionality:
    /// Copies the buffer to the screen.
    fn draw_to_buffer(frame: &mut [u8], buffer: Box<[[u8;4]; SCREEN_ARRAY_LENGTH]>) {
        for (i, c) in frame.chunks_exact_mut(4).enumerate() {
            c.copy_from_slice(&buffer[i])
        }
    }

    /// # Functionality:
    /// Draws the frame.
    pub fn draw(&mut self, cam_pos: (i32, i32), frame: &mut [u8]) {
        let buffer = self.draw_precursor(cam_pos);
        ChunkManager::draw_to_buffer(frame, buffer)
    }

    pub fn draw_v2(self, cam_pos: &(i32, i32), frame: &mut [u8]) {
        std::thread::scope(|s| {
            s.spawn(|| {
                let buffer = self.draw_precursor_v2(cam_pos);
                ChunkManager::draw_to_buffer(frame, buffer)
            });
        });
        
    }
}