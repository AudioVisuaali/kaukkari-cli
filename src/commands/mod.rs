pub mod apple_tv;
pub mod base;
pub mod chromecast;
pub mod disco;
pub mod fridge;
pub mod help;
pub mod off;
pub mod on;
pub mod starry_sky;
pub mod volume;
pub mod volume_down;
pub mod volume_mute;
pub mod volume_up;

use base::Command;

use crate::commands::{
    apple_tv::details_apple_tv, chromecast::details_chromecast, disco::details_disco,
    fridge::details_fridge, help::details_help, off::details_off, on::details_on,
    starry_sky::details_starry_sky, volume::details_volume, volume_down::details_volume_down,
    volume_mute::details_volume_mute, volume_up::details_volume_up,
};

pub fn generate_details() -> Vec<Command> {
    vec![
        details_help(),
        details_on(),
        details_off(),
        details_apple_tv(),
        details_chromecast(),
        details_volume(),
        details_volume_mute(),
        details_volume_up(),
        details_volume_down(),
        details_fridge(),
        details_starry_sky(),
        details_disco(),
    ]
}
