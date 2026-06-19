use daiko::Id;
use daiko::component::ComponentContext;
use daiko::state_management::StateHandle;

const HOME_APP_GRID_PAGE_STATE_ID: &str = "momo_home_app_grid_page_state";

#[derive(Default)]
pub(super) struct AppGridState {
    pub(crate) page_count: usize,
    pub(crate) active_page: usize,
}

pub(super) fn app_grid_state_handle(ctx: &mut ComponentContext) -> StateHandle<AppGridState> {
    ctx.use_shared_state(Id::new(HOME_APP_GRID_PAGE_STATE_ID), AppGridState::default)
}
