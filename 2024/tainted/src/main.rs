use bevy::prelude::*;

// Apps are containers for both logic and data
// of your game. Its what controls the loop of
// our game so that our 'systems' can update 
// our world. 

// There are two core parts of every App:
// 1. World, holds your data (entities, components).
// 2. Schedule, holds your logic (systems)

// Your App also holds a pointer->"run function", which
// we can override that control the actual event loop by
// advancing the Schedule which applies your logic (systems)
// to the World.

// The App interface uses a builder pattern which returns the
// modified App each time we call a method so we can chain them
// together until we call 'run'.

fn main() 
{
    App::new()
        // DefaultPlugins adds core plugins that all your game
        // to render on a window provided by your OS.
        // It is usually 'always' added.
        .add_plugins(DefaultPlugins)
        .add_systems(Update, hello_world_system)
        .run();
}

fn hello_world_system()
{
    println!("Hello, World!")
}
