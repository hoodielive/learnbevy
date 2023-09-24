# Stuff 

## Primary window
.add_plugins(DefaultPlugins.set(WindowPlugin {
           primary_window: Some(Window {
            title: "Name of Window".to_string(),
            resolution: (ARENA_WIDTH, ARENA_HEIGHT).into(),
            ..default()
        });
        ..default()
}));

This is interesting, you can always use the defaults:

.add_plugins(DefaultPlugins)
