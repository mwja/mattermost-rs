mod auth;
mod client;
mod store;
mod ui;

use gpui::{
    App, AppContext as _, Bounds, Context, Entity, InteractiveElement, IntoElement, Menu, MenuItem,
    ParentElement, Render, Styled, Subscription, Window, WindowBounds, WindowOptions, actions, div,
    px, size,
};
use mattermost::MattermostClient;
use simple_logger::SimpleLogger;

use crate::client::{AppContextClientExt, SharedClient};
use crate::ui::layout::Layout;
use crate::ui::login::{LoginEvent, LoginPage};

enum RootView {
    LoggedOut(Entity<LoginPage>),
    LoggedIn(Entity<Layout>),
}

struct Root {
    view: RootView,
    _login_subscription: Option<Subscription>,
}

actions!(actions, [Quit, Logout]);

impl Root {
    fn logged_out(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let login_page = cx.new(|cx| LoginPage::new(window, cx));
        let subscription = cx.subscribe_in(&login_page, window, Self::on_login_event);
        Self {
            view: RootView::LoggedOut(login_page),
            _login_subscription: Some(subscription),
        }
    }

    fn logged_in(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let layout = cx.new(|cx| Layout::new(window, cx));
        Self {
            view: RootView::LoggedIn(layout),
            _login_subscription: None,
        }
    }

    fn on_logout_action(&mut self, _action: &Logout, window: &mut Window, cx: &mut Context<Self>) {
        cx.set_client(None);
        auth::clear_session();
        *self = Self::logged_out(window, cx);
        cx.notify();
    }

    fn on_login_event(
        &mut self,
        _login_page: &Entity<LoginPage>,
        event: &LoginEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let LoginEvent::Success(client) = event;
        cx.set_client(Some(client.clone()));
        self.view = RootView::LoggedIn(cx.new(|cx| Layout::new(window, cx)));
        self._login_subscription = None;
        cx.notify();
    }
}

impl Render for Root {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .on_action(cx.listener(Self::on_logout_action))
            .child(match &self.view {
                RootView::LoggedOut(login_page) => login_page.clone().into_any_element(),
                RootView::LoggedIn(layout) => layout.clone().into_any_element(),
            })
    }
}

fn main() {
    SimpleLogger::new().init().unwrap();
    let app = gpui_platform::application().with_assets(gpui_component_assets::Assets);
    app.run(move |cx| {
        // This must be called before using any GPUI Component features.
        gpui_component::init(cx);
        gpui_tokio_bridge::init(cx);

        let cache_dir = directories::ProjectDirs::from("ch", "jacobmacweb", "mattermost-desktop")
            .map(|dirs| dirs.cache_dir().to_path_buf())
            .unwrap_or_else(std::env::temp_dir);

        log::debug!("Storing data in {}", cache_dir.display());

        store::init(cx, cache_dir);

        cx.observe_global::<SharedClient>(|cx| cx.set_menus(build_menus(cx)))
            .detach();

        cx.on_action(quit);

        cx.spawn(async move |cx| {
            let saved_client = auth::load_session().and_then(|session| {
                MattermostClient::try_from(session)
                    .inspect_err(|err| log::warn!("saved session is unusable: {err}"))
                    .ok()
            });

            cx.update(|cx| {
                cx.set_client(saved_client.clone());

                let bounds = Bounds::centered(None, size(px(500.), px(500.0)), cx);
                cx.open_window(
                    WindowOptions {
                        window_bounds: Some(WindowBounds::Windowed(bounds)),
                        ..Default::default()
                    },
                    move |window, cx| {
                        cx.new(|cx| match saved_client {
                            Some(_) => Root::logged_in(window, cx),
                            None => Root::logged_out(window, cx),
                        })
                    },
                )
                .unwrap();

                cx.activate(true);
            });
        })
        .detach();
    });
}

fn build_menus(cx: &App) -> Vec<Menu> {
    let logged_in = cx.get_client().is_some();

    vec![Menu {
        name: "Mattermost (Rust)".into(),
        items: vec![
            MenuItem::action("Log Out", Logout).disabled(!logged_in),
            MenuItem::Separator,
            MenuItem::action("Quit", Quit),
        ],
        disabled: false,
    }]
}

fn quit(_action: &Quit, cx: &mut App) {
    cx.quit();
}
