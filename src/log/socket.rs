//! A plugin to enable console logging back from the 3DS back to the development device.
//! Once initialized, the usual [`bevy::log`] macros can be used to instrument the app.

use bevy::app::prelude::*;
use bevy::ecs::prelude::*;
use ctru::services::soc::Soc;

#[derive(Default)]
pub struct SocketLogPlugin;

struct SocketLogger(Soc);

impl Default for SocketLogger {
    fn default() -> Self {
        let soc = Soc::new().expect("failed to init SOC");
        Self(soc)
    }
}

impl Plugin for SocketLogPlugin {
    fn build(&self, app: &mut App) {
        app.init_non_send_resource::<SocketLogger>()
            .add_systems(Startup, log_to_3dslink);
    }
}

fn log_to_3dslink(mut soc: NonSendMut<SocketLogger>) {
    // TODO: should this ignore failures? Or perhaps configurable behavior?
    soc.0
        .redirect_to_3dslink(true, true)
        .expect("unable to debug output to 3dslink");
}
