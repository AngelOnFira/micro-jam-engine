use input::Input;
use std::rc::Rc;
use std::sync::RwLock;
use std::{marker::PhantomData, time::Instant};
use wasm_bindgen::prelude::*;
use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

pub use vek;

use vek::*;

mod input;
mod utils;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, micro-jam!");
}

pub trait Game: Sized + 'static {
    const TITLE: &'static str;
    type SaveData: Default;

    fn init(console: &mut Console<Self>) -> Self;

    fn tick(&mut self, dt: f32, console: &mut Console<Self>);

    fn run() {
        run_with::<Self>()
    }
}

pub struct Console<'tick, G: Game> {
    pub input: Rc<RwLock<Input>>,
    pub graphics: Graphics<'tick>,
    pub audio: Audio,
    pub save: Save<G::SaveData>,
}

pub struct Graphics<'tick> {
    pub size: Vec2<usize>,
    pub framebuffer: &'tick mut [u32],
}

impl<'tick> Graphics<'tick> {
    // TODO: Methods for drawing shapes, sprites, perhaps even triangles, as well as getting access to the framebuffer
}

pub struct Audio;

impl Audio {
    //pub fn play(&mut self, sound: Sound) { todo!() }
}

pub struct Save<S> {
    phantom: PhantomData<S>,
}

impl<S> Save<S> {
    pub fn read(&mut self) -> S {
        todo!()
    }
    pub fn write(&mut self, save: S) {
        todo!()
    }
}

fn run_with<G: Game>() {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title(G::TITLE)
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

    let window_size = window.inner_size();
    let mut framebuffer = vec![0; window_size.width as usize * window_size.height as usize];
    let mut flag = false;

    let mut time = instant::Instant::now();

    let game_input = Rc::new(RwLock::new(Input {
        pressed_keys: [false; 256],
    }));

    let game_input_clone = game_input.clone();

    let mut game = G::init(&mut Console {
        input: game_input_clone,
        graphics: Graphics {
            size: Vec2::new(window_size.width as usize, window_size.height as usize),
            framebuffer: &mut framebuffer,
        },
        audio: Audio,
        save: Save {
            phantom: PhantomData,
        },
    });

    let game_input_clone = game_input.clone();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                // Grab the window's client area dimensions
                let (width, height) = {
                    let size = window.inner_size();
                    (size.width as usize, size.height as usize)
                };

                // Resize the off-screen buffer if the window size has changed
                if framebuffer.len() != width * height {
                    framebuffer.resize(width * height, 0);
                }

                // Blit the offscreen buffer to the window's client area
                surface.set_buffer(&framebuffer, width as u16, height as u16);
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => {
                *control_flow = ControlFlow::Exit;
            }
            // Event::WindowEvent {
            //     event:
            //         WindowEvent::KeyboardInput {
            //             input:
            //                 KeyboardInput {
            //                     state: ElementState::Pressed,
            //                     virtual_keycode: Some(VirtualKeyCode::Space),
            //                     ..
            //                 },
            //             ..
            //         },
            //     window_id,
            // } if window_id == window.id() => {
            //     // Flip the rectangle flag and request a redraw to show the changed image
            //     flag = !flag;
            //     window.request_redraw();
            // }
            /// Handle tracking of which keys are pressed
            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        input:
                            winit::event::KeyboardInput {
                                virtual_keycode: Some(keycode),
                                state,
                                ..
                            },
                        ..
                    },
                ..
            } => match state {
                winit::event::ElementState::Pressed => {
                    game_input.write().unwrap().pressed_keys[keycode as usize] = true;
                }
                winit::event::ElementState::Released => {
                    game_input.write().unwrap().pressed_keys[keycode as usize] = false;
                }
            },
            Event::MainEventsCleared => {
                let new_time = instant::Instant::now();

                game.tick(
                    new_time.duration_since(time).as_secs_f32(),
                    &mut Console {
                        input: game_input_clone,
                        graphics: Graphics {
                            size: {
                                let sz = window.inner_size();
                                Vec2::new(sz.width as usize, sz.height as usize)
                            },
                            framebuffer: &mut framebuffer,
                        },
                        audio: Audio,
                        save: Save {
                            phantom: PhantomData,
                        },
                    },
                );

                window.request_redraw();

                time = new_time;
            }
            _ => {}
        }
    });
}
