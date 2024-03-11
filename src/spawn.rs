use legion::World;

use crate::*;

static VILLAIN_MARKS : [char; 4] = ['!', '"', '#', '$'];

pub fn spawn_hero (ecs : &mut World, pos : Position) {
    ecs.push((
        Player,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@')
        },
    ));
}
pub fn spawn_villain(ecs : &mut World, pos : Position, i: usize) {
    ecs.push((
        //Player,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437(VILLAIN_MARKS[i % VILLAIN_MARKS.len()])
        },
    ));
}
pub fn spawn_seed(ecs : &mut World, pos : Position) {
    ecs.push((
        //Player,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('.')
        },
    ));
}
pub fn spawn_cherry(ecs : &mut World, pos : Position) {
    ecs.push((
        //Player,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('*')
        },
    ));
}