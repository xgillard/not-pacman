//! Entities are whatever object happen to occur in the game. 
//! These entities have properties which are called components.
//! 
//! Author:  X. Gillard
//! Date:    March 2023
//! Licence: MIT 

/// This is going to be our action hero (aka the pizza guy, aka
/// the main character w/ which you usually play on old arcade).
/// The goal of the hero in the game is to eat all of the available
/// seeds that have been laid you on the ground. 
/// The hero also has the ability to eat some magic powerups which 
/// have the side effect of turinging all the villains into edible
/// stuff for a limited period of time.
pub struct Hero{}
/// Tese are the villains of the game. They are trying to go eat 
/// the hero. When the hero eat magic powerups, then the villain
/// are made edible. If the hero collides with a villain as it is
/// edible, then the hero eats the villain. When the villain is 
/// not edible but a collision happens between the hero and a 
/// villain, then it is the villain who eats the hero.
/// 
/// When the villain is eated by the hero, it is respawned somewhere
/// on the map. When the hero gets killed it is likewise also respawned 
/// somewhere on the map.
pub struct Villain{}
/// This are the stuffs on the floor which the hero is trying to eat
/// as much of as it possibly can before being eated by a villain.
pub struct Seeds{}
/// This is a magic powerup. It can also be eated by the hero just like
/// the regular seeds. However, whenever the hero eats a powerup, all
/// the villains are made edible for a short (a few seconds) period of
/// time.
pub struct Powerup{}
