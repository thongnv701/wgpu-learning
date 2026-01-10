use std::sync::Arc;

#[cfg(target_arch = "wasm32")]
use winit::event_loop::{EventLoop, EventLoopProxy};
use winit::{
    application::ApplicationHandler, dpi::PhysicalPosition, event::{KeyEvent, WindowEvent}, event_loop::ActiveEventLoop, keyboard::PhysicalKey, window::{WindowAttributes, WindowId}
};

use crate::models::state::State;

pub struct App {
    #[cfg(target_arch = "wasm32")]
    proxy: Option<EventLoopProxy<State>>,
    state: Option<State>,
}

impl App {
    pub fn new(#[cfg(target_arch = "wasm32")] event_loop: &EventLoop<State>) -> Self {
        #[cfg(target_arch = "wasm32")]
        let proxy = Some(event_loop.create_proxy());
        Self {
            state: None,
            #[cfg(target_arch = "wasm32")]
            proxy,
        }
    }
}

impl ApplicationHandler<State> for App {
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        let state = match &mut self.state {
            Some(canvas) => canvas,
            None => return,
        };

        match event {
            // ...
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(code),
                        state: key_state,
                        ..
                    },
                ..
            } => state.handle_key(event_loop, code, key_state.is_pressed()),
            WindowEvent::RedrawRequested => {
                state.update();
                match state.render() {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        let size = state.window.inner_size();
                        state.resize(size.width, size.height);
                    }
                    Err(e) => {
                        log::error!("Update to render {}", e);
                    }
                }
            },
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(physical_size) => {
                state.resize(physical_size.width, physical_size.height);
            },
            WindowEvent::CursorMoved {
                position:  PhysicalPosition { x, y },
                ..
            } => state.handle_mouse_moved(x, y),
            _ => {}
        }
    }
    
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.state.is_none() {
            let window_attributes = WindowAttributes::default()
                .with_title("WGPU Application")
                .with_inner_size(winit::dpi::LogicalSize::new(800, 600));
            
            let window = Arc::new(
                event_loop
                    .create_window(window_attributes)
                    .expect("Failed to create window")
            );
            
            #[cfg(target_arch = "wasm32")]
            {
                use winit::platform::web::WindowExtWebSys;
                web_sys::window()
                    .and_then(|win| win.document())
                    .and_then(|doc| {
                        let dst = doc.get_element_by_id("wasm-example")?;
                        let canvas = web_sys::Element::from(window.canvas()?);
                        dst.append_child(&canvas).ok()?;
                        Some(())
                    })
                    .expect("Couldn't append canvas to document body.");
            }
            
            let state = pollster::block_on(State::new(window))
                .expect("Failed to create state");
            
            self.state = Some(state);
        }
    }
}
