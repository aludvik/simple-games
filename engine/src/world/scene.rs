use io::window::Window;
use io::keys::Key;

pub trait Scene {
    // Called periodically to update the scene
    fn update(&mut self) {}

    // Called to draw the scene to a window
    fn draw(&self, window: &Window) {}

    // Called when a key is pressed (when this scene is active)
    fn on_pressed(&mut self, key: Key) {}

    // Called when this scene gains focus
    fn on_focus(&mut self) {}

    // Called when this scene loses focus
    fn on_unfocus(&mut self) {}
}
