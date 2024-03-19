#![allow(dead_code)]
use bevy::prelude::*;

#[derive(Component)]
// Component
struct Position { x: f32, y: f32 }

// Entity also born from struct
struct Entity(u64);

// System
fn print_position_system(query: Query<&Position>)
{
    for position in &query
    {
        println!("Position: {} {}.", position.x, position.y)
    }
}

// System
fn hello()
{
    println!("Hello!")
}

fn main() 
{
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, hello)
        .run();
}

