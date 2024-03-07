use bevy::prelude::*;

#[derive(Component)]
struct Position { x: f32, y: f32 }
struct Entity(u64);

fn print_position_system(query: Query<&Position>)
{
    for position in &query
    {
        println!("Position: {} {}.", position.x, position.y)
    }
}

fn hello()
{
    println!("Hello!")
}

fn main() 
{
    App::new()
        .add_systems(Update, hello)
        .run();
}

