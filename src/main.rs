// SPDX-FileCopyrightText: © 2026 Obed Kooijman <contact@obedkooijman.me>
// SPDX-License-Identifier: GPL-2.0-or-later

pub mod host;

use crate::host::discover;
use bevy_app::{App, Startup};
use bevy_log::{LogPlugin, info};

fn main() {
    App::new()
        .add_plugins(LogPlugin::default())
        .add_systems(Startup, initialize)
        .run();
}

fn initialize() {
    let host = discover();
    info!("Application initialized on {:?}", host);
}
