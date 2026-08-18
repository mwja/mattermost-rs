use gpui::{AnyElement, Element, IntoElement, ParentElement, RenderOnce, Styled, div, rgb};
use gpui_component::{
    ActiveTheme, Icon, Selectable, Sizable,
    button::{Button, ButtonVariants},
    description_list::{DescriptionItem, DescriptionList},
    h_flex,
    hover_card::HoverCard,
    label::Label,
    list::ListItem,
    v_flex,
};

use crate::ui::channel::icon::channel_type_to_icon;

#[derive(IntoElement, Clone)]
pub struct ChannelRow {
    pub id: String,
    pub name: String,
    pub selected: bool,
    pub channel_type: mattermost::client::channel::ChannelType,
}

impl ChannelRow {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            name: "".to_string(),
            selected: false,
            channel_type: mattermost::client::channel::ChannelType::Open,
        }
    }

    pub fn channel_type(mut self, channel_type: mattermost::client::channel::ChannelType) -> Self {
        self.channel_type = channel_type;
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }
}

impl RenderOnce for ChannelRow {
    fn render(self, _window: &mut gpui::Window, _cx: &mut gpui::App) -> impl IntoElement {
        HoverCard::new(self.id.clone())
            .trigger(
                h_flex()
                    .items_center()
                    .gap_2()
                    .child(channel_type_to_icon(self.channel_type.clone()))
                    .child(
                        Label::new(self.name.clone())
                            .text_ellipsis()
                            .whitespace_nowrap(),
                    ),
            )
            .child(
                DescriptionList::new()
                    .children([
                        DescriptionItem::new("Name").value(
                            h_flex()
                                .gap_2()
                                .children([
                                    Icon::new(channel_type_to_icon(self.channel_type))
                                        .small()
                                        .into_any_element(),
                                    Label::new(self.name).into_any_element(),
                                ])
                                .into_any_element(),
                        ),
                        DescriptionItem::new("ID").value(Label::new(self.id).into_any_element()),
                    ])
                    .bordered(false)
                    .columns(1)
                    .layout(gpui::Axis::Vertical),
            )
    }
}
