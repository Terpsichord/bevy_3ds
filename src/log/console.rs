// use std::cell::RefMut;
// use std::rc::Rc;
// use bevy::prelude::*;
// use ctru::{console::{Console, ConsoleScreen}, services::gfx::Gfx};
//
// #[derive(Default)]
// pub struct ConsoleLogPlugin;
//
// #[self_referencing]
// struct ConsoleLogger {
//     gfx: Gfx,
//     #[borrows(mut gfx)]
//     #[covariant]
//     console: Console<'this>,
// }
//
// impl ConsoleLogger {
//     // Takes ownership of Gfx to prevent `bottom_screen` being accessed elsewhere
//     fn new(gfx: Gfx) -> Self {
//         Self {
//             console_builder: |gfx: &mut gfx| Console::new(gfx.bottom_screen.borrow_mut()),
//             gfx,
//         }
//     }
// }
//
// impl Plugin for ConsoleLogPlugin {
//     fn build(&self, app: &mut App) {
//         let gfx = app.world_mut().remove_non_send_resource::<Gfx>().expect("ConsoleLogPlugin requires the GFX resource");
//         let console = ConsoleLogger::new(gfx);
//         // app.insert_non_send_resource(console);
//     }
// }
