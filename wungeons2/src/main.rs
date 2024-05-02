use glyphon::{
    Color, FontSystem, Resolution, SwashCache, TextArea, TextAtlas, TextBounds, TextRenderer,
};
use wgpu::{
    CommandEncoderDescriptor, CompositeAlphaMode, DeviceDescriptor, Features, Instance,
    InstanceDescriptor, Limits, LoadOp, MultisampleState, Operations, PresentMode,
    RenderPassColorAttachment, RenderPassDescriptor, RequestAdapterOptions, SurfaceConfiguration,
    TextureFormat, TextureUsages, TextureViewDescriptor,
};
use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, KeyEvent, WindowEvent},
    event_loop::EventLoop,
    keyboard::PhysicalKey,
    window::WindowBuilder,
};

use std::sync::Arc;

use game_loop::game_loop;

const TILE_WIDTH: isize = 18;
const TILE_HEIGHT: isize = 30;


use std::time::SystemTime;

// use device_query::{DeviceQuery, Keycode};

// use event::Event;

// use rooms::{create_floor, create_item};
// use wurdle::{play, wurdle_words};

use crate::{
    components::{Component, Rect},
    inputs::handle_inputs,
    state::State,
    systems::get_systems,
};

mod components;
mod create;
mod dialogue;
mod effects;
mod entity;
mod event;
mod inputs;
mod items;
mod render;
mod rooms;
mod sight;
mod state;
mod systems;
mod colors;

use crate::render::render;


fn main() {
    pollster::block_on(run());
}


async fn run() {
    let event_loop = EventLoop::new().unwrap();
    let (width, height) = (800, 600);
    let window = Arc::new(
        WindowBuilder::new()
            .with_inner_size(LogicalSize::new(width as f64, height as f64))
            .with_title("glyphon hello world")
            // .with_resizable(false)
            .build(&event_loop)
            .unwrap(),
    );
    let size = window.inner_size();
    let scale_factor = window.scale_factor();

    // Set up surface
    let instance = Instance::new(InstanceDescriptor::default());
    let adapter = instance
        .request_adapter(&RequestAdapterOptions::default())
        .await
        .unwrap();
    let (device, queue) = adapter
        .request_device(
            &DeviceDescriptor {
                label: None,
                required_features: Features::empty(),
                required_limits: Limits::downlevel_defaults(),
            },
            None,
        )
        .await
        .unwrap();

    let surface = instance
        .create_surface(window.clone())
        .expect("Create surface");
    let swapchain_format = TextureFormat::Bgra8UnormSrgb;
    let config = SurfaceConfiguration {
        usage: TextureUsages::RENDER_ATTACHMENT,
        format: swapchain_format,
        width: size.width,
        height: size.height,
        present_mode: PresentMode::Fifo,
        alpha_mode: CompositeAlphaMode::Opaque,
        view_formats: vec![],
        desired_maximum_frame_latency: 2,
    };
    surface.configure(&device, &config);

    let mut font_system = FontSystem::new();
    let mut cache = SwashCache::new();
    let mut atlas = TextAtlas::new(&device, &queue, swapchain_format);
    let mut text_renderer =
        TextRenderer::new(&mut atlas, &device, MultisampleState::default(), None);

    let game = Game::new();
    let mut buffers = vec![];
    let mut to_remove = vec![];

    game_loop(
        event_loop,
        window.clone(),
        game,
        60,
        0.1,
        move |g| {
            let start = SystemTime::now();

            for (i, (system, components, single_shot)) in &mut g.game.systems.iter().enumerate() {
                system(&mut g.game.state, components);
                if *single_shot {
                    to_remove.push(i);
                }
            }

            for i in to_remove.iter().rev() {
                g.game.systems.remove(*i);
                g.game.state.system_components.remove(*i);
            }
            to_remove.clear();

            g.game.state.update_loop_duration = SystemTime::now().duration_since(start).unwrap();
        },
        move |g| {
            let start = SystemTime::now();

            buffers.clear();
            render(
                width,
                height,
                &mut font_system,
                scale_factor,
                &mut g.game.state,
                &[
                    Component::Position(None),
                    Component::Render(None),
                    Component::ZIndex(None),
                ],
                &mut buffers,
            );
            buffers.sort_by(|b1, b2| b1.4.partial_cmp(&b2.4).unwrap());
            let text_areas = buffers
                .iter_mut()
                .map(|(buffer, position, color, shift, Z_height)| {
                    // position.x += 1;
                    TextArea {
                        buffer,
                        left: (position.x * TILE_WIDTH) as f32,
                        top: (position.y * TILE_HEIGHT) as f32 + *shift,
                        scale: 1.0,
                        bounds: {
                            let mut bounds = TextBounds {
                                left: ((position.x - 1) * TILE_WIDTH) as i32,
                                top: ((position.y - 1) * TILE_HEIGHT) as i32,
                                right: ((position.x + TILE_WIDTH + 1) * TILE_WIDTH) as i32,
                                bottom: ((position.y + TILE_HEIGHT + 1) * TILE_HEIGHT) as i32,
                            };

                            // if *shift_up {
                            //     // bounds.top -= TILE_HEIGHT  / ;
                            // }

                            bounds
                        },

                        default_color: Color::rgba(color.0, color.1, color.2, color.3),
                    }
                });

            text_renderer
                .prepare(
                    &device,
                    &queue,
                    &mut font_system,
                    &mut atlas,
                    Resolution {
                        width: config.width,
                        height: config.height,
                    },
                    text_areas,
                    &mut cache,
                )
                .unwrap();

            // let shape_renderer = ShapeRenderer::new(&device, &config);

            let frame = surface.get_current_texture().unwrap();
            let view = frame.texture.create_view(&TextureViewDescriptor::default());
            let mut encoder =
                device.create_command_encoder(&CommandEncoderDescriptor { label: None });
            {
                let mut pass = encoder.begin_render_pass(&RenderPassDescriptor {
                    label: None,
                    color_attachments: &[Some(RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: Operations {
                            load: LoadOp::Clear(wgpu::Color::BLACK),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });

                // for t in text_areas {
                //     shape_renderer
                // }
                text_renderer.render(&atlas, &mut pass).unwrap();
            }

            queue.submit(Some(encoder.finish()));
            frame.present();

            atlas.trim();

            g.game.state.render_loop_duration = SystemTime::now().duration_since(start).unwrap();

            g.game.num_renders += 1;
            window.set_title(&format!(
                "num_updates: {}, num_renders: {}, update_duration: {}ms, render_duration: {}ms",
                g.game.num_updates,
                g.game.num_renders,
                g.game.state.update_loop_duration.as_millis(),
                g.game.state.render_loop_duration.as_millis()
            ));

            // g.game.your_render_function(&g.window);
        },
        |g, event| match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::KeyboardInput {
                    device_id: _,
                    event,
                    is_synthetic: _,
                } => {
                    match event {
                        KeyEvent {
                            physical_key,
                            text,
                            state,
                            repeat,
                            // repeat: false,
                            ..
                        } => match (physical_key, state) {
                            (PhysicalKey::Code(code), ElementState::Pressed) => {
                                // g.window.request_redraw();
                                handle_inputs(
                                    &mut g.game.state,
                                    &[Component::Player],
                                    Some(*code),
                                    *repeat,
                                    text.clone(),
                                )
                            }
                            (_, _) => {}
                        },
                        _ => {}
                    }
                    // buffer.set_text(
                    //     &mut font_system,
                    //     &
                    // g.game.text = format!(
                    //     "{}{}",
                    //     g.game.text,
                    //     if event.text.is_some() {
                    //         event.text.clone().unwrap().to_string()
                    //     } else {
                    //         "".to_string()
                    //     }
                    // );
                    //     Attrs::new().family(Family::SansSerif),
                    //     Shaping::Advanced,
                    // );
                    // window.request_redraw();
                }
                WindowEvent::Resized(_size) => {
                    // config.width = size.width;
                    // config.height = size.height;
                    // surface.configure(&device, &config);
                    // window.request_redraw();
                }
                WindowEvent::RedrawRequested => {
                    // text_renderer
                    //     .prepare(
                    //         &device,
                    //         &queue,
                    //         &mut font_system,
                    //         &mut atlas,
                    //         Resolution {
                    //             width: config.width,
                    //             height: config.height,
                    //         },
                    //         [TextArea {
                    //             buffer: &buffer,
                    //             left: 10.0,
                    //             top: 10.0,
                    //             scale: 1.0,
                    //             bounds: TextBounds {
                    //                 left: 0,
                    //                 top: 0,
                    //                 right: 600,
                    //                 bottom: 160,
                    //             },
                    //             default_color: Color::rgb(255, 255, 255),
                    //         }],
                    //         &mut cache,
                    //     )
                    //     .unwrap();

                    // let frame = surface.get_current_texture().unwrap();
                    // let view = frame.texture.create_view(&TextureViewDescriptor::default());
                    // let mut encoder =
                    //     device.create_command_encoder(&CommandEncoderDescriptor { label: None });
                    // {
                    //     let mut pass = encoder.begin_render_pass(&RenderPassDescriptor {
                    //         label: None,
                    //         color_attachments: &[Some(RenderPassColorAttachment {
                    //             view: &view,
                    //             resolve_target: None,
                    //             ops: Operations {
                    //                 load: LoadOp::Clear(wgpu::Color::BLACK),
                    //                 store: wgpu::StoreOp::Store,
                    //             },
                    //         })],
                    //         depth_stencil_attachment: None,
                    //         timestamp_writes: None,
                    //         occlusion_query_set: None,
                    //     });

                    //     text_renderer.render(&atlas, &mut pass).unwrap();
                    // }

                    // queue.submit(Some(encoder.finish()));
                    // frame.present();

                    // atlas.trim();
                }
                WindowEvent::CloseRequested => g.exit(),
                _ => {}
            },
            _ => {}
        },
    )
    .unwrap();
}

// #[derive(Default)]
struct Game {
    pub text: String,
    pub num_updates: u32,
    pub num_renders: u32,
    pub state: State,
    pub systems: Vec<(fn(&mut State, &[Component]), Vec<Component>, bool)>,
}

impl Game {
    pub fn new() -> Self {
        const GRID_SIZE: Rect = Rect {
            width: 70,
            height: 3,
        };

        let systems = get_systems();
        let state = State::new(
            GRID_SIZE,
            systems
                .iter()
                .map(|(_, components, _)| components.clone())
                .collect::<Vec<Vec<Component>>>(),
        );

        Self {
            text: "".into(),
            num_updates: 0,
            num_renders: 0,
            state,
            systems,
        }
    }

    pub fn your_update_function(&mut self) {
        self.num_updates += 1;
    }

    // pub fn your_render_function(&mut self, window: &Window) {
    //     self.num_renders += 1;
    //     window.set_title(&format!(
    //         "num_updates: {}, num_renders: {}",
    //         self.num_updates, self.num_renders
    //     ));
    // }
}


// fn main() {
//     env::set_var("RUST_BACKTRACE", "1");

//     const GRID_SIZE: Rect = Rect {
//         width: 70,
//         height: 3,
//     };

//     let mut systems = get_systems();
//     let mut state = State::new(
//         GRID_SIZE,
//         systems
//             .iter()
//             .map(|(_, components, _)| components.clone())
//             .collect::<Vec<Vec<Component>>>(),
//     );

//     let mut to_remove = vec![];
//     loop {
//         let start = SystemTime::now();

//         for (i, (system, components, single_shot)) in &mut systems.iter().enumerate() {
//             system(&mut state, components);
//             if *single_shot {
//                 to_remove.push(i);
//             }
//         }

//         for i in to_remove.iter().rev() {
//             systems.remove(*i);
//             state.system_components.remove(*i);
//         }
//         to_remove.clear();

//         state.full_loop_duration = Some(SystemTime::now().duration_since(start).unwrap());

//         if state.full_loop_duration.unwrap() < Duration::from_millis(16) {
//             sleep(Duration::from_millis(16) - state.full_loop_duration.unwrap());
//         }
//     }
// }
