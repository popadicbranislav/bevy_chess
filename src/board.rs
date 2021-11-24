use bevy::prelude::*;
use bevy_mod_picking::*;

pub struct Square {
    pub x: u8,
    pub y: u8,
}

impl Square {
    fn is_white(&self) -> bool {
        (self.x + self.y + 1) % 2 == 0
    }
}

#[derive(Default)]
pub struct SelectedSquare {
    entity: Option<Entity>,
}

fn create_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Add meshes and materials
    let mesh = meshes.add(Mesh::from(shape::Plane { size: 1. }));
    // let white_material = materials.add(Color::rgb(1., 0.9, 0.9).into());
    // let black_material = materials.add(Color::rgb(0., 0.1, 0.1).into());

    // Spawn 64 squares
    for i in 0..8 {
        for j in 0..8 {
            commands
                .spawn_bundle(PbrBundle {
                    mesh: mesh.clone(),
                    // Change material according to position to get alternating pattern
                    material: if (i + j + 1) % 2 == 0 {
                        materials.add(Color::rgb(1., 0.9, 0.9).into())
                    } else {
                        materials.add(Color::rgb(0., 0.1, 0.1).into())
                    },
                    transform: Transform::from_translation(Vec3::new(i as f32, 0., j as f32)),
                    ..Default::default()
                })
                .insert_bundle(PickableBundle::default())
                .insert(Square { x: i, y: j });
        }
    }
}

pub fn color_and_select_squares(
    mut selected_square: ResMut<SelectedSquare>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(
        Entity,
        &Square,
        &Handle<StandardMaterial>,
        &Hover,
        &Selection,
    )>,
) {
    for (entity, square, material_handle, hover, selection) in query.iter() {
        // Get the actual material
        let material = materials.get_mut(material_handle).unwrap();

        // Change the material color
        material.base_color = if hover.hovered() {
            Color::rgb(0.8, 0.3, 0.3)
        } else if selection.selected() {
            selected_square.entity = Some(entity);
            // } else if Some(entity) == selected_square.entity {
            Color::rgb(0.9, 0.1, 0.1)
        } else if square.is_white() {
            Color::rgb(1., 0.9, 0.9)
        } else {
            Color::rgb(0., 0.1, 0.1)
        };
    }
}

pub struct BoardPlugin;
impl Plugin for BoardPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<SelectedSquare>()
            .add_startup_system(create_board.system())
            .add_system(color_and_select_squares.system());
    }
}
