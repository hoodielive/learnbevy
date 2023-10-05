use bevy::prelude::*;

const ARENA_WIDTH: f32 = 800.0;
const ARENA_HEIGHT: f32 = 600.0;
const PLAYER_HEIGHT: f32 = 32.0;
const PLAYER_WIDTH: f32 = 22.0;


// Components
// Structs that contain some information

#[derive(Copy, Clone)]
enum Side {
    Left,
    Right,
}

#[derive(Component)]
struct Player {
    side: Side,
}

// Systems
// Functions that run on some subset (which is determined by query(ies)) of components

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz( ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, 1.0),
            ..default()
    });
}

<<<<<<< HEAD
const cat_sprite = asset_server.load("textures/cat-sprite.png");
=======
let cat_sprite = asset_server.load("textures/cat-sprite.png");

>>>>>>> 56127234bc6359e1a04c858455cfa99b5c95aa38
initialize_player(
    &mut commands,
    cat_sprite.clone(),
    Side::Left,
    PLAYER_WIDTH / 2.0,
    PLAYER_HEIGHT / 2.0,
);

initialize_player(
    &mut commands,
    cat_sprite.clone(),
    Side::Right,
    ARENA_WIDTH / 2.0,
    PLAYER_HEIGHT / 2.0,
);

fn initialize_player(
    commands: &mut Commands,
    cat_sprite: Handle<Image>,
    side: Side,
    x: f32,
    y: f32,
) {
    commands.spawn((
        Player { side },
        SpriteBundle {
           texture: cat_sprite,
           transform: Transform::from_xyz(x, y, 0.0),
           ..default()
        },
    ));    
}

fn main()
{
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Cat Volleyball".into(),
                resolution: (ARENA_WIDTH, ARENA_HEIGHT).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_startup_system(setup)
        .run();
}
