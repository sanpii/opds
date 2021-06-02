mod help;

pub use help::Help;

pub trait Widget<'a, W: tui::widgets::Widget> {
    fn draw(state: &'a crate::State) -> W;
}
