//! The game state is the shared state that needs to be updated 
//! upon each game 'tick'. This is typically the data which you
//! will store in the shared memory as you implement your multi-player
//! version of the game.
//! 
//! Author:  X. Gillard
//! Date:    March 2023
//! Licence: MIT 

use std::{fs::File, io::{BufReader, Read}};
use legion::{world::World, Resources, Schedule};
use crate::*;

pub struct State {
    ecs: World,
    resources: Resources,
    systems: Schedule,
}

impl State {
    pub fn new() -> Self {
        let ecs = World::default();
        let resources = Resources::default();
        let systems = build_scheduler();
        Self { ecs, resources, systems }
    }

    pub fn load_file(&mut self, fname: &str) -> Result<(usize, usize), std::io::Error> {
        let mut value = String::new();
        BufReader::new(File::open(fname)?).read_to_string(&mut value)?;
    
        let lines  = value.lines().collect::<Vec<_>>();
    
        let width = lines[0].len();
        let height = lines.len();
        
        let mut hero = Position {x: 0, y: 0};
        let mut villains = vec![];
        //
        let mut seeds = vec![];
        let mut cherries = vec![];
        //
        let mut tiles = vec![TileType::Floor; height * width];
    
        for (j, line) in lines.iter().enumerate() {
            for (i, c) in line.chars().enumerate() {
                match c {
                    '@' => { hero = Position{x: i, y: j}         },
                    '!' => { villains.push(Position{x: i, y: j}) },
                    '#' => { tiles[j*width + i] = TileType::Wall     },
                    '.' => { seeds.push(Position{x: i, y: j})    },
                    '*' => { cherries.push(Position{x: i, y: j}) },
                    _   => {/* ignore anything else */}
                }
            }
        }
        
        //
        self.resources.insert(Map{width, height, tiles});
        //
        spawn_hero(&mut self.ecs, hero);
        //
        for (i, villain) in villains.iter().copied().enumerate() {
            spawn_villain(&mut self.ecs, villain, i);
        }
        //
        for seed in seeds {
            spawn_seed(&mut self.ecs, seed);
        }
        // 
        for cherry in cherries {
            spawn_cherry(&mut self.ecs, cherry);
        }
        Ok((width, height))
    }
    
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut bracket_lib::prelude::BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();

        // this keeps track of the key that has potentially been pressed and saves
        // it as a resource in the game world.
        // note: 
        // Any two resources with the same type will be replaced by one another
        // in the ecs. There is thus no need to think of duplicates in this context
        self.resources.insert(ctx.key);
        // 
        self.systems.execute(&mut self.ecs, &mut self.resources);
        // effectively draw everything on screen (in batch to be more efficient)
        render_draw_buffer(ctx).expect("could not render");
    }
}