use crate::view::render_map;
use crate::world::map::World;
use crate::cell::cell::Cell;

pub fn tick(world: &mut World, cells: &mut Vec<Cell>) {

    println!("\x1B[H");

    // World

    render_map(world);
    world.update();

    // Cells

    let mut new_cells_buffer: Vec<Cell> = Vec::new();

    for cell in &mut *cells {
        cell.movement(world);
        
        if cell.energy >= 110.0 {
            new_cells_buffer.push(Cell::new((cell.x, cell.y)));
        }
    }

    for cell in new_cells_buffer {
        cells.push(cell);
    }
}