use gpui::{App, Context, IntoElement, ParentElement, Styled, WeakEntity, Window, div};
use gpui_component::{
    IndexPath, Selectable,
    list::{ListDelegate, ListItem, ListState},
};
use mattermost::client::{post::Post, user::User};

use crate::ui::post::{list::PostList, row::PostRow};

struct PostRowData {
    author: Option<User>,
    post: Post,
}

pub struct PostListDelegate {
    // We must have this as list delegates can only emit list events.
    owner: WeakEntity<PostList>,
    posts: Vec<PostRowData>,
    has_more: bool,
    loading: bool,
    loading_more: bool,
    error: Option<String>,
    selected_index: Option<IndexPath>,
}

impl PostListDelegate {
    pub fn new(owner: WeakEntity<PostList>) -> Self {
        Self {
            owner,
            posts: Vec::new(),
            has_more: true,
            loading: true,
            loading_more: false,
            error: None,
            selected_index: None,
        }
    }

    /// Called once the first page of posts finishes loading.
    pub fn set_posts(&mut self, posts: Vec<Post>, has_more: bool) {
        self.posts = posts
            .into_iter()
            .map(|post| PostRowData { author: None, post })
            .collect();
        self.has_more = has_more;
        self.loading = false;
        self.error = None;
    }

    /// Called once an older page of posts (via `load_more`) finishes loading.
    pub fn append_posts(&mut self, posts: Vec<Post>, has_more: bool) {
        self.posts.extend(
            posts
                .into_iter()
                .map(|post| PostRowData { author: None, post }),
        );
        self.has_more = has_more;
        self.loading_more = false;
    }

    /// Called if loading fails; surfaced via [`ListDelegate::render_empty`].
    pub fn set_error(&mut self, error: String) {
        self.error = Some(error);
        self.loading = false;
        self.loading_more = false;
    }

    pub fn set_loading_more(&mut self, loading_more: bool) {
        self.loading_more = loading_more;
    }

    /// Whether a `load_more` fetch should actually be started: there must be
    /// more history to fetch, and no fetch already in flight.
    pub fn should_load_more(&self) -> bool {
        self.has_more && !self.loading_more
    }

    /// The id of the oldest loaded post, used as the `before` cursor for pagination.
    pub fn oldest_post_id(&self) -> Option<String> {
        self.posts.first().map(|data| data.post.id.clone())
    }

    /// Attaches a fetched author to every currently-loaded post by that user.
    pub fn set_author(&mut self, user_id: &str, user: User) {
        for data in self.posts.iter_mut().filter(|data| data.post.user_id == user_id) {
            data.author = Some(user.clone());
        }
    }

    /// Distinct author ids among the loaded posts that don't have author data yet.
    pub fn missing_author_ids(&self) -> Vec<String> {
        let mut ids: Vec<String> = self
            .posts
            .iter()
            .filter(|data| data.author.is_none())
            .map(|data| data.post.user_id.clone())
            .collect();
        ids.sort_unstable();
        ids.dedup();
        ids
    }
}

impl ListDelegate for PostListDelegate {
    type Item = ListItem;

    fn loading(&self, _cx: &App) -> bool {
        self.loading
    }

    fn render_empty(
        &mut self,
        _window: &mut Window,
        _cx: &mut Context<ListState<Self>>,
    ) -> impl IntoElement {
        let message = match &self.error {
            Some(err) => format!("Failed to load messages: {err}"),
            None => "No messages yet".to_string(),
        };

        div()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .child(message)
    }

    fn items_count(&self, _section: usize, _cx: &App) -> usize {
        self.posts.len()
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

    fn has_more(&self, _cx: &App) -> bool {
        self.has_more
    }

    fn load_more_threshold(&self) -> usize {
        10
    }

    fn load_more(&mut self, _window: &mut Window, cx: &mut Context<ListState<Self>>) {
        let _ = self.owner.update_in(cx, |owner, window, cx| {
            owner.fetch_more(window, cx);
        });
    }

    fn render_item(
        &mut self,
        ix: IndexPath,
        _window: &mut Window,
        _cx: &mut Context<ListState<Self>>,
    ) -> Option<Self::Item> {
        let data = self.posts.get(ix.row)?;
        // No user-fetching API exists yet, so fall back to the raw user id.
        let author_name = data
            .author
            .as_ref()
            .map(|user| user.username.clone())
            .unwrap_or_else(|| data.post.user_id.clone());

        Some(
            ListItem::new(data.post.id.clone()).child(
                PostRow::new(&data.post.id)
                    .author_name(&author_name)
                    .message(&data.post.message)
                    .selected(Some(ix) == self.selected_index),
            ),
        )
    }
}
