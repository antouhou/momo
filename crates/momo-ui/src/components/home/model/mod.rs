use daiko::style::Color;

pub(super) const HOME_SCROLLABLE_ID: &str = "momo_home_apps_scrollable";
pub(super) const HOME_CLOCK_THREAD_ID: &str = "momo_home_clock_thread_started";
pub(super) const HOME_CLOCK_STATE_ID: &str = "momo_home_clock_text";

pub(super) const SCREEN_PADDING: f32 = 40.0;
pub(super) const SECTION_GAP: f32 = 24.0;
pub(super) const GRID_GAP: f32 = 18.0;
pub(super) const TILE_WIDTH: f32 = 248.0;
pub(super) const TILE_HEIGHT: f32 = 176.0;

#[derive(Clone, Copy)]
pub(super) struct MockApp {
    pub id: &'static str,
    pub name: &'static str,
    pub subtitle: &'static str,
    pub badge: &'static str,
    pub accent: [u8; 3],
}

pub(super) const MOCK_APPS: [MockApp; 28] = [
    MockApp {
        id: "live-tv",
        name: "Live TV",
        subtitle: "Channels",
        badge: "TV",
        accent: [60, 133, 246],
    },
    MockApp {
        id: "movies",
        name: "Movies",
        subtitle: "Library",
        badge: "MV",
        accent: [255, 110, 64],
    },
    MockApp {
        id: "music",
        name: "Music",
        subtitle: "Albums",
        badge: "MU",
        accent: [76, 175, 80],
    },
    MockApp {
        id: "photos",
        name: "Photos",
        subtitle: "Moments",
        badge: "PH",
        accent: [255, 193, 7],
    },
    MockApp {
        id: "browser",
        name: "Browser",
        subtitle: "Web",
        badge: "WB",
        accent: [0, 188, 212],
    },
    MockApp {
        id: "settings",
        name: "Settings",
        subtitle: "System",
        badge: "ST",
        accent: [171, 71, 188],
    },
    MockApp {
        id: "store",
        name: "Store",
        subtitle: "Apps",
        badge: "AP",
        accent: [244, 67, 54],
    },
    MockApp {
        id: "files",
        name: "Files",
        subtitle: "Storage",
        badge: "FL",
        accent: [121, 85, 72],
    },
    MockApp {
        id: "games",
        name: "Games",
        subtitle: "Arcade",
        badge: "GM",
        accent: [233, 30, 99],
    },
    MockApp {
        id: "news",
        name: "News",
        subtitle: "Today",
        badge: "NW",
        accent: [96, 125, 139],
    },
    MockApp {
        id: "weather",
        name: "Weather",
        subtitle: "Forecast",
        badge: "WX",
        accent: [3, 169, 244],
    },
    MockApp {
        id: "sports",
        name: "Sports",
        subtitle: "Scores",
        badge: "SP",
        accent: [139, 195, 74],
    },
    MockApp {
        id: "podcasts",
        name: "Podcasts",
        subtitle: "Listen",
        badge: "PC",
        accent: [255, 87, 34],
    },
    MockApp {
        id: "calendar",
        name: "Calendar",
        subtitle: "Schedule",
        badge: "CL",
        accent: [63, 81, 181],
    },
    MockApp {
        id: "mail",
        name: "Mail",
        subtitle: "Inbox",
        badge: "ML",
        accent: [0, 150, 136],
    },
    MockApp {
        id: "camera",
        name: "Camera",
        subtitle: "Capture",
        badge: "CM",
        accent: [255, 152, 0],
    },
    MockApp {
        id: "terminal",
        name: "Terminal",
        subtitle: "Tools",
        badge: "SH",
        accent: [158, 158, 158],
    },
    MockApp {
        id: "sleep",
        name: "Sleep",
        subtitle: "Power",
        badge: "ZZ",
        accent: [103, 58, 183],
    },
    MockApp {
        id: "search",
        name: "Search",
        subtitle: "Discover",
        badge: "SR",
        accent: [0, 121, 107],
    },
    MockApp {
        id: "kids",
        name: "Kids",
        subtitle: "Family",
        badge: "KD",
        accent: [255, 64, 129],
    },
    MockApp {
        id: "stream",
        name: "Stream",
        subtitle: "Now",
        badge: "ST",
        accent: [33, 150, 243],
    },
    MockApp {
        id: "radio",
        name: "Radio",
        subtitle: "Stations",
        badge: "RD",
        accent: [205, 220, 57],
    },
    MockApp {
        id: "books",
        name: "Books",
        subtitle: "Read",
        badge: "BK",
        accent: [141, 110, 99],
    },
    MockApp {
        id: "fitness",
        name: "Fitness",
        subtitle: "Workout",
        badge: "FT",
        accent: [255, 87, 34],
    },
    MockApp {
        id: "calls",
        name: "Calls",
        subtitle: "Contacts",
        badge: "CL",
        accent: [76, 175, 80],
    },
    MockApp {
        id: "travel",
        name: "Travel",
        subtitle: "Trips",
        badge: "TR",
        accent: [63, 81, 181],
    },
    MockApp {
        id: "recipes",
        name: "Recipes",
        subtitle: "Kitchen",
        badge: "RC",
        accent: [255, 193, 7],
    },
    MockApp {
        id: "security",
        name: "Security",
        subtitle: "Devices",
        badge: "SC",
        accent: [96, 125, 139],
    },
];

pub(super) fn columns_for_width(width: f32) -> usize {
    let slot = TILE_WIDTH + GRID_GAP;
    ((width + GRID_GAP) / slot).floor().max(1.0) as usize
}

pub(super) fn color(rgb: [u8; 3]) -> Color {
    Color::from_rgb(rgb[0], rgb[1], rgb[2])
}
