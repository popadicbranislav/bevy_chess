use bevy::prelude::*;
use bevy_mod_picking::*;

mod board;
mod pieces;

fn main() {
    App::build()
        // Set antialiasing to use 4 samples
        .insert_resource(Msaa { samples: 4 })
        // Set WindowDescriptor Resource to change title and size
        .insert_resource(WindowDescriptor {
            title: "Chess!".to_string(),
            width: 1200.,
            height: 800.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PickingPlugin)
        .add_plugin(InteractablePickingPlugin)
        // .add_plugin(HighlightablePickingPlugin)
        .add_plugin(board::BoardPlugin)
        .add_startup_system(setup.system())
        .add_startup_system(pieces::create_pieces.system())
        .run();
}

fn setup(mut commands: Commands) {
    // Camera
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_matrix(Mat4::from_rotation_translation(
                Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize(),
                Vec3::new(-2.0, 10.0, 4.0),
            )),
            ..Default::default()
        })
        .insert_bundle(PickingCameraBundle::default());

    // Light
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
        ..Default::default()
    });
}
