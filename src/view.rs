use crate::{
    world::map::{Content, Point, World},
};

pub fn render_map(world: &World) {
    for y in (-world.radius..=world.radius).rev() {
        for x in -world.radius..=world.radius {
            let point = world.get_imut_point(x as i32, y as i32);
            point_render(point);
        }
        print!("\n");
    }
}

fn point_render(point: &Point) {
    match point.content {
        Content::Empty => print!(". "),
        Content::Food => print!("* "),
        Content::Cell => print!("@ "),
        Content::Border => print!("# "),
    }
}
