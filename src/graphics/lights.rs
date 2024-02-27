use bevy::prelude::*;

pub (super) fn spawn_light( 
    mut commands: Commands
) {

    commands.spawn(DirectionalLightBundle {
        ..default()
    });
}
