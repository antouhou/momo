use super::LoginScreen;
use daiko::navigation::{FocusKey, FocusOrigin};
use daiko::testing::TestRunner;
use daiko::{App, AppContext, Vec2};

struct LoginScreenTestApp;

impl App for LoginScreenTestApp {
    type RootComponent = LoginScreen;

    fn create(&mut self, _ctx: &mut AppContext) -> Self::RootComponent {
        LoginScreen::for_testing()
    }

    fn stop(&mut self, _ctx: &mut AppContext) {}
}

fn test_runner() -> TestRunner<LoginScreenTestApp> {
    let mut runner = TestRunner::new(LoginScreenTestApp);
    runner.set_viewport_size(1280.0, 720.0);
    runner.run_frame();
    runner
}

#[test]
fn root_matches_the_viewport() {
    let runner = test_runner();

    runner.assert_element_bounds(
        "login-screen-root",
        Vec2::new(0.0, 0.0),
        Vec2::new(1280.0, 720.0),
    );
}

#[test]
fn directional_navigation_moves_between_profiles() {
    let mut runner = test_runner();

    runner.focus_element_by_key(FocusKey::new("profile-anton"), FocusOrigin::Navigation);
    runner.navigate_right();
    runner.assert_focused("profile-maya");
    runner.navigate_right();
    runner.assert_focused("profile-guest");
}

#[test]
fn activating_a_profile_opens_the_credential_panel() {
    let mut runner = test_runner();

    runner.focus_element_by_key(FocusKey::new("profile-anton"), FocusOrigin::Navigation);
    runner.press_confirm();
    runner.run_frame();

    assert!(runner.find_element_by_tag("credential-panel").is_some());
    assert!(runner.find_element_by_tag("login-submit").is_some());
}

#[test]
fn back_action_returns_to_the_profile_picker() {
    let mut runner = test_runner();

    runner.click_element("profile-anton");
    runner.run_frame();
    runner.click_element("login-back");
    runner.run_frame();

    assert!(runner.find_element_by_tag("profile-picker").is_some());
}

#[test]
fn power_action_is_always_available() {
    let runner = test_runner();

    assert!(runner.find_element_by_tag("power-button").is_some());
}
