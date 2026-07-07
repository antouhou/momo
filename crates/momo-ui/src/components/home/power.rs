use crate::components::home::model::HOME_POWER_HANDLE_ID;
use daiko::{AppContext, Id, component::ComponentContext};
use system_control::PowerHandle;

pub(crate) fn initialize_power_state(app_context: &mut AppContext, power_handle: PowerHandle) {
    app_context.peek_global_state(Id::new(HOME_POWER_HANDLE_ID), move || power_handle);
}

pub(crate) fn power_handle(ctx: &mut ComponentContext) -> PowerHandle {
    ctx.use_global_state(Id::new(HOME_POWER_HANDLE_ID), || -> PowerHandle {
        panic!("Power handle must be initialized before quick settings render")
    })
    .read()
    .clone()
}
