use egui::{Color32, Rect};
use glow::NativeTexture;

pub struct SpriteToDraw {
    pub texture: NativeTexture,
    pub rect: Rect,
    pub tint: Color32,
    pub blend_target: Color32,
    pub colour_blend_amount: f32,
    pub alpha_blend_amount: f32,
    pub rotation: f32,
}