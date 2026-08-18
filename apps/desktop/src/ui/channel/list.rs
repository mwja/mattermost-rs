use gpui::{
    AppContext, Context, Entity, EventEmitter, IntoElement, ParentElement, Render, Styled, Window,
    div,
};
use gpui_component::list::{List, ListState};
use mattermost::{
    client::channel::ChannelType,
    store::{AllChannelCategories, AllChannels},
};

use crate::{store::AsyncAppContextStoreExt, ui::channel::list_delegate::ChannelListDelegate};

pub enum ChannelListEvent {
    ChannelSelected(String),
}

pub struct ChannelList {
    list_state: Entity<ListState<ChannelListDelegate>>,
}

impl EventEmitter<ChannelListEvent> for ChannelList {}

impl ChannelList {
    pub fn new(
        window: &mut Window,
        cx: &mut Context<Self>,
        channel_types: impl Into<Vec<ChannelType>>,
    ) -> Self {
        use gpui_component::list::ListEvent;

        let list_state = cx.new(|cx| {
            ListState::new(ChannelListDelegate::new(channel_types.into()), window, cx)
                .searchable(true)
        });

        cx.subscribe(&list_state, |this, list_state, event, cx| match event {
            ListEvent::Confirm(ix) => {
                let Some(channel) = list_state.read(cx).delegate().channel_at(*ix) else {
                    return;
                };

                cx.emit(ChannelListEvent::ChannelSelected(channel.id.clone()));
            }
            _ => {}
        })
        .detach();

        cx.spawn({
            let list_state = list_state.clone();
            async move |_this, cx| {
                let result = cx.load(AllChannels).await;
                log::debug!("Loaded channels: {:?}", result);
                let categories = cx.load(AllChannelCategories).await;
                log::debug!("Loaded channel categories: {:?}", categories);

                list_state.update(cx, |state, cx| {
                    match result {
                        Ok(channels) => {
                            log::debug!("Loaded {} channels", channels.len());
                            state.delegate_mut().set_channels(channels);
                            state
                                .delegate_mut()
                                .set_channel_categories(categories.unwrap_or_default());
                        }
                        Err(e) => state
                            .delegate_mut()
                            .set_error(format!("Failed to load channels: {e}")),
                    }
                    cx.notify();
                });
            }
        })
        .detach();

        Self { list_state }
    }
}

impl Render for ChannelList {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .child(List::new(&self.list_state).flex_1())
    }
}
