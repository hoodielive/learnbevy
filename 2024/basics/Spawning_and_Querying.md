# Spawning entities & querying components:

Commands provide a safe, efficient and deferred way to modify your game's state, 
facilitating tasks like spawning, despawning, and adding components to entities, 
while queries offer immediate access to existing components, enabling data retrieval
and mutation within the ECS architecture. 

# Spawning entities

You can spawn entities with Commands, e.g.: 

`
commands.spawn(( Position { x: 0.0, y: 0.0 }, Velocity { x: 1.0, y: 1.0 }))
`

Note: pass in a tuple of all the components it is to consist of.

Queue up actions to spawn/despawn entities and add/remove components on existing entites.
Do not take effect right away, commands in the queue are automatically applied when its safe
to mutate the World. Require exclusive access to the World and cannot be applied in parallel.

Similar to performing INSERT and DELETE operations in a database.

# Querying components

You can query all entities that have certain components with systems via Query, e.g.:

`
fn update_position(mut query: Query<(&Velocity, &mut Position)>)
`

Note: Dependency injection based on Type (with bevy scheduler). 
You aren't making direct calls to these functions. Bevy's scheduler takes care of this.

Directly access the components stored in Bevy's ECS for reading and writing.
Can be executed in parallel unless two systems both query the same component type and at 
least one access is mutable.

Similar to performing SELECT and UPDATE operations in a database.

# App

The App struct ties together all the elements of the ECS architecture and executes your code.

`rust
fn main()
{
    App::new()
        .add_plugins(DefaultPlugins)
        .run();
}
`
