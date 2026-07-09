use colored::Colorize;

use crate::world::map::{Content, Point, World};

pub fn render_map(world: &World) {
    for y in (-world.radius..=world.radius).rev() {
        for x in -world.radius..=world.radius {
            let point = world.get_imut_point(x as i32, y as i32);
            point_render(point);
        }
        print!("\n");
    }
}

fn get_smell_color(smell: f32) -> (u8, u8, u8) {
    let max_val = 1.0;

    let gray = 60;

    if smell <= 0.0 {
        return (gray, gray, gray);
    } else {
        let factor = (smell / max_val).min(1.0);

        let r = (gray as f32 * (1.0 - factor)) as u8;
        let g = (gray as f32 + (200.0 - gray as f32) * factor) as u8;
        let b = (gray as f32 * (1.0 - factor)) as u8;

        (r, g, b)
    }
}

fn point_render(point: &Point) {
    match point.content {
        Content::Empty => print!("{}", "  ".on_custom_color(get_smell_color(point.smell))),
        Content::Food => print!(
            "{}",
            "$ ".on_custom_color(get_smell_color(point.smell)).black()
        ),
        Content::Cell => print!("{}", "()".on_black().white()),
        Content::Border => print!("{}", "  ".on_red().black()),
    }
}
