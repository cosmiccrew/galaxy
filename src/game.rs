use bevy::{
    reflect::{TypePath, TypeUuid},
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, MaterialMesh2dBundle},
};
use rand::Rng;

use crate::prelude::*;

pub struct GalaxyGamePlugin;

impl Plugin for GalaxyGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(EngineState::InGame), setup)
            .add_systems(
                Update,
                (planet_rotation, add_loaded_component, planet_switching)
                    .run_if(in_state(EngineState::InGame)),
            )
            .add_systems(OnExit(EngineState::InGame), teardown::<Loaded>);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    assets: Res<MyAssets>,
    mut materials: ResMut<Assets<EarthlikeMaterial>>,
) {
    // commands.spawn((
    //     MaterialMesh2dBundle {
    //         // mesh: meshes
    //         //     .add(shape::Quad::new(Vec2::new(200., 200.)).into())
    //         //     .into(),
    //         material: materials.add(EarthlikePlanetMaterial {
    //             color: Color::BLUE,
    //             color_texture: assets.dummy.clone(),
    //         }),
    //         ..default()
    //     },
    //     Planet,
    // ));

    // let _b = PlanetConfig {
    //         planet_type: PlanetType::Earthlike,
    //         seed: 100,
    //         ..Default::default()
    //     };

    // commands.spawn(PlanetBundle {
    //     material_mesh_2d_bundle: MaterialMesh2dBundle {
    //         mesh: (),
    //         material: materials.add(asset),
    //         ..default()
    //     },
    //     ..default()
    // });

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(500., 500.)).into())
                .into(),
            material: materials.add(EarthlikeMaterial {
                pixels: 100.,
                ..default()
            }),
            ..default()
        },
        Planet,
    ));

    commands.spawn((
        Planet,
        PlanetSettings {
            planet_type: PlanetType::Earthlike,
            ..default()
        },
    ));
}

fn planet_rotation(
    mut commands: Commands,
    mut query: Query<&mut Transform, With<Planet>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let mut direction = 0f32;

    if keyboard_input.pressed(KeyCode::Left) {
        direction += 1.;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        direction -= 1.;
    }

    for mut object in &mut query {
        object.rotate_z(time.delta_seconds() * FRAC_PI_2 * direction);
    }
}

fn planet_switching(
    mut commands: Commands,
    mut query: Query<&mut Handle<EarthlikeMaterial>, With<Planet>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut materials: ResMut<Assets<EarthlikeMaterial>>,
) {
    let planet_mat: &Handle<EarthlikeMaterial> = query.single();

    let mut planet_mat = materials.get_mut(planet_mat).unwrap();

    if keyboard_input.just_pressed(KeyCode::Space) {
        planet_mat.seed = rand::thread_rng().gen::<f32>();
    }

    let mut direction = 0f32;

    if keyboard_input.pressed(KeyCode::Up) {
        direction += 1.;
    }

    if keyboard_input.pressed(KeyCode::Down) {
        direction -= 1.;
    }

    planet_mat.pixels += direction;
}

fn add_loaded_component(
    mut commands: Commands,
    query: Query<Entity, (Without<Loaded>, Without<Persist>)>,
) {
    for entity in &query {
        commands.entity(entity).insert(Loaded);
    }
}
