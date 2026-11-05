/*
 * SPDX-FileCopyrightText: © 2026 Obed Kooijman <contact@obedkooijman.me>
 * SPDX-License-Identifier: GPL-2.0-or-later
 */
use std::env::consts;

#[derive(Debug)]
pub struct Host {
    pub os: &'static str,
    pub arch: &'static str,
}

pub fn discover() -> Host {
    Host {
        os: consts::OS,
        arch: consts::ARCH,
    }
}
