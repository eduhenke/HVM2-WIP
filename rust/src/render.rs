extern crate sdl2;

use crate::core::*;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;
use sdl2::{event::Event, surface::Surface};
use std::time::{Duration, Instant};

const IMAGE_SIZE: usize = 256;
const DEPTH: u16 = 16;

pub fn net_bin_tree_to_image(net: &mut Net, image: &mut [u8; IMAGE_SIZE * IMAGE_SIZE * 3]) {
    #[derive(Clone, Copy)]
    enum VisitedStatus {
        Unvisited,
        VisitedLeft,
        VisitedBoth,
    }
    use VisitedStatus::*;

    #[derive(Clone)]
    struct StackEntry {
        node: Ptr,
        visited: VisitedStatus,
        depth: u16,
    }

    let mut image_i = 0;

    let mut stack: Vec<StackEntry> = Vec::new();
    stack.push(StackEntry {
        node: net.root,
        visited: Unvisited,
        depth: 0,
    });

    while !stack.is_empty() {
        let (depth, visited, node) = {
            let entry = stack.last_mut().unwrap();
            (entry.depth, &mut entry.visited, entry.node)
        };

        if depth == DEPTH {
            if net.target(Ptr::new(VR1, node.val())).unwrap().is_era() {
                image[image_i..image_i + 3].copy_from_slice(&[0x00, 0x00, 0x00]);
            } else {
                image[image_i..image_i + 3].copy_from_slice(&[0xff, 0xff, 0xff]);
            }
            image_i += 3;
            stack.pop();
        } else {
            match visited {
                Unvisited => {
                    let left = *net.target(Ptr::new(VR1, node.val())).unwrap();
                    *visited = VisitedLeft;
                    stack.push(StackEntry {
                        node: left,
                        visited: Unvisited,
                        depth: depth + 1,
                    });
                }
                VisitedLeft => {
                    let right = *net.target(Ptr::new(VR2, node.val())).unwrap();
                    *visited = VisitedBoth;
                    stack.push(StackEntry {
                        node: right,
                        visited: Unvisited,
                        depth: depth + 1,
                    });
                }
                VisitedBoth => {
                    stack.pop();
                }
            }
        }
    }
}

pub fn net_list_to_image(net: &mut Net, image: &mut [u8; IMAGE_SIZE * IMAGE_SIZE * 3]) {
    fn left(net: &mut Net, node: Ptr) -> Ptr {
        *net.target(Ptr::new(VR1, node.val())).unwrap()
    }
    fn right(net: &mut Net, node: Ptr) -> Ptr {
        *net.target(Ptr::new(VR2, node.val())).unwrap()
    }

    // For each row
    let mut rows = net.root;
    for y in 0..IMAGE_SIZE {
        let current_row = left(net, rows);

        // For each cell in the row
        let mut cells = current_row;
        for x in 0..IMAGE_SIZE {
            let idx = (y * IMAGE_SIZE + x) * 3;
            let current_cell = left(net, cells);

            let is_true = !left(net, current_cell).is_era();

            image[idx..idx + 3].copy_from_slice(if is_true {
                &[0xff, 0xff, 0xff]
            } else {
                &[0x00, 0x00, 0x00]
            });

            cells = right(net, cells);
        }

        rows = right(net, rows);
    }
}

pub fn setup_window() -> (EventPump, Canvas<Window>) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("hvm demo", IMAGE_SIZE as u32, IMAGE_SIZE as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let event_pump = sdl_context.event_pump().unwrap();

    (event_pump, canvas)
}

pub fn render_image(canvas: &mut Canvas<Window>, image: &mut [u8; IMAGE_SIZE * IMAGE_SIZE * 3]) {
    fn image_to_texture<'a>(
        texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
        image: &mut [u8; IMAGE_SIZE * IMAGE_SIZE * 3],
    ) -> sdl2::render::Texture<'a> {
        let text = Surface::from_data(
            image,
            IMAGE_SIZE as u32,
            IMAGE_SIZE as u32,
            (IMAGE_SIZE as u32) * 3,
            PixelFormatEnum::RGB24,
        )
        .unwrap()
        .as_texture(texture_creator)
        .unwrap();
        text
    }

    let text_creator = canvas.texture_creator();

    let texture = image_to_texture(&text_creator, image);
    canvas.copy(&texture, None, None).unwrap();

    canvas.present();
}
