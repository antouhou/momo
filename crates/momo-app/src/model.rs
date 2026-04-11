#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShellMode {
    Standalone,
    LayerShell,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShellConfiguration {
    pub mode: ShellMode,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellViewModel {
    pub mode: ShellMode,
    pub output_count: usize,
    pub view_count: usize,
    pub compositor_name: &'static str,
}
