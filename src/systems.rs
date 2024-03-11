//! The systems are the bits of code providing the game logic.
//! 
//! Author:  X. Gillard
//! Date:    March 2023
//! Licence: MIT 

use std::time::{Duration, Instant};

use bracket_lib::{color::{ColorPair, BLACK, WHITE}, terminal::{to_cp437, DrawBatch, Point, VirtualKeyCode}};
use crate::*;

/// Duration of a powerup
const POWERUP_DURATION : u64 = 3;

/// This function creates the ECS schedule which decides when a given system should be run
pub fn build_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(render_map_system())
        .add_system(user_input_system())
        .add_system(random_walk_system())
        .add_system(superfood_system())
        .add_system(eat_food_system())
        .add_system(hunt_down_victim_system())
        .add_system(reset_roles_system())
        .add_system(render_entities_system())
        .build()
}

/// This system renders the world map
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

/// This system renders all entities in the world
#[system]
#[read_component(Render)]
#[read_component(Position)]
pub fn render_entities(ecs: &SubWorld) {
    let mut foodbatch = DrawBatch::new();
    foodbatch.target(1);

    <(&Position, &Render)>::query()
        .filter(component::<Food>())
        .iter(ecs)
        .for_each(|(pos, render)| {
            foodbatch.set(
                pos.into_point(),
                render.color,
                render.glyph,
            );
        });

    foodbatch.submit(10_000).expect("draw entity error");


    let mut charactersbatch = DrawBatch::new();
    foodbatch.target(2);

    <(&Position, &Render)>::query()
        .filter(component::<Character>())
        .iter(ecs)
        .for_each(|(pos, render)| {
            charactersbatch.set(
                pos.into_point(),
                render.color,
                render.glyph,
            );
        });

    charactersbatch.submit(10_000).expect("draw entity error");
}

/// This system deals with the user input
#[system]
#[read_component(Position)]
#[write_component(Position)]
pub fn user_input(
    ecs: &mut SubWorld,
    #[resource] map: &Map,
    #[resource] key: &Option<VirtualKeyCode>
) {
    if let Some(key) = key {
        let movement = match key {
            VirtualKeyCode::Left  => (-1, 0),
            VirtualKeyCode::Right => ( 1, 0),
            VirtualKeyCode::Up    => ( 0,-1),
            VirtualKeyCode::Down  => ( 0, 1),
            _                     => ( 0, 0),
        };
        <&mut Position>::query()
            .filter(component::<Player>())
            .iter_mut(ecs)
            .for_each(|pos| {
                *pos = next_position(map, *pos, movement);
            })
    }
}

#[system]
#[read_component(Position)]
#[read_component(Player)]
#[read_component(Food)]
pub fn eat_food(ecs: &mut SubWorld, cmd: &mut CommandBuffer) {
    let mut hero = Position::default();
    <&Position>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .for_each(|p| hero = *p);

    <(Entity, &Position)>::query()
        .filter(component::<Food>())
        .iter(ecs)
        .for_each(|(entity, pos)| 
            if *pos == hero { 
                cmd.remove(*entity) 
            });
}

#[system]
#[read_component(Position)]
#[read_component(Player)]
#[read_component(Superfood)]
#[read_component(Naughty)]
#[write_component(Food)]
#[write_component(Victim)]
#[write_component(Render)]
pub fn superfood(ecs: &mut SubWorld, cmd: &mut CommandBuffer) {
    let mut hero = Position::default();
    <&Position>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .for_each(|p| hero = *p);

    let mut villain_as_food = false;
    <(Entity, &Position)>::query()
        .filter(component::<Superfood>())
        .iter(ecs)
        .for_each(|(entity, pos)| 
            if *pos == hero { 
                cmd.remove(*entity);
                villain_as_food = true;
            });
    
    if villain_as_food {
        // after superfood, the hero is no longer a victim
        <(Entity, &mut Render)>::query()
            .filter(component::<Player>())
            .iter_mut(ecs)
            .for_each(|(entity, render)| {
                render.color = ColorPair::new(RED, BLACK);

                cmd.remove_component::<Victim>(*entity);
                cmd.add_component(*entity, Hunter);
                cmd.add_component(*entity, ResetRoles {
                    instant: Instant::now() + Duration::from_secs(POWERUP_DURATION),
                    add: Role::Victim,
                    remove: Role::Hunter
                });
            });
        
        // after superfood, the villains are all food
        <(Entity, &mut Render)>::query()
            .filter(component::<Naughty>())
            .iter_mut(ecs)
            .for_each(|(entity, render)| {
                render.color = ColorPair::new(RED, BLACK);

                cmd.remove_component::<Hunter>(*entity);
                cmd.add_component(*entity, Victim);
                cmd.add_component(*entity, ResetRoles {
                    instant: Instant::now() + Duration::from_secs(POWERUP_DURATION),
                    add: Role::Hunter,
                    remove: Role::Victim
                });
            })
    }
}

#[system]
#[read_component(Position)]
#[read_component(Hunter)]
#[read_component(Victim)]
pub fn hunt_down_victim(ecs: &mut SubWorld, cmd: &mut CommandBuffer) {
    <&Position>::query()
        .filter(component::<Hunter>())
        .iter(ecs)
        .for_each(|hunter| {
            <(Entity, &Position)>::query()
                .filter(component::<Victim>())
                .iter(ecs)
                .for_each(|(entity, victim)| {
                    if hunter == victim {
                        cmd.remove(*entity);
                    }
                })
        });
}

#[system]
#[read_component(ResetRoles)]
#[write_component(ResetRoles)]
#[write_component(Hunter)]
#[write_component(Victim)]
#[write_component(Render)]
pub fn reset_roles(ecs: &mut SubWorld, cmd: &mut CommandBuffer) {
    let now = Instant::now();
    <(Entity, &ResetRoles, &mut Render)>::query()
        .iter_mut(ecs)
        .for_each(|(entity, reset, render)| {
            if now >= reset.instant {
                match reset.remove {
                    Role::Victim => cmd.remove_component::<Victim>(*entity),
                    Role::Hunter => cmd.remove_component::<Hunter>(*entity)
                }

                match reset.add {
                    Role::Victim => cmd.add_component(*entity, Victim),
                    Role::Hunter => cmd.add_component(*entity, Hunter)
                }

                cmd.remove_component::<ResetRoles>(*entity);
                render.color = ColorPair::new(WHITE, BLACK);
            }
        });
}


#[system]
#[write_component(RandomWalk)]
#[write_component(Position)]
pub fn random_walk(
        ecs: &mut SubWorld, 
        #[resource] rng: &mut RandomNumberGenerator,
        #[resource] map: &Map
    ) {
    let now = Instant::now();
    let next= now + Duration::from_millis(250);

    <(&mut RandomWalk, &mut Position)>::query()
        .iter_mut(ecs)
        .for_each(|(rwalk, pos)| {
            if rwalk.0 <= now {
                let choice = rng.roll_dice(1, 4);
                let movement = match choice {
                    1 => (1, 0),
                    2 => (-1, 0),
                    3 => (0, 1),
                    4 => (0, -1),
                    _ => (0, 0)
                };

                *pos   = next_position(map, *pos, movement);
                *rwalk = RandomWalk(next);
            }
        })
}