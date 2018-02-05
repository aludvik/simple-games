use io::window::Window;

pub trait Scene {
    // Called periodically to update the scene
    fn update(&mut self);

    // Called to draw the scene to a window
    fn draw(&self, window: &Window);
}
