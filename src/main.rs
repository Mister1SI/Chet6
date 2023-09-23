#![windows_subsystem = "windows"]

use iced::{Settings, Sandbox};
use chet6::Chet6;

fn main() {
    Chet6::run(Settings::default()).unwrap();
}