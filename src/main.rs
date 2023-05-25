use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy::scene::{scene_spawner, scene_spawner_system};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<LoadingState>()
        .add_startup_system(setup)
        .add_system(check_asset_loading.in_set(OnUpdate(LoadingState::Unloaded)))
        .add_systems(
            (
                spawn,
                apply_system_buffers, scene_spawner, scene_spawner_system, // force scene to spawn in
                label,
            )
                .chain()
                .in_schedule(OnEnter(LoadingState::Loaded)),
        )
        .run();
}

#[derive(Resource)]
struct HelmetHandle(Handle<Gltf>);

#[derive(Component, Debug)]
struct SceneMarker;

impl Drop for SceneMarker {
    fn drop(&mut self) {
        println!("Dropping scene marker");
    }
}

#[derive(Component, Debug)]
struct NodeMarker;

impl Drop for NodeMarker {
    fn drop(&mut self) {
        println!("Dropping node marker");
    }
}

#[derive(Debug, States, Clone, Copy, PartialEq, Eq, Hash, Default)]
enum LoadingState {
    #[default]
    Unloaded,
    Loaded,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let robot_handle: Handle<Gltf> = asset_server.load("models/FlightHelmet.gltf");
    commands.insert_resource(HelmetHandle(robot_handle));

    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(10.0, 10.0, 15.0)
            .looking_at(Vec3::new(0.0, 2.0, 0.0), Vec3::Y),
        ..default()
    });
}

fn check_asset_loading(
    mut commands: Commands,
    server: Res<AssetServer>,
    handle: Res<HelmetHandle>,
) {
    match server.get_load_state(handle.0.id()) {
        bevy::asset::LoadState::Loaded => {
            println!("All assets loaded!");
            commands.insert_resource(NextState(Some(LoadingState::Loaded)))
        }
        bevy::asset::LoadState::Failed => panic!("Unable to load all assets"),
        _ => {}
    }
}

fn spawn(mut commands: Commands, helmet: Res<HelmetHandle>, gltf_assets: Res<Assets<Gltf>>) {
    commands.spawn((
        SceneBundle {
            scene: gltf_assets
                .get(&helmet.0)
                .expect("Expected to find gltf asset")
                .default_scene
                .clone()
                .expect("Expected gltf asset to have default scene"),
            ..Default::default()
        },
        SceneMarker,
    ));
    println!("Requested to spawn scene");
}

fn label(mut commands: Commands, named_nodes: Query<(Entity, &Name)>) {
    println!("Labeling {} nodes", named_nodes.iter().len());
    for (entity, name) in named_nodes.iter() {
        println!("{entity:?}: {}", name.as_str()); 
        match name.as_str() {
            "RubberWood_low" => {
                println!("Found node: {entity:?}");
                commands.entity(entity).insert(NodeMarker);
            }
            _ => {}
        }
    }
}
