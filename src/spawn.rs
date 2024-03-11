use std::time::Instant;

use legion::World;

use crate::*;

static VILLAIN_MARKS : [char; 4] = ['!', '"', '#', '$'];

pub fn spawn_hero (ecs : &mut World, pos : Position) {
    ecs.push((
        Character,
        Player,
        Victim,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@')
        },
    ));
}
pub fn spawn_villain(ecs : &mut World, pos : Position, i: usize) {
    ecs.push((
        Character,
        Naughty,
        Hunter,
        RandomWalk(Instant::now()),
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437(VILLAIN_MARKS[i % VILLAIN_MARKS.len()])
        },
    ));
}
pub fn spawn_seed(ecs : &mut World, pos : Position) {
    ecs.push((
        Food,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('.')
        },
    ));
}
pub fn spawn_powerup(ecs : &mut World, pos : Position) {
    ecs.push((
        Food,
        Superfood,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('*')
        },
    ));
}