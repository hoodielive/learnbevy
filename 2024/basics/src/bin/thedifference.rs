#![allow(dead_code)]
use bevy::prelude::*;

// Components

#[derive(Component, Debug)]
struct Position { x: f32, y: f32 }

#[derive(Component, Debug)]
struct Velocity { x: f32, y: f32 }

fn main()
{
    // Game Loop runs continuously.
}

// Systems:

fn spawn_spaceship(mut commands: Commands)
{
    // creates entity or object
    // In ECS, our spaceship would be an entity
    // and its position and velocity would be
    // be the components that this entity consists
    // of. The system would handle the movement based
    // on the position and velocity component.
    
    commands.spawn(( Position { x: 0.0, y: 0.0 }, Velocity { x: 1.0, y: 1.0 } ));
}

fn update_position(mut query: Query<(&Velocity, &mut Position)>)
{
    // reads and updates Components
    // Query reads velocity and position and then updates
    // the position based on the velocity. Its important to
    // know ECS data is stored in components and behavior is
    // stored in systems. There is not a tight coupling between
    // the update_position and the spaceship. There is NO class
    // for the spaceship. Its just a entity that is spawned into
    // the game world with 2 components.
    
    for (velocity, mut position) in query.iter_mut()
    {
        position.x += velocity.x;
        position.y += velocity.y;
    }
}
