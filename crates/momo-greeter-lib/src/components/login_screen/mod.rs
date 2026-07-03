mod action_button;
mod clock;
mod login_panel;
mod power_button;
mod profile_tile;
mod state;
mod style;

use crate::components::login_screen::clock::Clock;
use crate::components::login_screen::login_panel::LoginPanel;
use crate::components::login_screen::power_button::PowerButton;
use crate::components::login_screen::profile_tile::{
    AvatarTone, GlyphScale, ProfileTile, ProfileTilePresentation,
};
use crate::components::login_screen::state::{
    GreeterState, GreeterView, PROFILE_ACTIONS, ProfileAction, UserProfile,
};
use crate::components::login_screen::style::{
    footer_style, header_style, main_content_style, profile_row_style, root_style,
    title_block_style, title_text_style,
};
use daiko::Element;
use daiko::component::{Component, ComponentContext};
use daiko::navigation::{FocusBoundary, FocusEntryPolicy, TraversalPolicy};
use daiko::state_management::StateHandle;
use daiko::widgets::text::Text;

#[derive(Clone, Copy)]
pub struct LoginScreen {
    live_clock: bool,
}

impl LoginScreen {
    pub fn new() -> Self {
        Self { live_clock: true }
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
                    .with_content(Clock::new(self.live_clock)),
            )
            .with_content(match view {
                GreeterView::Profiles => profile_picker(ctx, greeter_state),
                GreeterView::Credentials(profile) => Element::new()
                    .with_style(main_content_style())
                    .with_content(LoginPanel::new(profile, greeter_state)),
            })
            .with_content(
                Element::new()
                    .with_tag("greeter-footer")
                    .with_style(footer_style())
                    .with_content(PowerButton),
            )
    }
}

fn profile_picker(ctx: &mut ComponentContext, greeter_state: StateHandle<GreeterState>) -> Element {
    let mut profile_row = Element::new()
        .with_tag("profile-row")
        .with_style(profile_row_style());

    for action in PROFILE_ACTIONS {
        let profile_tile = ProfileTile::new(ctx, profile_tile_presentation(*action));
        if profile_tile.activated() {
            handle_profile_action(*action, &greeter_state);
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

fn handle_profile_action(action: ProfileAction, greeter_state: &StateHandle<GreeterState>) {
    match action {
        ProfileAction::Login(profile) => {
            println!("Selected user {}", profile.name());
            greeter_state.write().view = GreeterView::Credentials(profile);
        }
        ProfileAction::AddUser => println!("Pressed add user button"),
    }
}

fn profile_tile_presentation(action: ProfileAction) -> ProfileTilePresentation {
    match action {
        ProfileAction::Login(UserProfile::Anton) => ProfileTilePresentation {
            tag: "profile-anton",
            label: UserProfile::Anton.name(),
            glyph: UserProfile::Anton.initials(),
            avatar_tone: AvatarTone::Blue,
            glyph_scale: GlyphScale::Standard,
            is_preferred_focus: true,
        },
        ProfileAction::Login(UserProfile::Maya) => ProfileTilePresentation {
            tag: "profile-maya",
            label: UserProfile::Maya.name(),
            glyph: UserProfile::Maya.initials(),
            avatar_tone: AvatarTone::Violet,
            glyph_scale: GlyphScale::Standard,
            is_preferred_focus: false,
        },
        ProfileAction::Login(UserProfile::Guest) => ProfileTilePresentation {
            tag: "profile-guest",
            label: UserProfile::Guest.name(),
            glyph: UserProfile::Guest.initials(),
            avatar_tone: AvatarTone::Green,
            glyph_scale: GlyphScale::Standard,
            is_preferred_focus: false,
        },
        ProfileAction::AddUser => ProfileTilePresentation {
            tag: "profile-add-user",
            label: "Add user",
            glyph: "+",
            avatar_tone: AvatarTone::Neutral,
            glyph_scale: GlyphScale::Large,
            is_preferred_focus: false,
        },
    }
}
