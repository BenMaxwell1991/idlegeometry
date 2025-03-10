use egui::Vec2;

trait Vec2Extensions {
    fn normalize_or_zero(self) -> Self;
}

impl Vec2Extensions for Vec2 {
    fn normalize_or_zero(self) -> Self {
        let length = self.length();
        if length == 0.0 {
            Vec2::ZERO
        } else {
            self / length
        }
    }
}
