use bevy::{app::ScheduleRunnerPlugin, log::LogPlugin, prelude::*};

mod protocol;
mod server;

#[cfg(not(feature = "dedicated-server"))]
mod client;

use modcraft_lib::add;

fn main() {
    info!("Welcome to modcraft (now different)");

    #[cfg(feature = "dedicated-server")]
    info!("Compiled as dedicated server");

    #[cfg(not(feature = "dedicated-server"))]
    info!("Compiled as client");

    info!("Modcraft lib is working: {} + {} = {}", 2, 2, add(2, 2));

    let mut app = App::new();
    app.add_plugins((ScheduleRunnerPlugin::default(), LogPlugin::default()));

    #[cfg(feature = "dedicated-server")]
    server::setup_app(&mut app);

    #[cfg(not(feature = "dedicated-server"))]
    client::setup_app(&mut app);

    app.run();
}
