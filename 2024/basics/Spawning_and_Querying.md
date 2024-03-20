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

# Querying components

You can query all entities that have certain components with systems via Query, e.g.:

`
fn update_position(mut query: Query<(&Velocity, &mut Position)>)
`
