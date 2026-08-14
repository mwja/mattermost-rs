use gpui::{AnyElement, Element, IntoElement, ParentElement, RenderOnce, Styled, div, rgb};
use gpui_component::{
    ActiveTheme, Icon, Selectable, Sizable,
    button::{Button, ButtonVariants},
    description_list::{DescriptionItem, DescriptionList},
    h_flex,
    hover_card::HoverCard,
    label::Label,
    v_flex,
};

use crate::ui::channel::icon::channel_type_to_icon;

#[derive(IntoElement, Clone)]
pub struct Channel {
    pub id: String,
    pub name: String,
    pub selected: bool,
    pub channel_type: mattermost::client::channel::ChannelType,
}

impl Channel {
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

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

impl RenderOnce for Channel {
    fn render(self, _window: &mut gpui::Window, _cx: &mut gpui::App) -> impl IntoElement {
        HoverCard::new(self.id.clone())
            .trigger(
                Button::new(self.id.clone())
                    .text_align(gpui::TextAlign::Left)
                    .px_4()
                    .child(
                        div()
                            .w_full()
                            .flex()
                            .overflow_hidden()
                            .justify_start()
                            .items_center()
                            .child(
                                div()
                                    .text_ellipsis()
                                    .whitespace_nowrap()
                                    .child(self.name.clone()),
                            ),
                    )
                    .icon(Icon::new(
                        Icon::new(channel_type_to_icon(self.channel_type.clone())).small(),
                    ))
                    .ghost()
                    .selected(self.selected),
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

impl Selectable for Channel {
    fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
    fn is_selected(&self) -> bool {
        self.selected
    }
}
