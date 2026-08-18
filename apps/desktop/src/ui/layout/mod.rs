use gpui::{AppContext, Entity, IntoElement, ParentElement, Render, Styled, div, px};
use gpui_component::{ActiveTheme, h_resizable, resizable_panel};

pub mod left_sidebar;
use left_sidebar::LeftSidebar;
use mattermost::client::channel::ChannelType;

use crate::ui::{
    channel::{ChannelList, list::ChannelListEvent},
    post::list::PostList,
};

pub struct Layout {
    channel_list: Entity<ChannelList>,
    channel_selected: Option<String>,
    posts: Option<Entity<PostList>>,
}

impl Layout {
    pub fn new(window: &mut gpui::Window, cx: &mut gpui::Context<Self>) -> Self {
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

        cx.subscribe_in(&channel_list, window, |this, _, event, window, cx| {
            match event {
                ChannelListEvent::ChannelSelected(channel_id) => {
                    this.select_channel(channel_id.clone(), window, cx);
                }
            }
        })
        .detach();

        Self {
            channel_list,
            channel_selected: None,
            posts: None,
        }
    }

    fn select_channel(
        &mut self,
        channel_id: String,
        window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) {
        if self.channel_selected.as_deref() == Some(channel_id.as_str()) {
            return;
        }

        self.posts = Some(cx.new(|cx| PostList::new(channel_id.clone(), window, cx)));
        self.channel_selected = Some(channel_id);
        cx.notify();
    }
}

impl Render for Layout {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        let content = match &self.posts {
            Some(posts) => posts.clone().into_any_element(),
            None => div()
                .size_full()
                .flex()
                .items_center()
                .justify_center()
                .child("Select a channel to view messages")
                .into_any_element(),
        };

        div().bg(cx.theme().background).size_full().child(
            h_resizable("root-layout")
                .child(
                    resizable_panel()
                        .size(px(250.))
                        .size_range(px(250.)..px(400.))
                        .child(
                            LeftSidebar::new()
                                .child(self.channel_list.clone())
                                .into_any_element(),
                        ),
                )
                .child(content),
        )
    }
}
