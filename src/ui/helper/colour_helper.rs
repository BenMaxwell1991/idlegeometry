use eframe::egui::Color32;

pub fn lighten_colour(colour: Color32, amount: i8) -> Color32 {
    fn adjust_channel(value: u8, amount: i8) -> u8 {
        ((value as i16 + amount as i16).clamp(0, 255)) as u8
    }

    Color32::from_rgb(
        adjust_channel(colour.r(), amount),
        adjust_channel(colour.g(), amount),
        adjust_channel(colour.b(), amount),
    )
}
