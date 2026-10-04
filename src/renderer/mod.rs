use gpui::{Context, Window, div, prelude::*};

pub struct TerminalView {
    pub frame: usize,
}

impl Render for TerminalView {
    fn render(&mut self, window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let size = window.viewport_size();

        let width: f32 = size.width.into();
        let height: f32 = size.height.into();

        let font_size = gpui::px(13.0);
        let font_id = window.text_system().resolve_font(&gpui::font("Menlo"));

        let row_height = 16.0;
        let cell_width: f32 = window
            .text_system()
            .ch_advance(font_id, font_size)
            .expect("could not measure font")
            .into();

        let cols = (width / cell_width).floor().max(1.0) as usize;
        let rows = (height / row_height).floor().max(1.0) as usize;

        let active_row = self.frame % rows;
        self.frame += 1;
        window.request_animation_frame();

        div()
            .font_family("Menlo")
            .text_size(gpui::px(13.0))
            .size_full()
            .bg(gpui::black())
            .text_color(gpui::white())
            .flex()
            .flex_col()
            .overflow_hidden()
            .children((0..rows).map(|r| {
                if r == active_row {
                    div().h(gpui::px(row_height)).child("░".repeat(cols))
                } else {
                    div().h(gpui::px(row_height)).child("█".repeat(cols))
                }
            }))
    }
}
