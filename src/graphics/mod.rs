//! This is the `graphics` module which provide basic drawing functionality making use of the PSP's sceGu
//! that proved the best performance.

pub(crate) const PI: f32 = 3.1415926536;

#[allow(dead_code)]
/// This modules describes and gives access to the PSP screen (width: `480`, height: `272`).
pub mod canvas;
mod utils;
pub use crate::graphics::canvas::Canvas;
#[allow(dead_code)]
/// This module defines some preset colors for ease of use and allows the user to manually input a color (`R,G,B,A`).
pub mod colors;
pub use crate::graphics::colors::{Color, Colors};
#[allow(dead_code)]
/// This module defines basic primitives (Rect, Triangle, Ellipse) to allow it to be drawn easily.
pub mod primitives;
pub use crate::graphics::primitives::Primitive;
#[allow(dead_code)]
/// This module defines a 2D sprite, based on a 16-bit aligned image with matching W and H.
pub mod sprite;
pub use crate::graphics::sprite::Sprite;

use crate::core::{Vec2, Vec3};

/// The `Drawable` trait is implemented to make somewhat uniform the experience of handling objects drawn on screen.
pub trait Drawable {
    fn draw(&mut self);

    fn get_size(&mut self) -> Vec2<f32>;
    fn set_size(&mut self, new_size: Vec2<f32>);

    fn get_scale(&mut self) -> Vec2<f32>;
    fn set_scale(&mut self, new_scale: Vec2<f32>);

    fn get_pos(&mut self) -> Vec3<f32>;
    fn set_pos(&mut self, new_position: Vec3<f32>);

    fn get_rot(&mut self) -> f32;
    fn set_rot(&mut self, new_rotation: f32);
}

/// Defines a vertex used by the drawable functions.
#[repr(C, align(4))]
#[derive(Default, Clone, Copy, Debug)]
pub struct Vertex {
    u: f32,
    v: f32,
    color: u32,
    x: f32,
    y: f32,
    z: f32,
}
