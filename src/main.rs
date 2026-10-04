use gpui::{App, AppContext, Application, WindowOptions};

use crate::renderer::TerminalView;

mod renderer;

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.open_window(WindowOptions::default(), |_, cx| {
            cx.new(|_| TerminalView { frame: 0 })
        })
        .expect("failed to open window");
        cx.activate(true);
    });
}
