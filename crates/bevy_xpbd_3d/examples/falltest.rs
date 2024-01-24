 #![allow(clippy::unnecessary_cast)]

use bevy::prelude::*;
use bevy_xpbd_3d::{math::*, prelude::*};
use examples_common_3d::XpbdExamplePlugin;
use big_space::{FloatingOrigin, GridCell};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins.build().disable::<TransformPlugin>(),
                      XpbdExamplePlugin,
                      big_space::FloatingOriginPlugin::<i128>::default()
        ))
        .insert_resource(ClearColor(Color::rgb(0.05, 0.05, 0.1)))
        .insert_resource(Msaa::Sample4)
        .add_systems(Startup, setup)
        .add_systems(Update, logger)
        .run();
}

/// The acceleration used for movement.
#[derive(Component)]
struct FallTag(u128);

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let cube_mesh = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
    let cube_size = 2.0;

    commands.spawn((FloatingOrigin, GridCell::<i128>::default()));
    // Ground
    commands.spawn((
        PbrBundle {
            mesh: cube_mesh.clone(),
            material: materials.add(Color::rgb(0.7, 0.7, 0.8).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::new(10000.0, 90.0, 10000.0)),
            ..default()
        },
        RigidBody::Static,
        //FloatingOrigin,
        GridCell::<i128> { x: 0, y: -1, z: 0 },
        Collider::cuboid(1.0, 1.0, 1.0),
    ));

    // Spawn cube stacks
    let position = Vec3::new(0f32, 1000f32, 0f32);
    commands.spawn((
        PbrBundle {
            mesh: cube_mesh.clone(),
            material: materials.add(Color::rgb(0.2, 0.7, 0.9).into()),
            transform: Transform::from_translation(position)
                .with_scale(Vec3::splat(cube_size as f32)),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::cuboid(1.0, 1.0, 1.0),
        FallTag(0),
        //FloatingOrigin,
        GridCell::<i128>::default()
    ));
}

fn logger(
    time: Res<Time>,
    //keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&GlobalTransform, &GridCell<i128>, &mut FallTag)>,
) {
    
    let delta_time = time.delta().as_millis();

    for (gtx, gc, mut c) in &mut query {
        c.0 += delta_time;
        if c.0 > 2000 {
            println!("at {:?}\t {:?}", gc, gtx.translation());
            c.0 = 0;
        }
    }
    
}
