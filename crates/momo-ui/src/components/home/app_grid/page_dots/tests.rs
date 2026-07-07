use daiko::{
    Pos2,
    lyon::{algorithms::hit_test::hit_test_path, lyon_tessellation::FillRule},
};

use super::{
    ACTIVE_PAGE_DOT_WIDTH, PAGE_DOT_ACTIVE_NECK_RATIO, PAGE_DOT_SIZE,
    liquid::{LiquidMorphSpec, build_liquid_morph_path},
};

#[test]
fn liquid_morph_path_builds_a_bridge_between_pages() {
    let path = build_liquid_morph_path(
        LiquidMorphSpec {
            from_center_x: 10.0,
            to_center_x: 38.0,
            from_width: ACTIVE_PAGE_DOT_WIDTH,
            to_width: ACTIVE_PAGE_DOT_WIDTH,
            height: PAGE_DOT_SIZE,
            top_y: 0.0,
            neck_ratio: PAGE_DOT_ACTIVE_NECK_RATIO,
        },
        0.5,
    );

    assert!(hit_test_path(
        &Pos2::new(24.0, PAGE_DOT_SIZE / 2.0),
        &path,
        FillRule::EvenOdd,
        0.1,
    ));
}
