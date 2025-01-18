#![doc = include_str!("../README.md")]

use bevy::app::{PluginGroup, PluginGroupBuilder};

pub mod core;
pub mod input;
pub mod log;

/// A default set of plugins to get an app up and running. This also includes
/// most (but not all) of the same plugins in [`bevy::DefaultPlugins`] for ease
/// of use.
#[derive(Default)]
pub struct DefaultPlugins;

impl PluginGroup for DefaultPlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>()
            // Add log plugin early so we can see what's going on

            // TODO: Get socket logging working
            // .add(log::SocketLogPlugin);
        ;

        group = group
            .add(bevy::log::LogPlugin::default())
            .add(bevy::core::TaskPoolPlugin::default())
            .add(bevy::core::TypeRegistrationPlugin)
            .add(bevy::core::FrameCountPlugin)
            .add(bevy::time::TimePlugin)
            .add(bevy::transform::TransformPlugin)
            .add(bevy::hierarchy::HierarchyPlugin)
            .add(bevy::diagnostic::DiagnosticsPlugin)
            .add(bevy::input::InputPlugin)
            .add(bevy::window::WindowPlugin::default())
            .add(bevy::a11y::AccessibilityPlugin)
            // TODO: Replace this with winit runner
            .add(bevy::app::ScheduleRunnerPlugin::default());

        // Default bevy_3ds plugins
        group = group.add(core::CorePlugin).add(input::InputPlugin);

        // TODO: feature-dependent plugins like render, gltf, audio, etc.

        group
    }
}
