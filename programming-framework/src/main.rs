use bevy::prelude::*;
use bevy::app::AppExit;

fn main() {
    App::new()
        // Bevy itself:
        .add_plugins(DefaultPlugins)
       
        // events: 
        .add_event::<LevelUpEvent>()

        // systems to run once at startup:
        .add_startup_systems(Update (
           player_level_up,
           debug_levelups,
           debug_stats_change,
           bevy::window::close_on_esc);
        )

        // launch the app!
        .run()
}

#[derive(Component)]
struct Health {
    hp: f32,
    extra: f32,
}

#[derive(Component)]
struct PlayerXp(u32);

#[derive(Component)]
struct PlayerName(String);

#[derive(Component)]
struct MainMenuUI;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Friendly;

// Components can be accessed from systems, using queries.
// You can add/remove components on existing entities, using Commands.

#[derive(Bundle)]
struct PlayerBundle {
    xp: PlayerXp,
    name: PlayerName,
    health: Health,
    _p: Player,

    #[bundle]
    sprite: SpriteSheetBundle,
}

fn exit_system(mut exit: EventWriter<AppExit>) {
    exit.send(AppExit);
}


