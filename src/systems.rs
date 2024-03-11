//! The systems are the bits of code providing the game logic.
//! 
//! Author:  X. Gillard
//! Date:    March 2023
//! Licence: MIT 

use bracket_lib::{color::{ColorPair, BLACK, WHITE}, terminal::{to_cp437, DrawBatch, Point, VirtualKeyCode}};
use crate::*;


pub fn build_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(render_map_system())
        .add_system(render_entities_system())
        .add_system(user_input_system())
        .build()
}

#[system]
pub fn render_map(#[resource] map: &Map) {
    let mut drawbatch = DrawBatch::new();
    drawbatch.target(0);

    for y in 0..map.height {
        for x in 0..map.width {
            let pos   = Position{x, y};
            let glyph = match map[pos] {
                TileType::Wall  => to_cp437('0'),
                TileType::Floor => to_cp437(' '),
            };
            drawbatch.set(
                Point::new(x,y), 
                ColorPair::new(WHITE, BLACK), 
                glyph);
        }
    }

    drawbatch.submit(0).expect("draw error");
}

#[system]
#[read_component(Render)]
#[read_component(Position)]
pub fn render_entities(ecs: &SubWorld) {
    let mut drawbatch = DrawBatch::new();
    drawbatch.target(1);

    <(&Position, &Render)>::query()
        .iter(ecs)
        .for_each(|(pos, render)| {
            drawbatch.set(
                pos.into_point(),
                render.color,
                render.glyph,
            );
        });

    drawbatch.submit(10_000).expect("draw entity error");
}

#[system]
#[read_component(Position)]
#[write_component(Position)]
pub fn user_input(
    ecs: &mut SubWorld,
    #[resource] map: &Map,
    #[resource] key: &Option<VirtualKeyCode>
) {
    // todo: see page 117
}