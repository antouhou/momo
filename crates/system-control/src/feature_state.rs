#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FeatureState<T, UnsupportedReason, UnavailableReason> {
    Loading,
    Unsupported(UnsupportedReason),
    Unavailable(UnavailableReason),
    Ready(T),
}
