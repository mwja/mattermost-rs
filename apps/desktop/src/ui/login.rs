use gpui::{
    AppContext as _, Context, Entity, EventEmitter, IntoElement, ParentElement, Render, Styled,
    Subscription, Window, prelude::FluentBuilder as _, px,
};
use gpui_component::{
    ActiveTheme,
    button::{Button, ButtonVariants as _},
    input::{Input, InputEvent, InputState},
    label::Label,
    v_flex,
};
use mattermost::MattermostClient;

use crate::auth;

pub enum LoginEvent {
    Success(MattermostClient),
}

pub struct LoginPage {
    server_input: Entity<InputState>,
    username_input: Entity<InputState>,
    password_input: Entity<InputState>,
    error: Option<String>,
    loading: bool,
    _subscriptions: Vec<Subscription>,
}

impl EventEmitter<LoginEvent> for LoginPage {}

impl LoginPage {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let server_input = cx.new(|cx| {
            InputState::new(window, cx).placeholder("https://mattermost.example.com")
        });
        let username_input =
            cx.new(|cx| InputState::new(window, cx).placeholder("Email or username"));
        let password_input =
            cx.new(|cx| InputState::new(window, cx).masked(true).placeholder("Password"));

        username_input.update(cx, |state, cx| state.focus(window, cx));

        let _subscriptions = vec![
            cx.subscribe_in(&username_input, window, Self::on_input_event),
            cx.subscribe_in(&password_input, window, Self::on_input_event),
        ];

        Self {
            server_input,
            username_input,
            password_input,
            error: None,
            loading: false,
            _subscriptions,
        }
    }

    fn on_input_event(
        &mut self,
        _state: &Entity<InputState>,
        event: &InputEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let InputEvent::PressEnter { .. } = event {
            self.submit(cx);
        }
    }

    fn submit(&mut self, cx: &mut Context<Self>) {
        if self.loading {
            return;
        }

        let base_url = self.server_input.read(cx).value().trim().to_string();
        let username = self.username_input.read(cx).value().trim().to_string();
        let password = self.password_input.read(cx).value().to_string();

        if base_url.is_empty() || username.is_empty() || password.is_empty() {
            self.error = Some("Please fill in the server, username and password.".into());
            cx.notify();
            return;
        }

        self.loading = true;
        self.error = None;
        cx.notify();

        cx.spawn(async move |this, cx| {
            let login = gpui_tokio_bridge::spawn(cx, async move {
                MattermostClient::new_with_login(base_url, username, password).await
            })
            .await;

            let outcome = match login {
                Ok(Ok(client)) => Ok(client),
                Ok(Err(err)) => Err(err.to_string()),
                Err(_) => Err("Login failed unexpectedly, please try again.".to_string()),
            };

            this.update(cx, |this, cx| {
                this.loading = false;
                match outcome {
                    Ok(client) => {
                        auth::save_session(&(&client).into());
                        cx.emit(LoginEvent::Success(client));
                    }
                    Err(err) => this.error = Some(err),
                }
                cx.notify();
            })
        })
        .detach();
    }
}

impl Render for LoginPage {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .items_center()
            .justify_center()
            .bg(cx.theme().background)
            .child(
                v_flex()
                    .w(px(340.))
                    .gap_4()
                    .p_6()
                    .border_1()
                    .border_color(cx.theme().border)
                    .rounded(cx.theme().radius)
                    .bg(cx.theme().secondary)
                    .child(Label::new("Sign in to Mattermost").text_lg())
                    .child(
                        v_flex()
                            .gap_1()
                            .child(Label::new("Server URL").text_sm())
                            .child(Input::new(&self.server_input).disabled(self.loading)),
                    )
                    .child(
                        v_flex()
                            .gap_1()
                            .child(Label::new("Username").text_sm())
                            .child(Input::new(&self.username_input).disabled(self.loading)),
                    )
                    .child(
                        v_flex()
                            .gap_1()
                            .child(Label::new("Password").text_sm())
                            .child(
                                Input::new(&self.password_input)
                                    .mask_toggle()
                                    .disabled(self.loading),
                            ),
                    )
                    .when_some(self.error.as_ref(), |this, error| {
                        this.child(Label::new(error.clone()).text_sm().text_color(cx.theme().danger))
                    })
                    .child(
                        Button::new("login-submit")
                            .label("Sign in")
                            .primary()
                            .w_full()
                            .loading(self.loading)
                            .on_click(cx.listener(|this, _, _window, cx| this.submit(cx))),
                    ),
            )
    }
}
