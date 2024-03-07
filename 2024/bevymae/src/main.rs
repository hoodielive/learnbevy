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

fn main() 
{
    
}
