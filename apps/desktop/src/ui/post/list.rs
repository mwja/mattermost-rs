use gpui::{
    AppContext, AsyncApp, Context, Entity, IntoElement, ParentElement, Render, Styled, Window, div,
};
use gpui_component::list::{List, ListState};
use mattermost::{
    client::post::{GetPostsAt, GetPostsOptions, PostsCursor},
    store::{ChannelPosts, UserById},
};

use crate::{store::AsyncAppContextStoreExt, ui::post::list_delegate::PostListDelegate};

const POSTS_PER_PAGE: u64 = 30;

pub struct PostList {
    channel_id: String,
    list_state: Entity<ListState<PostListDelegate>>,
}

impl PostList {
    pub fn new(channel_id: String, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let owner = cx.weak_entity();

        let list_state = cx.new(|cx| ListState::new(PostListDelegate::new(owner), window, cx));

        cx.spawn({
            let list_state = list_state.clone();
            let channel_id = channel_id.clone();

            async move |_this, cx| {
                let options = GetPostsOptions {
                    channel_id,
                    at: GetPostsAt::Cursor(PostsCursor {
                        per_page: POSTS_PER_PAGE,
                        ..Default::default()
                    }),
                    ..Default::default()
                };

                let result = cx.load(ChannelPosts::new(options)).await;

                let _ = list_state.update(cx, |state, cx| {
                    match result {
                        Ok(posts) => {
                            let has_more = posts.len() as u64 == POSTS_PER_PAGE;
                            state.delegate_mut().set_posts(posts, has_more);
                        }
                        Err(err) => state
                            .delegate_mut()
                            .set_error(format!("Failed to load messages: {err}")),
                    }
                    cx.notify();
                });

                fetch_missing_authors(&list_state, cx);
            }
        })
        .detach();

        Self {
            channel_id,
            list_state,
        }
    }

    pub fn fetch_more(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        let (should_load, before) = {
            let list_state = self.list_state.read(cx);
            let delegate = list_state.delegate();
            (delegate.should_load_more(), delegate.oldest_post_id())
        };

        let Some(before) = before.filter(|_| should_load) else {
            return;
        };

        self.list_state.update(cx, |state, cx| {
            state.delegate_mut().set_loading_more(true);
            cx.notify();
        });

        let channel_id = self.channel_id.clone();
        let list_state = self.list_state.clone();

        cx.spawn(async move |_this, cx| {
            let options = GetPostsOptions {
                channel_id,
                at: GetPostsAt::Cursor(PostsCursor {
                    before: Some(before),
                    per_page: POSTS_PER_PAGE,
                    ..Default::default()
                }),
                ..Default::default()
            };

            let result = cx.load(ChannelPosts::new(options)).await;

            let _ = list_state.update(cx, |state, cx| {
                match result {
                    Ok(posts) => {
                        let has_more = posts.len() as u64 == POSTS_PER_PAGE;
                        state.delegate_mut().append_posts(posts, has_more);
                    }
                    Err(err) => {
                        log::warn!("Failed to load more messages: {err}");
                        state.delegate_mut().set_loading_more(false);
                    }
                }
                cx.notify();
            });

            fetch_missing_authors(&list_state, cx);
        })
        .detach();
    }
}

/// Fetches (and caches) any post authors not already attached to the loaded
/// posts, patching each one in as it resolves rather than waiting on all of them.
fn fetch_missing_authors(list_state: &Entity<ListState<PostListDelegate>>, cx: &mut AsyncApp) {
    let missing = list_state.read_with(cx, |state, _| state.delegate().missing_author_ids());

    for user_id in missing {
        let list_state = list_state.clone();

        cx.spawn(async move |cx| {
            let Ok(user) = cx.load(UserById(user_id.clone())).await else {
                return;
            };

            let _ = list_state.update(cx, |state, cx| {
                state.delegate_mut().set_author(&user_id, user);
                cx.notify();
            });
        })
        .detach();
    }
}

impl Render for PostList {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .child(List::new(&self.list_state).flex_1())
    }
}
