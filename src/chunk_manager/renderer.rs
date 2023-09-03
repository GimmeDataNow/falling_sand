use crate::chunk_manager::ChunkManager;
use crate::config::{SCREEN_WIDTH_USIZE, SCREEN_HEIGHT, SCREEN_WIDTH, SCREEN_ARRAY_LENGTH};
use crate::Cell;

impl ChunkManager {
    fn draw_precursor(&mut self, cam_pos: (i32, i32)) -> Box<[[u8;4]; SCREEN_ARRAY_LENGTH]> {

        // this holds all of the color data
        let mut color_map = Box::new([[0;4]; SCREEN_ARRAY_LENGTH]);

        // Loop through the cells within the area
        for dy in 0..SCREEN_HEIGHT {
            for dx in 0..SCREEN_WIDTH {
        
                let cell_x = cam_pos.0 + dx - SCREEN_WIDTH/2;
                let cell_y = cam_pos.1 - dy + SCREEN_HEIGHT/2;
        
                let coords = (cell_x, cell_y);

                color_map[dx as usize + dy as usize * SCREEN_WIDTH_USIZE] = self.get_cell_at_global_coords_force_load(coords).unwrap_or(Cell::default()).color
            }
        }
        color_map
    }

    fn draw_to_buffer(frame: &mut [u8], buffer: Box<[[u8;4]; SCREEN_ARRAY_LENGTH]>) {
        for (i, c) in frame.chunks_exact_mut(4).enumerate() {
            c.copy_from_slice(&buffer[i])
        }
    }

    pub fn draw(&mut self, cam_pos: (i32, i32), frame: &mut [u8]) {
        let buffer = self.draw_precursor(cam_pos);
        ChunkManager::draw_to_buffer(frame, buffer)
    }
}