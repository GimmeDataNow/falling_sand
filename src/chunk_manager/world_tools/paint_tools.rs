use crate::chunk_manager;
use crate::chunk_manager::{Cell, CellType};
use crate::config;

/// # Functionality:
/// This enum dictates what shape the `paint_brush()` function should assume.
/// # Options:
/// The options of `BrushType` are: `Square` and `Circle`.
#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum BrushType {
    Square(usize),
    Circle(usize)
}

impl chunk_manager::ChunkManager {

    /// # Functionality:
    /// This function draws a line in the simulation space using the `bresenham` line drawing algorithm
    pub fn paint_brush(&mut self, coords: (i32, i32), brush_type: BrushType, paint_material: CellType) {

        let radius = match brush_type {
            BrushType::Square(radius) => radius,
            _ => 1 
        };

        // bresenham algorithm
        for x in 0..radius {
            for y in 0..radius {

                match brush_type {
                    BrushType::Square(_) => { self.set_cell_at_global_coords((coords.0 + x as i32 - (radius/2) as i32, coords.1 + y as i32 - (radius/2) as i32), Cell::build_cell(paint_material)); },
                    _ => ()
                };
                
            }
        }
    }

    /// # Functionality:
    /// This function draws a line in the simulation space using the `bresenham` line drawing algorithm
    pub fn draw_line(&mut self, cam_pos: (i32, i32), mouse_prev_cell: (isize, isize), mouse_cell: (isize, isize), brush_type: BrushType, paint_material: CellType) {

        // bresenham algorithm
        for (x, y) in bresenham::Bresenham::new(mouse_prev_cell, mouse_cell) {

            // set cells
            self.paint_brush((x as i32 + cam_pos.0 as i32 - config::SCREEN_WIDTH/2, -y as i32 + cam_pos.1 as i32 + config::SCREEN_HEIGHT/2), brush_type, paint_material);
        }
    }


    
}
