use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

fn redraw(buffer: &mut [u32], width: usize, height: usize, flag: bool) {
    for (index, color) in buffer.iter_mut().enumerate() {
        let y = index / width;
        let x = index % width;

        if flag && x >= 100 && x < width - 100 && y >= 100 && y < height - 100 {
            *color = 0x00ffffff;
        } else {
            let red = (x & 0xff) ^ (y & 0xff);
            let green = (x & 0x7f) ^ (y & 0x7f);
            let blue = (x & 0x3f) ^ (y & 0x3f);
            *color = (blue | (green << 8) | (red << 16)) as u32;
        }
    }
}

fn main() {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("Press space to show/hide a rectangle")
        .build(&event_loop)
        .unwrap();

    #[cfg(target_arch = "wasm32")]
    {
        use winit::platform::web::WindowExtWebSys;

        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .body()
            .unwrap()
            .append_child(&window.canvas())
            .unwrap();
    }

    let context = unsafe { softbuffer::Context::new(&window) }.unwrap();
    let mut surface = unsafe { softbuffer::Surface::new(&context, &window) }.unwrap();

    let mut buffer = Vec::new();
    let mut flag = false;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                // Grab the window's client area dimensions
                let (width, height) = {
                    let size = window.inner_size();
                    (size.width as usize, size.height as usize)
                };

                // Resize the off-screen buffer if the window size has changed
                if buffer.len() != width * height {
                    buffer.resize(width * height, 0);
                }

                // Draw something in the offscreen buffer
                redraw(&mut buffer, width, height, flag);

                // Blit the offscreen buffer to the window's client area
                surface.set_buffer(&buffer, width as u16, height as u16);
            }

            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => {
                *control_flow = ControlFlow::Exit;
            }

            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Space),
                                ..
                            },
                        ..
                    },
                window_id,
            } if window_id == window.id() => {
                // Flip the rectangle flag and request a redraw to show the changed image
                flag = !flag;
                window.request_redraw();
            }

            _ => {}
        }
    });
}