use gpui::{AppContext, Entity, IntoElement, ParentElement, Render, Styled, div, px};
use gpui_component::{ActiveTheme, h_resizable, resizable_panel};

pub mod left_sidebar;
use left_sidebar::LeftSidebar;

pub struct Layout {
    pub left_sidebar: Entity<LeftSidebar>,
}

impl Layout {
    pub fn new(window: &mut gpui::Window, cx: &mut gpui::Context<Self>) -> Self {
        let left_sidebar = cx.new(|cx| left_sidebar::LeftSidebar::new(window, cx));
        Self { left_sidebar }
    }
}

impl Render for Layout {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        div().bg(cx.theme().background).size_full().child(
            h_resizable("root-layout")
                .child(
                    resizable_panel()
                        .size(px(250.))
                        .size_range(px(250.)..px(400.))
                        .child(self.left_sidebar.clone().into_any_element()),
                )
                .child(div().child("right").into_any_element()),
        )
    }
}
