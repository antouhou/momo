use super::{ROUND_ICON_BUTTON_ICON_SIZE, ROUND_ICON_BUTTON_SIZE, RoundIconButton};
use daiko::{
    Element, Vec2,
    component::{Component, ComponentContext},
    testing::{SimpleTestApp, TestRunner},
};

const TEST_ICON: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20"><path fill="currentColor" d="M0 0h20v20H0z"/></svg>"#;

#[derive(Clone, Copy)]
struct RoundIconButtonProbe;

impl Component for RoundIconButtonProbe {
    fn to_element(&self, context: &mut ComponentContext) -> Element {
        let activation_count = context.use_local_state(|| 0_u8);
        let button = RoundIconButton::new(context, TEST_ICON).with_tag("shared-round-icon-button");
        if button.activated() {
            *activation_count.write() += 1;
        }

        Element::new()
            .with_tag(format!("activation-count-{}", *activation_count.read()))
            .with_content(button)
    }
}

#[test]
fn round_icon_button_uses_shared_metrics_and_emits_activation() {
    let mut runner = TestRunner::new(SimpleTestApp::new(
        Element::new().with_content(RoundIconButtonProbe),
    ));
    runner.set_viewport_size(100.0, 100.0);
    runner.run_frame();

    let (button_position, button_size) = runner.get_element_bounds("shared-round-icon-button");
    assert_eq!(button_size, Vec2::splat(ROUND_ICON_BUTTON_SIZE));

    let button_id = runner
        .tree()
        .elements()
        .find_map(|(element_id, element)| {
            (element.tag() == Some("shared-round-icon-button")).then_some(element_id)
        })
        .expect("round icon button should be rendered");
    let icon_id = runner.tree().children_ids(&button_id)[0];
    let icon_layout = runner
        .tree()
        .layout(&icon_id)
        .expect("icon should have a layout");
    let expected_icon_offset = (ROUND_ICON_BUTTON_SIZE - ROUND_ICON_BUTTON_ICON_SIZE as f32) * 0.5;
    assert_eq!(
        icon_layout.position_absolute,
        button_position + Vec2::splat(expected_icon_offset)
    );
    assert_eq!(
        icon_layout.size,
        Vec2::splat(ROUND_ICON_BUTTON_ICON_SIZE as f32)
    );

    runner.click_element("shared-round-icon-button");
    runner.run_frame();
    assert!(runner.find_element_by_tag("activation-count-1").is_some());
}
