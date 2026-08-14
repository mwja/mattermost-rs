use gpui::{AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window, div};
use gpui_component::ActiveTheme;
use mattermost::client::channel::ChannelType;

use crate::ui::channel::ChannelList;

#[derive(Debug, Clone)]
pub struct LeftSidebar {
    channel_list: Entity<ChannelList>,
}

impl LeftSidebar {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let channel_list = cx.new(|cx| {
            ChannelList::new(
                window,
                cx,
                [
                    ChannelType::Open,
                    ChannelType::Private,
                    ChannelType::Direct,
                    ChannelType::Group,
                ],
            )
        });

        Self { channel_list }
    }
}

impl Render for LeftSidebar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex_1()
            .bg(cx.theme().secondary)
            .child(self.channel_list.clone())
            .into_any_element()
    }
}
