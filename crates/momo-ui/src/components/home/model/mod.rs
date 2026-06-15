use daiko::animation::{AnimationParameters, transition};
use daiko::component::ComponentContext;
use daiko::navigation::FocusKey;
use daiko::state_management::StateHandle;
use daiko::style::{Color, Transform};
use daiko::{Id, Vec2};
use std::path::PathBuf;
use std::rc::Rc;
use std::time::Duration;

pub(super) const HOME_APP_GRID_PAGE_STATE_ID: &str = "momo_home_app_grid_page_state";
pub(super) const HOME_APP_GRID_FOCUSED_KEY_ID: &str = "momo_home_app_grid_focused_key";
pub(super) const HOME_APP_ENTRIES_STATE_ID: &str = "momo_home_app_entries";
pub(super) const HOME_APP_GRID_SCROLL_ACCUMULATOR_ID: &str =
    "momo_home_app_grid_scroll_accumulator";
pub(super) const HOME_APP_GRID_SMOOTH_OFFSET_ID: &str = "momo_home_app_grid_smooth_offset";
pub(super) const HOME_CLOCK_THREAD_ID: &str = "momo_home_clock_thread_started";
pub(super) const HOME_CLOCK_STATE_ID: &str = "momo_home_clock_text";
pub(super) const HOME_BLUETOOTH_HANDLE_ID: &str = "momo_home_bluetooth_handle";
pub(super) const HOME_BLUETOOTH_OBSERVATION_ID: &str = "momo_home_bluetooth_observation";
pub(super) const HOME_BLUETOOTH_STATE_ID: &str = "momo_home_bluetooth_state";
pub(super) const HOME_BATTERY_HANDLE_ID: &str = "momo_home_battery_handle";
pub(super) const HOME_BATTERY_OBSERVATION_ID: &str = "momo_home_battery_observation";
pub(super) const HOME_BATTERY_STATE_ID: &str = "momo_home_battery_state";
pub(super) const HOME_VOLUME_HANDLE_ID: &str = "momo_home_volume_handle";
pub(super) const HOME_VOLUME_OBSERVATION_ID: &str = "momo_home_volume_observation";
pub(super) const HOME_VOLUME_STATE_ID: &str = "momo_home_volume_state";
pub(super) const HOME_LAUNCH_CHANNEL_ID: &str = "momo_home_launch_channel";
pub(super) const HOME_TOP_ROW_APPS_FOCUS_KEY_ID: &str = "momo_home_top_row_apps";
pub(super) const HOME_TOP_ROW_SETTINGS_FOCUS_KEY_ID: &str = "momo_home_top_row_settings";

pub(super) const SCREEN_PADDING: f32 = 40.0;
pub(super) const SECTION_GAP: f32 = 24.0;
pub(super) const GRID_GAP: f32 = 18.0;
pub(super) const TILE_WIDTH: f32 = 248.0;
pub(super) const TILE_HEIGHT: f32 = 176.0;
pub(super) const TILE_BORDER_RADIUS: f32 = 18.0;
pub(super) const TILE_BORDER_WIDTH: f32 = 2.0;
pub(super) const TILE_FOCUS_SCALE: f32 = 1.05;
pub(super) const TILE_FOCUS_LIFT_Y: f32 = -3.0;
pub(super) const TILE_FOCUS_ANIMATION_DURATION_MS: u64 = 100;
pub(super) const TILE_ICON_SIZE: f32 = 96.0;
pub(super) const TILE_ICON_GLYPH_SIZE: usize = 64;
pub(super) const TILE_PADDING: f32 = 16.0;
pub(super) const TILE_CONTENT_GAP: f32 = 12.0;

pub fn home_top_row_apps_focus_key() -> FocusKey {
    FocusKey::new(HOME_TOP_ROW_APPS_FOCUS_KEY_ID)
}

pub fn home_top_row_settings_focus_key() -> FocusKey {
    FocusKey::new(HOME_TOP_ROW_SETTINGS_FOCUS_KEY_ID)
}

#[derive(Clone)]
pub(super) enum AppIcon {
    BuiltIn(BuiltInAppIcon),
    #[expect(dead_code)]
    File(PathBuf),
}

#[derive(Clone, Copy)]
pub(super) enum BuiltInAppIcon {
    LiveTv,
    Movies,
    Music,
    Photos,
    Browser,
    Settings,
    Store,
    Files,
    Games,
    News,
    Weather,
    Sports,
    Podcasts,
    Calendar,
    Mail,
    Camera,
    Terminal,
    Sleep,
    Search,
    Kids,
    Stream,
    Radio,
    Books,
    Fitness,
    Calls,
    Travel,
    Recipes,
    Security,
}

#[derive(Clone)]
pub(super) enum AppLaunch {
    Mock,
}

#[derive(Clone)]
pub(super) struct AppEntry {
    pub id: Rc<String>,
    pub name: Rc<String>,
    pub icon: AppIcon,
    pub launch: AppLaunch,
    pub accent: [u8; 3],
}

impl AppEntry {
    pub(super) fn id(&self) -> &str {
        self.id.as_str()
    }

    pub(super) fn name(&self) -> &str {
        self.name.as_str()
    }
}

#[derive(Clone)]
pub(super) struct LaunchRequest {
    pub app: Rc<AppEntry>,
    pub position: Vec2,
    pub size: Vec2,
    pub icon_position: Vec2,
    pub icon_size: Vec2,
}

pub(super) fn app_entries(ctx: &mut ComponentContext) -> StateHandle<Vec<Rc<AppEntry>>> {
    ctx.use_global_state(Id::new(HOME_APP_ENTRIES_STATE_ID), default_app_entries)
}

fn default_app_entries() -> Vec<Rc<AppEntry>> {
    MOCK_APP_SPECS
        .iter()
        .copied()
        .map(MockAppSpec::to_entry)
        .collect()
}

fn app_text(text: &'static str) -> Rc<String> {
    Rc::new(text.to_owned())
}

#[derive(Clone, Copy)]
pub(super) struct MockAppSpec {
    pub id: &'static str,
    pub name: &'static str,
    pub icon: BuiltInAppIcon,
    pub accent: [u8; 3],
}

impl MockAppSpec {
    fn to_entry(self) -> Rc<AppEntry> {
        Rc::new(AppEntry {
            id: app_text(self.id),
            name: app_text(self.name),
            icon: AppIcon::BuiltIn(self.icon),
            launch: AppLaunch::Mock,
            accent: self.accent,
        })
    }
}

pub(super) const MOCK_APP_SPECS: [MockAppSpec; 28] = [
    MockAppSpec {
        id: "live-tv",
        name: "Live TV",
        icon: BuiltInAppIcon::LiveTv,
        accent: [60, 133, 246],
    },
    MockAppSpec {
        id: "movies",
        name: "Movies",
        icon: BuiltInAppIcon::Movies,
        accent: [255, 110, 64],
    },
    MockAppSpec {
        id: "music",
        name: "Music",
        icon: BuiltInAppIcon::Music,
        accent: [76, 175, 80],
    },
    MockAppSpec {
        id: "photos",
        name: "Photos",
        icon: BuiltInAppIcon::Photos,
        accent: [255, 193, 7],
    },
    MockAppSpec {
        id: "browser",
        name: "Browser",
        icon: BuiltInAppIcon::Browser,
        accent: [0, 188, 212],
    },
    MockAppSpec {
        id: "settings",
        name: "Settings",
        icon: BuiltInAppIcon::Settings,
        accent: [171, 71, 188],
    },
    MockAppSpec {
        id: "store",
        name: "Store",
        icon: BuiltInAppIcon::Store,
        accent: [244, 67, 54],
    },
    MockAppSpec {
        id: "files",
        name: "Files",
        icon: BuiltInAppIcon::Files,
        accent: [121, 85, 72],
    },
    MockAppSpec {
        id: "games",
        name: "Games",
        icon: BuiltInAppIcon::Games,
        accent: [233, 30, 99],
    },
    MockAppSpec {
        id: "news",
        name: "News",
        icon: BuiltInAppIcon::News,
        accent: [96, 125, 139],
    },
    MockAppSpec {
        id: "weather",
        name: "Weather",
        icon: BuiltInAppIcon::Weather,
        accent: [3, 169, 244],
    },
    MockAppSpec {
        id: "sports",
        name: "Sports",
        icon: BuiltInAppIcon::Sports,
        accent: [139, 195, 74],
    },
    MockAppSpec {
        id: "podcasts",
        name: "Podcasts",
        icon: BuiltInAppIcon::Podcasts,
        accent: [255, 87, 34],
    },
    MockAppSpec {
        id: "calendar",
        name: "Calendar",
        icon: BuiltInAppIcon::Calendar,
        accent: [63, 81, 181],
    },
    MockAppSpec {
        id: "mail",
        name: "Mail",
        icon: BuiltInAppIcon::Mail,
        accent: [0, 150, 136],
    },
    MockAppSpec {
        id: "camera",
        name: "Camera",
        icon: BuiltInAppIcon::Camera,
        accent: [255, 152, 0],
    },
    MockAppSpec {
        id: "terminal",
        name: "Terminal",
        icon: BuiltInAppIcon::Terminal,
        accent: [158, 158, 158],
    },
    MockAppSpec {
        id: "sleep",
        name: "Sleep",
        icon: BuiltInAppIcon::Sleep,
        accent: [103, 58, 183],
    },
    MockAppSpec {
        id: "search",
        name: "Search",
        icon: BuiltInAppIcon::Search,
        accent: [0, 121, 107],
    },
    MockAppSpec {
        id: "kids",
        name: "Kids",
        icon: BuiltInAppIcon::Kids,
        accent: [255, 64, 129],
    },
    MockAppSpec {
        id: "stream",
        name: "Stream",
        icon: BuiltInAppIcon::Stream,
        accent: [33, 150, 243],
    },
    MockAppSpec {
        id: "radio",
        name: "Radio",
        icon: BuiltInAppIcon::Radio,
        accent: [205, 220, 57],
    },
    MockAppSpec {
        id: "books",
        name: "Books",
        icon: BuiltInAppIcon::Books,
        accent: [141, 110, 99],
    },
    MockAppSpec {
        id: "fitness",
        name: "Fitness",
        icon: BuiltInAppIcon::Fitness,
        accent: [255, 87, 34],
    },
    MockAppSpec {
        id: "calls",
        name: "Calls",
        icon: BuiltInAppIcon::Calls,
        accent: [76, 175, 80],
    },
    MockAppSpec {
        id: "travel",
        name: "Travel",
        icon: BuiltInAppIcon::Travel,
        accent: [63, 81, 181],
    },
    MockAppSpec {
        id: "recipes",
        name: "Recipes",
        icon: BuiltInAppIcon::Recipes,
        accent: [255, 193, 7],
    },
    MockAppSpec {
        id: "security",
        name: "Security",
        icon: BuiltInAppIcon::Security,
        accent: [96, 125, 139],
    },
];

pub(super) fn columns_for_width(width: f32) -> usize {
    let slot = TILE_WIDTH + GRID_GAP;
    ((width + GRID_GAP) / slot).floor().max(1.0) as usize
}

pub(super) fn rows_for_height(height: f32) -> usize {
    let slot = TILE_HEIGHT + GRID_GAP;
    ((height + GRID_GAP) / slot).floor().max(1.0) as usize
}

pub(super) fn color(rgb: [u8; 3]) -> Color {
    Color::from_rgb(rgb[0], rgb[1], rgb[2])
}

pub(super) fn tile_icon_origin() -> Vec2 {
    Vec2::new(
        (TILE_WIDTH - TILE_ICON_SIZE) / 2.0,
        TILE_PADDING + TILE_BORDER_WIDTH,
    )
}

pub(super) fn tile_focus_transform(
    size: Vec2,
    is_focused: bool,
    ctx: &mut ComponentContext,
) -> Transform {
    let scale = transition(
        if is_focused { TILE_FOCUS_SCALE } else { 1.0 },
        AnimationParameters::default()
            .with_duration(Duration::from_millis(TILE_FOCUS_ANIMATION_DURATION_MS))
            .to_transition_options(),
        ctx,
    );
    let lift_y = transition(
        if is_focused { TILE_FOCUS_LIFT_Y } else { 0.0 },
        AnimationParameters::default()
            .with_duration(Duration::from_millis(TILE_FOCUS_ANIMATION_DURATION_MS))
            .to_transition_options(),
        ctx,
    );

    Transform::new()
        .with_origin(size.x * 0.5, size.y * 0.5)
        .then_scale(scale, scale)
        .then_translate(0.0, lift_y)
}

pub(super) fn transformed_local_rect(
    position: Vec2,
    transform: &Transform,
    local_position: Vec2,
    size: Vec2,
) -> (Vec2, Vec2) {
    let effective_transform = transform
        .clone()
        .with_position_relative_to_parent(position.x, position.y)
        .compose_2(&Transform::new());
    let corners = [
        effective_transform.transform_local_point2d_to_world(local_position.x, local_position.y),
        effective_transform
            .transform_local_point2d_to_world(local_position.x + size.x, local_position.y),
        effective_transform
            .transform_local_point2d_to_world(local_position.x, local_position.y + size.y),
        effective_transform
            .transform_local_point2d_to_world(local_position.x + size.x, local_position.y + size.y),
    ];
    let (min_x, max_x) = corners
        .iter()
        .map(|(x, _)| *x)
        .fold((f32::INFINITY, f32::NEG_INFINITY), |(min_x, max_x), x| {
            (min_x.min(x), max_x.max(x))
        });
    let (min_y, max_y) = corners
        .iter()
        .map(|(_, y)| *y)
        .fold((f32::INFINITY, f32::NEG_INFINITY), |(min_y, max_y), y| {
            (min_y.min(y), max_y.max(y))
        });

    (
        Vec2::new(min_x, min_y),
        Vec2::new(max_x - min_x, max_y - min_y),
    )
}
