mod model;
mod service;
mod storage;
mod ui;

fn main() {
    let _ = ui::tui::run_terminal();
}
