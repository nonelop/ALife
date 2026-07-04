use crate::{
    cell::cell::Cell,
    world::map::{Content, Point, World},
};

pub fn render_map(world: &World, cell: &Cell) {
    for y in (-world.radius..=world.radius).rev() {
        for x in -world.radius..=world.radius {
            let point = world.get_imut_point(x as i32, y as i32);
            point_render(point, cell);
        }
        print!("\n");
    }
}

fn point_render(point: &Point, cell: &Cell) {
    match point.content {
        Content::Empty => print!(". "),
        Content::Food => print!("* "),
        Content::Border => print!("# "),
    }
    if cell.x == point.x && cell.y == point.y {
        print!("\x1b[1D\x1b[1D@ ");
    }
}
