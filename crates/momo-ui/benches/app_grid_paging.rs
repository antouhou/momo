use std::{
    hint::black_box,
    time::{Duration, Instant},
};
use criterion::{Criterion, criterion_group, criterion_main};
use daiko::{
    Pos2, Vec2,
    integration::input::{InputEvent, InputEventModifiers},
    testing::TestRunner,
};
use momo_ui::benchmark_support::AppGridBenchmarkApp;

const VIEWPORT_WIDTH: f32 = 1280.0;
const VIEWPORT_HEIGHT: f32 = 720.0;
const APP_COUNT: usize = 72;
const PAGING_FRAMES: usize = 12;

fn app_grid_paging_bench(criterion: &mut Criterion) {
    criterion.bench_function("app_grid/frame_during_paging", |bencher| {
        bencher.iter_custom(|iterations| {
            let mut total_average_frame_time = Duration::ZERO;
            for _ in 0..iterations {
                let mut runner = prepared_runner();
                let start = Instant::now();
                for _ in 0..PAGING_FRAMES {
                    runner.run_frame();
                }
                total_average_frame_time += start.elapsed() / PAGING_FRAMES as u32;
                black_box(runner);
            }
            total_average_frame_time
        });
    });
}

fn prepared_runner() -> TestRunner<AppGridBenchmarkApp> {
    let mut runner = TestRunner::new(AppGridBenchmarkApp::new(APP_COUNT));
    runner.set_viewport_size(VIEWPORT_WIDTH, VIEWPORT_HEIGHT);

    for _ in 0..3 {
        runner.run_frame();
    }

    let (tile_position, tile_size) = runner.get_element_bounds("benchmark-app-0");
    runner.move_pointer_to(Pos2::new(
        tile_position.x + tile_size.x * 0.5,
        tile_position.y + tile_size.y * 0.5,
    ));
    runner.run_frame();

    runner
        .app_runner_mut()
        .context
        .add_input_event(InputEvent::scroll(
            Vec2::new(0.0, 10.0),
            InputEventModifiers::default(),
            Instant::now(),
        ));
    runner.run_frame();

    runner
}

criterion_group!(benches, app_grid_paging_bench);
criterion_main!(benches);
