use daiko::hot_reloading::DynApp;
pub use momo_greeter_lib::create_greeter;

/// For the hot-reloading system to work, the function must have this exact name and signature.
/// The app needs to be wrapped in DynApp. For production builds, you can create the app directly in
/// main.rs and run it without hot-reloading.
#[unsafe(no_mangle)]
pub fn create_app() -> DynApp {
    let greeter = create_greeter(std::env::args().skip(1))
        .expect("failed to initialize system control services");
    DynApp::new(greeter)
}
