use daiko::{AppContext, Id, component::ComponentContext};
use system_control::SessionHandle;
use crate::components::home::model::HOME_SESSION_HANDLE_ID;

pub(crate) fn initialize_session_state(
    app_context: &mut AppContext,
    session_handle: SessionHandle,
) {
    app_context.peek_global_state(Id::new(HOME_SESSION_HANDLE_ID), move || session_handle);
}

pub(crate) fn session_handle(ctx: &mut ComponentContext) -> SessionHandle {
    ctx.use_global_state(Id::new(HOME_SESSION_HANDLE_ID), || -> SessionHandle {
        panic!("Session handle must be initialized before quick settings render")
    })
    .read()
    .clone()
}
