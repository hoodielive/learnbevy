use bevy::prelude::*;

fn main() {
    App::new()
        .add_startup_system(setup)
        .add_system(print_names)
        .run();
}

pub fn setup(mut commands: Commands) {
    commands.spawn((
                Person {
                    name: "Oyeku".to_string(),
                },
                Employed {
                    job: Job::Doctor,
                }
            )
    );
    commands.spawn((
            Person {
                name: "Ogbe".to_string(),
            },
            Employed {
                job: Job::Lawyer,
            }
        )
    );
    commands.spawn((
                Person {
                    name: "Iwori".to_string(),
                },
                Employed {
                    job: Job::FireFighter,
                }
            )
    );
}

pub fn print_names(person_query: Query<&Person>) {
    for person in person_query.iter() {
        println!("Name: {}", person.name);
    }
}

#[derive(Component)]
pub struct Person {
    pub name: String,
}

#[derive(Component)]
pub struct Employed {
    pub job: Job 

}

#[derive(Debug)]
pub enum Job {
    Doctor,
    FireFighter,
    Lawyer.
}
