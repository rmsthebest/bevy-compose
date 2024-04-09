use bevy::prelude::*;
use bevy_compose::{button, compose, Compose, Composer};

#[derive(Resource)]
struct Count(i32);

fn ui(world: &mut World) -> impl Compose {
    let count = world.resource_ref::<Count>().0;

    world.resource_mut::<Count>().0 += 1;

    (
        format!("High five count: {}", count),
        button(String::from("Up high!")),
    )
}

fn main() {
    let mut app = App::new();

    app.world.insert_resource(Count(0));
    app.world.spawn(Composer::new(ui));
    app.world
        .spawn((Camera2dBundle::default(), IsDefaultUiCamera));

    app.add_plugins(DefaultPlugins)
        .add_systems(Update, compose)
        .run();
}
