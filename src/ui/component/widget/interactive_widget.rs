use crate::ui::helper::colour_helper::lighten_colour;
use eframe::egui::{Color32, Response, Ui};

pub trait InteractiveWidget {
    fn button_clicked(&self, ui: &mut Ui, response: &Response) -> bool {
        if !ui.is_enabled() {
            return false;
        }
        let button_id = response.id;
        let mut click_started_inside = ui.data(|data| data.get_temp::<bool>(button_id)).unwrap_or(false);
        let is_mouse_down = response.ctx.input(|i| i.pointer.primary_down());

        if is_mouse_down {
            let press_origin = response.ctx.input(|i| i.pointer.press_origin());
            click_started_inside = press_origin.map_or(false, |pos| response.rect.contains(pos));
            ui.data_mut(|data| data.insert_temp(button_id, click_started_inside));
        }

        is_mouse_down && click_started_inside
    }

    fn released_inside(&self, ui: &mut Ui, response: &Response) -> bool {
        if !ui.is_enabled() {
            return false;
        }
        let button_id = response.id;
        let click_started_inside = ui.data(|data| data.get_temp::<bool>(button_id)).unwrap_or(false);
        let is_mouse_up = response.ctx.input(|i| i.pointer.primary_released());
        let release_inside = response.contains_pointer();

        is_mouse_up && click_started_inside && release_inside
    }

    fn determine_colour(
        &self,
        ui: &mut Ui,
        base_colours: Vec<Color32>,
        button_clicked: bool,
        hovering: bool,
    ) -> Vec<Color32> {
        if !ui.is_enabled() {
            return base_colours;
        }
        let click_colours = self.click_colours();
        let hover_colours = self.hover_colours();

        assert_eq!(base_colours.len(), click_colours.len());
        assert_eq!(base_colours.len(), hover_colours.len());

        base_colours
            .into_iter()
            .enumerate()
            .map(|(i, color)| {
                if button_clicked {
                    lighten_colour(color, click_colours[i])
                } else if hovering {
                    lighten_colour(color, hover_colours[i])
                } else {
                    color
                }
            })
            .collect()
    }

    fn click_colours(&self) -> Vec<i8> {
        vec![-20, -25, -30]
    }

    fn hover_colours(&self) -> Vec<i8> {
        vec![25, 30, 35]
    }

}
