//! Core functionality for running a Bevy app on the 3DS. This plugin initializes
//! some basic 3DS functionality and handles the main application loop, including
//! [`AppExit`].

use bevy::app::AppExit;
use bevy::prelude::*;
use ctru::prelude::*;

#[derive(Default)]
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        let gfx = Gfx::new().expect("unable to init GFX");
        let apt = Apt::new().expect("failed to init APT");

        app.insert_non_send_resource(gfx)
            .insert_non_send_resource(apt)
            // Check APT and exit system before everything else
            // TODO: reread https://bevyengine.org/news/bevy-0-10/#ecs-schedule-v3
            // and make sure these are scheduled correctly
            .add_systems(First, exit_system)
            // run gfx flush after all other stages
            .add_systems(Last, flush_gfx);
    }
}

fn exit_system(apt: NonSend<Apt>, mut exit: EventWriter<AppExit>) {
    if !apt.main_loop() {
        exit.send(AppExit::Success);
    }
}

fn flush_gfx(gfx: NonSend<Gfx>) {
    // TODO: Add flushing
    // gfx.flush_buffers()
    // gfx.swap_buffers();
    gfx.wait_for_vblank();
}
