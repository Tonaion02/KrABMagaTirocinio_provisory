use bevy::math::Vec2;
use bevy::prelude::{Camera2dBundle, Commands, Res, ResMut};
use bevy::prelude::{Query, With};
use bevy::window::{PrimaryWindow, Window};

use bevy::ecs::system::Resource;

use bevy::prelude::Transform;
use bevy::render::camera::OrthographicProjection;
use bevy::render::camera::ScalingMode;
use bevy::utils::default;
//T: commented for errors
//use crate::engine::state::State;
//T: commented for errors
// use crate::visualization::{
//     asset_handle_factory::AssetHandleFactoryResource,
//     simulation_descriptor::SimulationDescriptor,
//     visualization_state::VisualizationState,
//     wrappers::{ActiveSchedule, ActiveState},
// };

use bevy::prelude::AssetServer;
use bevy::sprite::SpriteBundle;

use crate::visualization::simulation_descriptor::{self, SimulationDescriptor};

// T: Commented to rewrite
/// The main startup system which bootstraps a simple orthographic camera, centers it to aim at the simulation,
/// then calls the user provided init callback.
// pub fn init_system<I: VisualizationState<S> + 'static + bevy::prelude::Resource, S: State>(
//     on_init: Res<I>,
//     mut sprite_factory: AssetHandleFactoryResource,
//     mut commands: Commands,
//     state_resource: ResMut<ActiveState<S>>,
//     schedule_resource: ResMut<ActiveSchedule>,
//     window: Query<&Window, With<PrimaryWindow>>,
//     mut sim: ResMut<SimulationDescriptor>,
// ) {
//     if let Ok(window) = window.get_single() {
//         // Right handed coordinate system, equal to how it is implemented in [`OrthographicProjection::new_2d()`].
//         let far = 1000.;
//         // Offset the whole simulation to the left to take the width of the UI panel into account.
//         let ui_offset = -sim.ui_width;
//         // Scale the simulation so it fills the portion of the screen not covered by the UI panel.
//         let scale_x = sim.width / (window.width() + ui_offset);
//         // The translation x must depend on the scale_x to keep the left offset constant between window resizes.
//         let mut initial_transform = Transform::from_xyz(ui_offset * scale_x, 0., far - 0.1);
//         initial_transform.scale.x = scale_x;
//         initial_transform.scale.y = sim.height / window.height();

//         commands.spawn(Camera2dBundle {
//             projection: OrthographicProjection {
//                 far,
//                 scaling_mode: ScalingMode::WindowSize(1.),
//                 viewport_origin: Vec2::new(0., 0.),
//                 ..default()
//             }
//             .into(),
//             transform: initial_transform,
//             ..default()
//         });

//         on_init.on_init(
//             &mut commands,
//             &mut sprite_factory,
//             &mut state_resource.0.lock().expect("error on lock"),
//             &mut schedule_resource.0.lock().expect("error on lock"),
//             &mut *sim,
//         );
//         on_init.setup_graphics(
//             &mut schedule_resource.0.lock().expect("error on lock"),
//             &mut commands,
//             &mut state_resource.0.lock().expect("error on lock"),
//             sprite_factory,
//         )
//     }
// }


//T: rewrited from me
pub fn init_system(
    mut commands: Commands,
    window: Query<&Window, With<PrimaryWindow>>,
    mut sim: ResMut<SimulationDescriptor>,
    asset_server: Res<AssetServer>,
) {
    if let Ok(window) = window.get_single() {

        println!("init_system is executed");

        // Right handed coordinate system, equal to how it is implemented in [`OrthographicProjection::new_2d()`].
        let far = 1000.;
        // Offset the whole simulation to the left to take the width of the UI panel into account.
        let ui_offset = -sim.ui_width;
        // Scale the simulation so it fills the portion of the screen not covered by the UI panel.
        let scale_x = sim.width / (window.width() + ui_offset);
        // The translation x must depend on the scale_x to keep the left offset constant between window resizes.
        let mut initial_transform = Transform::from_xyz(ui_offset * scale_x, 0., far - 0.1);
        initial_transform.scale.x = scale_x;
        initial_transform.scale.y = sim.height / window.height();

        //T: Spawn a necessary 2D Camera
        commands.spawn(Camera2dBundle {
            projection: OrthographicProjection {
                far,
                scaling_mode: ScalingMode::WindowSize(1.),
                viewport_origin: Vec2::new(0., 0.),
                ..default()
            }
            .into(),
            transform: initial_transform,
            ..default()
        });

        //T: find a method to substitute this call that permits to execute code to initialize
        //T: graphic components of simulation
        // on_init.on_init(
        //     &mut commands,
        //     &mut sprite_factory,
        //     &mut state_resource.0.lock().expect("error on lock"),
        //     &mut schedule_resource.0.lock().expect("error on lock"),
        //     &mut *sim,
        // );
        // on_init.setup_graphics(
        //     &mut schedule_resource.0.lock().expect("error on lock"),
        //     &mut commands,
        //     &mut state_resource.0.lock().expect("error on lock"),
        //     sprite_factory,
        // )
    }
}