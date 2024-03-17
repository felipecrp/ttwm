mod desktop;
mod workspace;
mod container;
mod layout;
mod window;
mod geometry;
mod api;

use crate::workspace::Workspace;
use api::win::WinWindowManager;
use desktop::Desktop;
use window::WindowManager;

fn main() {
    let desktop = Desktop::new();
}
