mod desktop;
mod workspace;
mod container;
mod layout;
mod window;
mod window_event;
mod geometry;
mod api;

use std::{thread, time::Duration};

use desktop::Desktop;


fn main() {
    let mut desktop = Desktop::new();
    desktop.init();

    loop {
        thread::sleep(Duration::from_millis(1000));
    }
}

