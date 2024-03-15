#![allow(dead_code)]
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
        
        .add_plugins(DefaultPlugins)
        .add_systems(Update, hello_world_system)
        .run();
}

fn hello_world_system()
{
    println!("Hello, World!")
}

mod game
{
    use bevy::app::PluginGroupBuilder;
    use super::logic::LogicPlugin;
    use super::camera::CameraPlugin;
    use super::physics::PhysicsPlugin;

    pub struct GamePlugins;

    impl PluginGroup for GamePlugins
    {
        fn build(self) -> PluginGroupBuilder
        {
            PluginGroupBuilder::start::<Self>()
                .add(CameraPlugin::default())
                .add(PhysicsPlugin::default())
                .add(LogicPlugin)
        }
    }
}

// We create a plugin by implementing Plugin on a struct
// and defining a method (build) which should mutate the
// App passed to it by performing the necessary setup such
// as adding systems, resources and events to your game:

pub struct CameraPlugin
{
    debug: bool,    
}

impl Plugin for CameraPlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_systems(Startup, initialize_camera);

        if self.debug
        {
            app.add_plugins(DebugCameraPlugin)
        }
    }
}

fn initialize_camera(mut commands: Commands)
{
    commands.spawn(Camera2dBundle::default());
}

fn DebugCameraPlugin()
{
    unimplemented!()    
}
