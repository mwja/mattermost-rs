use gpui::{IntoElement, ParentElement, RenderOnce, Styled};
use gpui_component::{Selectable, StyledExt, avatar::Avatar, h_flex, label::Label, v_flex};

#[derive(IntoElement, Clone)]
pub struct PostRow {
    pub id: String,
    pub author_name: String,
    pub author_avatar: String,
    pub message: String,
    selected: bool,
    // TODO: Embeds etc
}

impl PostRow {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            author_name: "".to_string(),
            author_avatar: "".to_string(),
            message: "".to_string(),
            selected: false,
        }
    }

    pub fn author_name(mut self, name: &str) -> Self {
        self.author_name = name.to_string();
        self
    }

    pub fn author_avatar(mut self, avatar: &str) -> Self {
        self.author_avatar = avatar.to_string();
        self
    }

    pub fn message(mut self, message: &str) -> Self {
        self.message = message.to_string();
        self
    }
}

impl RenderOnce for PostRow {
    fn render(self, window: &mut gpui::Window, cx: &mut gpui::App) -> impl IntoElement {
        h_flex()
            .gap_3()
            .child(
                Avatar::new()
                    .name(self.author_name.clone())
                    .src(self.author_avatar),
            )
            .child(
                v_flex()
                    .gap_1()
                    .child(Label::new(self.author_name).font_bold())
                    .child(Label::new(self.message)),
            )
    }
}

impl Selectable for PostRow {
    fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
    fn is_selected(&self) -> bool {
        self.selected
    }
}
