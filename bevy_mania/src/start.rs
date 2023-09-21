#[derive(Component)]
struct Player {
    name: String,
}

#[derive(Component)]
struct Score {
    value: usize,
}

#[derive(Resource, Default)]
// Mutable
struct GameState {
    current_round: usize,
    total_players: usize,
    winning_player: Option<String>,
}

#[derive(Resource)]
// Read-only
struct GameRules {
    winning_score: usize,
    max_rounds: usize,
    max_players: usize,
}

// Systems: logic that runs on entities, comps and resources.

fn print_message_system() {
   println!("Message from the system.");
}

fn new_round_system(game_rules: Res<GameRules>, mut game_state: ResMut<GameState>) {
    game_state.current_round += 1;
    println!( "Begin round {} of {}", game_rules.max_rounds);
}
