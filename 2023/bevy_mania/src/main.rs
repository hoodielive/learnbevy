#![allow(unused_variables)]

use bevy::prelude::*;

fn main()
{
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor 
        {
            title: "Osa Meji".to_string(),
            width: 598.0,
            height: 676.0,
            ..Default::default()
        })
       .add_plugins(DefaultPlugins)
       .run();
}
