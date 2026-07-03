mod action_button;
mod login_panel;
mod power_button;
mod profile_tile;
mod state;
mod style;
#[cfg(test)]
mod tests;

use crate::components::login_screen::login_panel::LoginPanel;
use crate::components::login_screen::power_button::PowerButton;
use crate::components::login_screen::profile_tile::ProfileTile;
use crate::components::login_screen::state::{GreeterState, GreeterView, PROFILE_ACTIONS};
use crate::components::login_screen::style::{
    brand_text_style, footer_style, header_style, hint_text_style, main_content_style,
    profile_row_style, root_style, subtitle_text_style, title_block_style, title_text_style,
};
use daiko::Element;
use daiko::component::{Component, ComponentContext};
use daiko::navigation::{FocusBoundary, FocusEntryPolicy, TraversalPolicy};
use daiko::widgets::text::Text;

#[derive(Clone, Copy)]
pub struct LoginScreen;

impl LoginScreen {
    pub fn new() -> Self {
        Self
    }
}

impl Component for LoginScreen {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let focus_scope = ctx.focus_scope();
        focus_scope.set_boundary(FocusBoundary::Stop);
        focus_scope.set_entry_policy(FocusEntryPolicy::Spatial(
            TraversalPolicy::RectilinearDistance,
        ));

        let greeter_state = ctx.use_local_state(GreeterState::default);
        let view = greeter_state.read().view;

        Element::new()
            .with_tag("login-screen-root")
            .with_style(root_style())
            .with_content(
                Element::new()
                    .with_tag("greeter-header")
                    .with_style(header_style())
                    .with_content(Text::new("MOMO").with_style(brand_text_style())),
            )
            .with_content(match view {
                GreeterView::Profiles => profile_picker(greeter_state),
                GreeterView::Credentials(profile) => Element::new()
                    .with_style(main_content_style())
                    .with_content(LoginPanel::new(profile, greeter_state)),
            })
            .with_content(
                Element::new()
                    .with_tag("greeter-footer")
                    .with_style(footer_style())
                    .with_content(PowerButton)
                    .with_content(
                        Text::new("Enter / A  Select    •    Arrows / D-pad  Navigate")
                            .with_style(hint_text_style()),
                    ),
            )
    }
}

fn profile_picker(greeter_state: daiko::state_management::StateHandle<GreeterState>) -> Element {
    let mut profile_row = Element::new()
        .with_tag("profile-row")
        .with_style(profile_row_style());

    for action in PROFILE_ACTIONS {
        profile_row.add_content(ProfileTile::new(*action, greeter_state.clone()));
    }

    Element::new()
        .with_tag("profile-picker")
        .with_style(main_content_style())
        .with_content(
            Element::new()
                .with_style(title_block_style())
                .with_content(Text::new("Welcome back").with_style(title_text_style()))
                .with_content(Text::new("Who's signing in?").with_style(subtitle_text_style())),
        )
        .with_content(profile_row)
}
