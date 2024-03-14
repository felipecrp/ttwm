mod win;

use crate::container;

/**
 * Return all windows
 */
pub fn get_all_windows() -> Vec<container::WindowContainer> {
    win::get_all_windows()
}
