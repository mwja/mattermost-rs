use gpui::{
    App, Context, InteractiveElement, IntoElement, ParentElement, Styled, Task, Window, div,
};
use gpui_component::{
    ActiveTheme, IndexPath,
    label::Label,
    list::{ListDelegate, ListState},
    v_flex,
};
use mattermost::client::{self, channel::ChannelType};

use crate::ui::channel::channel::Channel;

pub struct ChannelListDelegate {
    channels: Vec<client::channel::Channel>,
    channel_categories: Vec<client::channel::ChannelCategory>,
    channel_types: Vec<ChannelType>,
    search_query: String,
    selected_index: Option<IndexPath>,
    loading: bool,
    error: Option<String>,
}

impl ChannelListDelegate {
    pub fn new(channel_types: Vec<ChannelType>) -> Self {
        Self {
            channels: Vec::new(),
            channel_categories: Vec::new(),
            channel_types,
            search_query: String::new(),
            selected_index: None,
            loading: true,
            error: None,
        }
    }

    /// Called once the channel list finishes loading.
    pub fn set_channels(&mut self, channels: Vec<client::channel::Channel>) {
        self.channels = channels;
        self.loading = false;
        self.error = None;
    }

    pub fn set_channel_categories(&mut self, categories: Vec<client::channel::ChannelCategory>) {
        self.channel_categories = categories;
    }

    /// Called if loading fails; surfaced via [`ListDelegate::render_empty`].
    pub fn set_error(&mut self, error: String) {
        self.error = Some(error);
        self.loading = false;
    }
    fn visible(&self) -> impl Iterator<Item = &client::channel::Channel> {
        self.channels.iter().filter(|c| {
            self.channel_types.contains(&c.channel_type)
                && c.display_name
                    .to_lowercase()
                    .contains(&self.search_query.to_lowercase())
        })
    }

    fn ordered_categories(&self) -> impl Iterator<Item = &client::channel::ChannelCategory> {
        let mut ordered_categories = self.channel_categories.iter().collect::<Vec<_>>();

        ordered_categories.sort_by_key(|cat| cat.sort_order);

        ordered_categories.into_iter()
    }

    /// Returns the categories and the channels in it, in order. Elements are also
    /// filtered for visibility. Should also order the categories
    fn ordered_categories_with_channels(
        &self,
    ) -> impl Iterator<
        Item = (
            &client::channel::ChannelCategory,
            Vec<&client::channel::Channel>,
        ),
    > {
        self.ordered_categories().map(move |cat| {
            let channels = self
                .visible()
                .filter(|channel| cat.channel_ids.contains(&channel.id))
                .collect::<Vec<_>>();

            (cat, channels)
        })
    }
}

impl ListDelegate for ChannelListDelegate {
    type Item = Channel;

    fn loading(&self, _cx: &App) -> bool {
        self.loading
    }

    fn render_loading(
        &mut self,
        _window: &mut Window,
        _cx: &mut Context<ListState<Self>>,
    ) -> impl IntoElement {
        v_flex()
            .size_full()
            .items_center()
            .justify_center()
            .child("Loading channels…")
    }

    fn render_empty(
        &mut self,
        _window: &mut Window,
        _cx: &mut Context<ListState<Self>>,
    ) -> impl IntoElement {
        let message = match &self.error {
            Some(err) => format!("Failed to load channels: {err}"),
            None => "No channels".to_string(),
        };

        div()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .child(message)
    }

    fn sections_count(&self, _cx: &App) -> usize {
        self.channel_categories.len()
    }

    fn items_count(&self, section: usize, _cx: &App) -> usize {
        self.ordered_categories_with_channels()
            .nth(section)
            .map(|(_, v)| v.len())
            .unwrap_or(0)
    }

    fn render_section_header(
        &mut self,
        section: usize,
        window: &mut Window,
        cx: &mut Context<ListState<Self>>,
    ) -> Option<impl IntoElement> {
        let category = self.ordered_categories().nth(section)?;

        Some(
            div()
                .id(category.id.clone())
                .bg(cx.theme().background)
                .size_full()
                .px_4()
                .py_2()
                .flex()
                .items_center()
                .child(Label::new(category.display_name.clone().to_uppercase())),
        )
    }

    fn render_item(
        &mut self,
        ix: IndexPath,
        _window: &mut Window,
        _cx: &mut Context<ListState<Self>>,
    ) -> Option<Self::Item> {
        self.ordered_categories_with_channels()
            .nth(ix.section)
            .and_then(|(_, channels)| channels.into_iter().nth(ix.row))
            .map(|item| {
                Channel::new(&item.id)
                    .name(&item.display_name)
                    .selected(Some(ix) == self.selected_index)
                    .channel_type(item.channel_type.clone())
            })
    }

    fn set_selected_index(
        &mut self,
        ix: Option<IndexPath>,
        _window: &mut Window,
        cx: &mut Context<ListState<Self>>,
    ) {
        self.selected_index = ix;
        cx.notify();
    }

    fn perform_search(
        &mut self,
        query: &str,
        _window: &mut Window,
        _cx: &mut Context<ListState<Self>>,
    ) -> Task<()> {
        self.search_query = query.to_string();
        Task::ready(())
    }
}
