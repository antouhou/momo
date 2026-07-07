mod style;

use std::sync::Arc;
use daiko::{
    Element, StringOrReference,
    component::{Component, ComponentContext},
    state_management::StateHandle,
    widgets::{text::Text, text_input::TextInput},
};
use crate::{
    auth::{GreeterAuthStatus, submit_greeter_auth_request, use_greeter_auth_state},
    components::login_screen::{
        action_button::ActionButton,
        login_panel::style::{
            actions_style, auth_message_text_style, avatar_style, avatar_text_style, content_style,
            input_label_text_style, input_style, panel_style,
        },
        state::{GreeterState, GreeterView},
        style::{subtitle_text_style, title_block_style, title_text_style},
    },
    users::{GreeterUser, GreeterUsersStatus, use_greeter_users_state},
};

#[derive(Clone)]
pub(super) struct LoginPanel {
    user_index: usize,
    greeter_state: StateHandle<GreeterState>,
}

impl LoginPanel {
    pub(super) fn new(user_index: usize, greeter_state: StateHandle<GreeterState>) -> Self {
        Self {
            user_index,
            greeter_state,
        }
    }
}

impl Component for LoginPanel {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let password_input = TextInput::new(ctx).with_style(input_style());
        let password = password_input.current_text();
        let back_button = ActionButton::new(ctx, "login-back", "Back", false, false);
        let auth_state = use_greeter_auth_state(ctx);
        let is_authenticating = {
            let auth_guard = auth_state.read();
            matches!(&auth_guard.status, GreeterAuthStatus::Authenticating { .. })
        };
        let login_button_label = if is_authenticating {
            "Signing in"
        } else {
            "Log in"
        };
        let login_button = ActionButton::new(ctx, "login-submit", login_button_label, true, true);

        if back_button.activated() {
            tracing::debug!("pressed back button");
            auth_state.write().status = GreeterAuthStatus::Idle;
            self.greeter_state.write().view = GreeterView::Profiles;
        }

        let users_state = use_greeter_users_state(ctx);
        let users_guard = users_state.read();
        let user = match &users_guard.status {
            GreeterUsersStatus::Ready(users) => users.get(self.user_index),
            GreeterUsersStatus::Loading
            | GreeterUsersStatus::Empty
            | GreeterUsersStatus::Unavailable(_) => None,
        };

        let Some(user) = user else {
            return missing_user_content();
        };

        if login_button.activated() && !is_authenticating {
            tracing::debug!(username = %user.username, "pressed login button");
            let secret = password.read().clone();
            *password.write() = String::new();
            if let Err(error) =
                submit_greeter_auth_request(&auth_state, Arc::clone(&user.username), secret)
            {
                auth_state.write().status = GreeterAuthStatus::Failed {
                    username: Arc::clone(&user.username),
                    message: Arc::new(error.to_string()),
                };
            }
        }

        let auth_message = {
            let auth_guard = auth_state.read();
            auth_status_message(&auth_guard.status, user)
        };
        credential_content(
            user,
            self.user_index,
            password_input,
            back_button,
            login_button,
            auth_message,
        )
    }
}

fn missing_user_content() -> Element {
    Element::new()
        .with_tag("credential-panel")
        .with_style(content_style())
        .with_content(
            Element::new()
                .with_style(title_block_style())
                .with_content(Text::new("User unavailable").with_style(title_text_style()))
                .with_content(
                    Text::new("Return to the user list").with_style(subtitle_text_style()),
                ),
        )
}

fn credential_content(
    user: &GreeterUser,
    user_index: usize,
    password_input: TextInput,
    back_button: ActionButton,
    login_button: ActionButton,
    auth_message: Option<AuthMessage>,
) -> Element {
    let mut panel = Element::new()
        .with_style(panel_style())
        .with_content(
            Element::new()
                .with_style(avatar_style(user_index))
                .with_content(
                    Text::new(Arc::clone(&user.initials)).with_style(avatar_text_style()),
                ),
        )
        .with_content(Text::new("Password").with_style(input_label_text_style()))
        .with_content(password_input);

    if let Some(message) = auth_message {
        panel.add_content(
            Text::new(message.text).with_style(auth_message_text_style(message.is_error)),
        );
    }

    panel.add_content(
        Element::new()
            .with_style(actions_style())
            .with_content(back_button)
            .with_content(login_button),
    );

    Element::new()
        .with_tag("credential-panel")
        .with_style(content_style())
        .with_content(
            Element::new()
                .with_style(title_block_style())
                .with_content(
                    Text::new(Arc::clone(&user.display_name)).with_style(title_text_style()),
                )
                .with_content(
                    Text::new("Enter your password to continue").with_style(subtitle_text_style()),
                ),
        )
        .with_content(panel)
}

struct AuthMessage {
    text: StringOrReference,
    is_error: bool,
}

fn auth_status_message(status: &GreeterAuthStatus, user: &GreeterUser) -> Option<AuthMessage> {
    match status {
        GreeterAuthStatus::Idle => None,
        GreeterAuthStatus::Authenticating { username } if username == &user.username => {
            Some(AuthMessage {
                text: "Authenticating".into(),
                is_error: false,
            })
        }
        GreeterAuthStatus::Started { username } if username == &user.username => {
            Some(AuthMessage {
                text: "Starting session".into(),
                is_error: false,
            })
        }
        GreeterAuthStatus::Failed { username, message } if username == &user.username => {
            Some(AuthMessage {
                text: Arc::clone(message).into(),
                is_error: true,
            })
        }
        GreeterAuthStatus::Authenticating { .. }
        | GreeterAuthStatus::Started { .. }
        | GreeterAuthStatus::Failed { .. } => None,
    }
}
