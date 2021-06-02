mod help;
mod logs;

pub use help::Help;
pub use logs::Logs;

pub trait Widget<'a, W: tui::widgets::Widget> {
    fn draw(state: &'a crate::State) -> W;
}
