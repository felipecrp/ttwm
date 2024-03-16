
pub mod win;


/**
 * Return all windows
 */
pub fn get_all_windows() -> Vec<win::WindowHandler> {
    win::get_all_windows()
}
