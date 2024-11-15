use bevy::prelude::*;
use bevy_pg_calendar::prelude::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(PGCalendarPlugin{
        active:      false,
        hour_length: 5,
        start_hour:  6,
        ..default()
    })
    .run();
}
