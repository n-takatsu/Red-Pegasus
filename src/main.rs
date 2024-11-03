use std::error::Error;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::Window;

fn main() -> Result<(), Box<dyn Error>> {
    let event_loop = EventLoop::builder().build()?;
    let mut app = Application::default();
    event_loop.run_app(&mut app)?;
    Ok(())
}

#[derive(Default)]
struct Application {
    window: Option<Window>,
}

impl ApplicationHandler for Application {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window_attributes = Window::default_attributes().with_title("Winit window");

        let window = event_loop
            .create_window(window_attributes)
            .expect("failed to create initial window");
        self.window = Some(window);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Destroyed => {
                // ...
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            event_loop.exit();
        }
    }
}