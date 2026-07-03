mod style;

use crate::components::login_screen::profile_tile::style::{
    avatar_style, avatar_text_style, label_text_style, tile_style,
};
use crate::components::login_screen::state::{
    GreeterState, GreeterView, ProfileAction, UserProfile,
};
use daiko::Element;
use daiko::component::{Component, ComponentContext};
use daiko::navigation::FocusOrigin;
use daiko::state_management::StateHandle;
use daiko::widgets::text::Text;

#[derive(Clone)]
pub(super) struct ProfileTile {
    action: ProfileAction,
    greeter_state: StateHandle<GreeterState>,
}

impl ProfileTile {
    pub(super) fn new(action: ProfileAction, greeter_state: StateHandle<GreeterState>) -> Self {
        Self {
            action,
            greeter_state,
        }
    }

    fn activate(&self) {
        match self.action {
            ProfileAction::Login(profile) => {
                println!("Selected user {}", profile.name());
                self.greeter_state.write().view = GreeterView::Credentials(profile);
            }
            ProfileAction::AddUser => println!("Pressed add user button"),
        }
    }
}

impl Component for ProfileTile {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let mut pointer = ctx.pointer();
        let focusable = ctx.focusable();
        let tag = self.action.tag();

        focusable.set_preferred_focus(matches!(
            self.action,
            ProfileAction::Login(UserProfile::Anton)
        ));

        if pointer.just_pressed() {
            focusable.request_focus(FocusOrigin::Pointer);
        }

        if pointer.just_pressed() || focusable.just_activated() {
            self.activate();
        }

        let is_highlighted = pointer.is_hovering() || focusable.is_focus_visible();

        Element::new()
            .with_tag(tag)
            .with_style(tile_style(ctx, is_highlighted))
            .with_content(
                Element::new()
                    .with_style(avatar_style(ctx, self.action, is_highlighted))
                    .with_content(
                        Text::new(self.action.glyph()).with_style(avatar_text_style(self.action)),
                    ),
            )
            .with_content(
                Text::new(self.action.label()).with_style(label_text_style(is_highlighted)),
            )
    }
}
