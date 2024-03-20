# Spawning entities & querying components:

Commands provide a safe, efficient and deferred way to modify your game's state, 
facilitating tasks like spawning, despawning, and adding components to entities, 
while queries offer immediate access to existing components, enabling data retrieval
and mutation within the ECS architecture. 

# Spawning entities

You can spawn entities with Commands, e.g.: 

`
commands.spawn(( Position { x: 0.0, y: 0.0 }, Velocity { x: 1.0, y: 1.0 }))
`

Note: pass in a tuple of all the components it is to consist of.

Queue up actions to spawn/despawn entities and add/remove components on existing entites.
Do not take effect right away, commands in the queue are automatically applied when its safe
to mutate the World. Require exclusive access to the World and cannot be applied in parallel.

Similar to performing INSERT and DELETE operations in a database.

# Querying components

You can query all entities that have certain components with systems via Query, e.g.:

`
fn update_position(mut query: Query<(&Velocity, &mut Position)>)
`

Note: Dependency injection based on Type (with bevy scheduler). 
You aren't making direct calls to these functions. Bevy's scheduler takes care of this.

Directly access the components stored in Bevy's ECS for reading and writing.
Can be executed in parallel unless two systems both query the same component type and at 
least one access is mutable.

Similar to performing SELECT and UPDATE operations in a database.

# App

The App struct ties together all the elements of the ECS architecture and executes your code.

`
fn main()
{
    App::new()
        .add_plugins(DefaultPlugins)
        .run();
}
`

Data: Your game's data.

Schedule: List of systems and their run order.

Runner Function: Manages the application's event loop. Each frame it executes Bevy's internal systems and
your application layer.

# Code example

`
# Cargo.toml
[package]
name = "name of project"
version = "0.1.0"
edition = "2021"

[dependencies]
bevy = "0.12.0"

# Enable a small amount of optimization in debug mode
[profile-dev]
opt-level = 1

# Enable high optimizations
[profile.dev.package."*"]
opt-level = 3
`

# Basic 

`
#![allow(dead_code)]

use bevy::prelude::*;

#[derive(Component, Debug)]
struct Position { x: f32, y: 32 }

#[derive(Component, Debug)]
struct Velocity { x: f32, y: 32 }

fn main()
{
    // Main game loop
    // This is an event loop. As such will run until closed.

    App::new
        .add_systems(Startup, spawn_spaceship)
        .add_systems(Update, (update_position, print_position))
        .add_plugins(DefaultPlugins)
        .run();
}

fn spawn_spaceship(mut commands: Commands)
{
    commands.spawn(( Position { x: 0, y: 0 }, Velocity { x: 1.0, y: 1.0 }));
}

fn update_position(mut query: Query<(&Position, &mut Velocity)>)
{
    for (position, velocity) in query.iter_mut()
    {
        position.x += velocity.x;
        position.y += velocity.y;
    }
}

fn print_position(query: Query<(Entity, &Position)>)
{
    for (entity, position) in query.iter_mut()
    {
        info!("Entity {:?} is at position {:?}", entity, position);
    }
}
`

