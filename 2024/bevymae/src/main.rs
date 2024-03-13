// We use a RESOURCE when we need something to be 
// standalone (not associated with any other data).
// For example, you could create a resource to store
// your game's graphic settings, or the data for the
// currently active game mode or session.
// Note: Resources are singletons.

#[derive(Resource)]
struct GameSettings
{
    current_level: u32,
    difficulty: u32,
    max_time_seconds: u32,
}

fn setup_game(mut commands: Commands)
{
    commands.insert_resource(
        GameSettings
        {
            current_level: 1,
            difficulty: 100,
            max_time_seconds: 60,
        }
    );
}

fn spawn_extra_enemies(mut commands: Commands, game_settings: Res<GameSettings>)
{
    if game_settings.difficulty > 50
    {
       commands.spawn((
        // ...        
       ));
    }
}

#[derive(Component)]
struct Xp(u32);

#[derive(Component)]
// This is a health component:
struct Health
{
    current: u32,
    max: u32
}

#[derive(Component)]
struct Player;

fn level_up(mut query: Query<(&mut Xp, &mut Health)>)
{
    for (mut xp, mut health) in query.iter_mut()
    {
        if xp.0 > 1000
        {
           xp.0 -= 1000;
           health.max += 25;
           health.current = health.max;
        }
    }
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>)
{
    commands.spawn((
        Player,
        Health
        {
            current: 100,
            max: 125,
        },
        Xp(0),
        SpriteBundle
        {
           texture: asset_server.load("player.png") ,
           transform: Transform::from_xyz(25.0, 50.0, 0.0),
           ..Default::default()
        },
    ));
}
