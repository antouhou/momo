mod action_button;
mod clock;
mod login_panel;
mod power_button;
mod profile_tile;
mod state;
mod style;

use std::sync::Arc;
use daiko::{
    Element, StringOrReference,
    component::{Component, ComponentContext},
    navigation::{FocusBoundary, FocusEntryPolicy, TraversalPolicy},
    state_management::StateHandle,
    widgets::text::Text,
};
use crate::{
    auth::{GreeterAuthStatus, use_greeter_auth_state},
    components::login_screen::{
        clock::Clock,
        login_panel::LoginPanel,
        power_button::PowerButton,
        profile_tile::{AvatarTone, GlyphScale, ProfileTile, ProfileTilePresentation},
        state::{GreeterState, GreeterView},
        style::{
            footer_style, header_style, main_content_style, profile_row_style, root_style,
            subtitle_text_style, title_block_style, title_text_style,
        },
    },
    users::{GreeterUser, GreeterUsersStatus, use_greeter_users_state},
};

#[derive(Clone)]
pub struct LoginScreen {}

impl LoginScreen {
    pub fn new() -> Self {
        Self {}
    }
}

impl Component for LoginScreen {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let focus_scope = ctx.focus_scope();
        focus_scope.set_boundary(FocusBoundary::Stop);
        focus_scope.set_entry_policy(FocusEntryPolicy::Spatial(
            TraversalPolicy::RectilinearDistance,
        ));

        let users_state = use_greeter_users_state(ctx);
        let users_guard = users_state.read();
        let auth_state = use_greeter_auth_state(ctx);
        if matches!(&auth_state.read().status, GreeterAuthStatus::Started { .. }) {
            ctx.app_context.close_app();
        }
        let greeter_state = ctx.use_local_state(GreeterState::default);
        let view = greeter_state.read().view;

        Element::new()
            .with_tag("login-screen-root")
            .with_style(root_style())
            .with_content(
                Element::new()
                    .with_tag("greeter-header")
                    .with_style(header_style())
                    .with_content(Clock::new(true)),
            )
            .with_content(login_content(ctx, greeter_state, view, &users_guard.status))
            .with_content(
                Element::new()
                    .with_tag("greeter-footer")
                    .with_style(footer_style())
                    .with_content(PowerButton),
            )
    }
}

fn login_content(
    ctx: &mut ComponentContext,
    greeter_state: StateHandle<GreeterState>,
    view: GreeterView,
    users_status: &GreeterUsersStatus,
) -> Element {
    match (view, users_status) {
        (_, GreeterUsersStatus::Loading) => status_content("Loading users", "Please wait"),
        (_, GreeterUsersStatus::Unavailable(message)) => {
            status_content("Unable to load users", Arc::clone(message))
        }
        (_, GreeterUsersStatus::Empty) => {
            status_content("No users found", "No login-capable local users were found")
        }
        (GreeterView::Profiles, GreeterUsersStatus::Ready(users)) => {
            profile_picker(ctx, greeter_state, users)
        }
        (GreeterView::Credentials { user_index }, GreeterUsersStatus::Ready(_)) => Element::new()
            .with_style(main_content_style())
            .with_content(LoginPanel::new(user_index, greeter_state)),
    }
}

fn status_content(title: &'static str, subtitle: impl Into<StringOrReference>) -> Element {
    Element::new()
        .with_tag("greeter-status")
        .with_style(main_content_style())
        .with_content(
            Element::new()
                .with_style(title_block_style())
                .with_content(Text::new(title).with_style(title_text_style()))
                .with_content(Text::new(subtitle).with_style(subtitle_text_style())),
        )
}

fn profile_picker(
    ctx: &mut ComponentContext,
    greeter_state: StateHandle<GreeterState>,
    users: &[GreeterUser],
) -> Element {
    let mut profile_row = Element::new()
        .with_tag("profile-row")
        .with_style(profile_row_style());

    for (index, user) in users.iter().enumerate() {
        let profile_tile = ProfileTile::new(ctx, profile_tile_presentation(user, index));
        if profile_tile.activated() {
            tracing::debug!(username = %user.username, "selected user");
            greeter_state.write().view = GreeterView::Credentials { user_index: index };
        }
        profile_row.add_content(profile_tile);
    }

    Element::new()
        .with_tag("profile-picker")
        .with_style(main_content_style())
        .with_content(
            Element::new()
                .with_style(title_block_style())
                .with_content(Text::new("Welcome back").with_style(title_text_style())),
        )
        .with_content(profile_row)
}

fn profile_tile_presentation(user: &GreeterUser, index: usize) -> ProfileTilePresentation {
    ProfileTilePresentation {
        label: Arc::clone(&user.display_name),
        glyph: Arc::clone(&user.initials),
        avatar_tone: avatar_tone(index),
        glyph_scale: GlyphScale::Standard,
        is_preferred_focus: index == 0,
    }
}

fn avatar_tone(user_index: usize) -> AvatarTone {
    match user_index % 3 {
        0 => AvatarTone::Blue,
        1 => AvatarTone::Violet,
        _ => AvatarTone::Green,
    }
}
