mod style;

use crate::components::login_screen::action_button::ActionButton;
use crate::components::login_screen::login_panel::style::{
    actions_style, avatar_style, avatar_text_style, content_style, input_label_text_style,
    input_style, panel_style,
};
use crate::components::login_screen::state::{GreeterState, GreeterView};
use crate::components::login_screen::style::{
    subtitle_text_style, title_block_style, title_text_style,
};
use crate::users::{GreeterUser, GreeterUsersStatus, use_greeter_users_state};
use daiko::Element;
use daiko::component::{Component, ComponentContext};
use daiko::state_management::StateHandle;
use daiko::widgets::text::Text;
use daiko::widgets::text_input::TextInput;
use std::sync::Arc;

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
        let login_button = ActionButton::new(ctx, "login-submit", "Log in", true, true);

        if back_button.activated() {
            println!("Pressed back button");
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

        if login_button.activated() {
            println!("Pressed login button for {}", user.username);
            let _ = password.read();
        }

        credential_content(
            user,
            self.user_index,
            password_input,
            back_button,
            login_button,
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
) -> Element {
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
        .with_content(
            Element::new()
                .with_style(panel_style())
                .with_content(
                    Element::new()
                        .with_style(avatar_style(user_index))
                        .with_content(
                            Text::new(Arc::clone(&user.initials)).with_style(avatar_text_style()),
                        ),
                )
                .with_content(Text::new("Password").with_style(input_label_text_style()))
                .with_content(password_input)
                .with_content(
                    Element::new()
                        .with_style(actions_style())
                        .with_content(back_button)
                        .with_content(login_button),
                ),
        )
}
