#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GreeterLaunchMode {
    Shell,
    Standalone,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GreeterLaunchConfiguration {
    pub mode: GreeterLaunchMode,
    greeter_arguments: Vec<String>,
}

impl GreeterLaunchConfiguration {
    pub fn from_env() -> Self {
        Self::from_args(std::env::args().skip(1))
    }

    pub fn from_args(arguments: impl IntoIterator<Item = String>) -> Self {
        let mut mode = GreeterLaunchMode::Shell;
        let mut greeter_arguments = Vec::new();

        for argument in arguments {
            if argument == "--standalone-test" {
                mode = GreeterLaunchMode::Standalone;
            } else {
                greeter_arguments.push(argument);
            }
        }

        Self {
            mode,
            greeter_arguments,
        }
    }

    pub fn into_greeter_arguments(self) -> Vec<String> {
        self.greeter_arguments
    }
}

#[cfg(test)]
mod tests;
