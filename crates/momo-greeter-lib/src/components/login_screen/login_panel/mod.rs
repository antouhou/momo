mod style;

use crate::components::login_screen::action_button::ActionButton;
use crate::components::login_screen::login_panel::style::{
    actions_style, avatar_style, avatar_text_style, content_style, input_label_text_style,
    input_style, panel_style,
};
use crate::components::login_screen::state::{GreeterState, GreeterView, UserProfile};
use crate::components::login_screen::style::{
    subtitle_text_style, title_block_style, title_text_style,
};
use daiko::Element;
use daiko::component::{Component, ComponentContext};
use daiko::state_management::StateHandle;
use daiko::widgets::text::Text;
use daiko::widgets::text_input::TextInput;

#[derive(Clone)]
pub(super) struct LoginPanel {
    profile: UserProfile,
    greeter_state: StateHandle<GreeterState>,
}

impl LoginPanel {
    pub(super) fn new(profile: UserProfile, greeter_state: StateHandle<GreeterState>) -> Self {
        Self {
            profile,
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

        if login_button.activated() {
            println!(
                "Pressed login button for {} (password length: {})",
                self.profile.name(),
                password.read().chars().count()
            );
        }

        Element::new()
            .with_tag("credential-panel")
            .with_style(content_style())
            .with_content(
                Element::new()
                    .with_style(title_block_style())
                    .with_content(
                        Text::new(format!("Welcome, {}", self.profile.name()))
                            .with_style(title_text_style()),
                    )
                    .with_content(
                        Text::new("Enter your password to continue")
                            .with_style(subtitle_text_style()),
                    ),
            )
            .with_content(
                Element::new()
                    .with_style(panel_style())
                    .with_content(
                        Element::new()
                            .with_style(avatar_style(self.profile))
                            .with_content(
                                Text::new(self.profile.initials()).with_style(avatar_text_style()),
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
}
