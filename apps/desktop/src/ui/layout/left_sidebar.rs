use gpui::{AnyElement, App, IntoElement, ParentElement, RenderOnce, Styled, Window, div};
use gpui_component::ActiveTheme;

#[derive(Default, IntoElement)]
pub struct LeftSidebar {
    items: Vec<AnyElement>,
}

impl LeftSidebar {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }
}

impl ParentElement for LeftSidebar {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.items.extend(elements);
    }
}

impl RenderOnce for LeftSidebar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        div().flex_1().bg(cx.theme().secondary).children(self.items)
    }
}
