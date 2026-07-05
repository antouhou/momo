#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum GreeterView {
    Profiles,
    Credentials { user_index: usize },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct GreeterState {
    pub(super) view: GreeterView,
}

impl Default for GreeterState {
    fn default() -> Self {
        Self {
            view: GreeterView::Profiles,
        }
    }
}
