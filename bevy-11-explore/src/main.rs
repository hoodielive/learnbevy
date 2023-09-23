use bevy::prelude::*;
use bevy::window::PrimaryWindow;

fn main() {
    App::new().add_plugins((DefaultPlugins, HelloPlugin)).run();
}

// Components.

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        // add things to our app here.
        app.insert_resource(GreetTimer(Timer::from_seconds(4.0, TimerMode::Repeating)))
            .add_systems(Startup, add_people)
            .add_systems(Update, greet_people);
    }
}

// Systems.

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Oyeku Osa".to_string())));
    commands.spawn((Person, Name("Ogunda Ogbe".to_string())));
    commands.spawn((Person, Name("Irete Tutu".to_string())));
}

// Resources.

#[derive(Resource)]
struct GreetTimer(Timer);

fn greet_people(
    time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>    
) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}
