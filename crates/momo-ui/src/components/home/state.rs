use daiko::{Id, channel::Channel, component::ComponentContext, state_management::StateHandle};

const HOME_VIEW_REQUEST_CHANNEL_ID: &str = "momo_home_view_request_channel";
const HOME_VIEW_STATE_ID: &str = "momo_home_view_state";

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum HomeView {
    #[default]
    Apps,
    Overview,
}

fn use_home_view_state_handle(ctx: &mut ComponentContext) -> StateHandle<HomeView> {
    ctx.use_shared_state(Id::new(HOME_VIEW_STATE_ID), HomeView::default)
}

/// The state is not meant to be written directly to; Only the home component itself is meant
/// to manipulate it based on the requests it receives from the channel
pub(crate) fn use_home_view(ctx: &mut ComponentContext) -> HomeView {
    *use_home_view_state_handle(ctx).read()
}

/// This method is meant to be used only by the home component. Do not change the visibility
/// of this function or try to access the state in any other way.
pub(super) fn update_active_home_view_from_requests(ctx: &mut ComponentContext) -> HomeView {
    let home_view_state = use_home_view_state_handle(ctx);
    let home_view_request_channel = use_home_view_request_channel(ctx);

    if let Some(requested_home_view) = home_view_request_channel.iter().next() {
        *home_view_state.write() = requested_home_view;
    }

    *home_view_state.read()
}

pub(crate) fn use_home_view_request_channel(ctx: &mut ComponentContext) -> Channel<HomeView> {
    ctx.use_channel_with_id(HOME_VIEW_REQUEST_CHANNEL_ID)
}
