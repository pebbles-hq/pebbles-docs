//! Global app state — cross-window signals plus the docs' thin adapter over the
//! framework router.
//!
//! Navigation runs through [`pebbles::core::router`], so on the web the Rust route
//! IS the browser URL (deep links + Back/Forward), and on desktop/mobile it is the
//! same router held in memory. The docs address screens by a short **id**
//! (`"buttons"`); [`current_route`] derives that id from the router path (reactive —
//! reading it subscribes the caller), and [`navigate`] maps an id back to a path.
//! The other globals here (counter, channel, doc/learn sections, menu) show
//! SolidJS's headline feature: the **same** `create_signal` primitive, created once
//! at app scope (via [`init`]) and shared across every window without prop-drilling.

use std::cell::RefCell;

use pebbles::prelude::*;

thread_local! {
    static COUNTER: RefCell<Option<Signal<i32>>> = const { RefCell::new(None) };
    static PING: RefCell<Option<Channel<String>>> = const { RefCell::new(None) };
    static DOC_SECTION: RefCell<Option<Signal<String>>> = const { RefCell::new(None) };
    static LEARN_SECTION: RefCell<Option<Signal<String>>> = const { RefCell::new(None) };
    static MENU_OPEN: RefCell<Option<Signal<bool>>> = const { RefCell::new(None) };
}

/// Create the global app-scope state (call once, before any component renders, so
/// it's owned globally — and thus shared across windows — rather than by a component).
pub fn init() {
    let _ = counter();
    let _ = ping();
    let _ = doc_section();
    let _ = learn_section();
    let _ = menu_open();
    // Dev hook: GALLERY_ROUTE=<route-id> opens straight on a screen (desktop). On
    // web the framework router seeds itself from the URL, so this is overridden by a
    // deep link there — which is exactly right.
    if let Ok(r) = std::env::var("GALLERY_ROUTE") {
        pebbles::core::router::replace(&id_to_path(&r));
    }
}

/// The docs address a route by a short **id** (`"buttons"`, `"landing"`); the
/// framework router addresses a **path** (`"/buttons"`, `"/"`). These map between
/// them so the browser URL reads naturally and the landing page is the root.
fn id_to_path(id: &str) -> String {
    if id == LANDING || id.is_empty() {
        "/".to_string()
    } else {
        format!("/{id}")
    }
}
fn path_to_id(path: &str) -> String {
    let p = path.trim_start_matches('/');
    if p.is_empty() {
        LANDING.to_string()
    } else {
        p.to_string()
    }
}

/// The current route **id**, derived from the framework router (reactive — reading
/// it subscribes the caller, so it re-renders on navigation, incl. browser
/// Back/Forward on web).
pub fn current_route() -> String {
    path_to_id(&pebbles::core::router::path())
}

/// A counter shared across every window (the same signal, read by capture).
pub fn counter() -> Signal<i32> {
    COUNTER.with(|cell| {
        let mut cell = cell.borrow_mut();
        if cell.is_none() {
            *cell = Some(create_signal(0));
        }
        cell.unwrap()
    })
}

/// A typed cross-window message channel.
pub fn ping() -> Channel<String> {
    PING.with(|cell| {
        let mut cell = cell.borrow_mut();
        if cell.is_none() {
            *cell = Some(channel());
        }
        cell.unwrap()
    })
}

/// Navigate to a route.
pub fn navigate(to: &str) {
    pebbles::core::log::info(pebbles::core::log::Cat::Nav, format!("navigate → {to}"));
    // Through the framework router: on web this also pushes the browser URL (deep
    // links + Back/Forward); on desktop it is in-memory history.
    pebbles::core::router::navigate(&id_to_path(to));
}

/// The two top-level, non-widget routes. Everything else in [`NAV`] is a widget
/// screen rendered in the docs sidenav + content layout.
///
/// - [`LANDING`] — the marketing landing page (the front door; no docs chrome).
/// - [`DOCS`] — the searchable component index (categories → widget grid).
pub const LANDING: &str = "landing";
/// See [`LANDING`].
pub const DOCS: &str = "docs";
/// The guided tutorial hub (teaching path), see [`LANDING`].
pub const LEARN: &str = "learn";
/// The showcase — apps built with Pebbles, see [`LANDING`].
pub const SHOWCASE: &str = "showcase";

/// Go to the marketing landing page.
pub fn to_landing() {
    navigate(LANDING);
}

/// The docs hub's active section id (which sidenav page is open). Global so the top
/// nav can open the hub *at* a section, not just its front page.
pub fn doc_section() -> Signal<String> {
    DOC_SECTION.with(|cell| {
        let mut cell = cell.borrow_mut();
        if cell.is_none() {
            *cell = Some(create_signal(String::from("introduction")));
        }
        cell.unwrap()
    })
}

/// Open the docs hub on a specific section (sets the section, then navigates).
pub fn open_docs(section: &str) {
    doc_section().set(section.to_string());
    navigate(DOCS);
}

/// Go to the docs hub on its first page (Introduction).
pub fn to_docs() {
    open_docs("introduction");
}

/// Go to the docs hub on the Components catalog.
pub fn to_components() {
    open_docs("catalog");
}

/// The learn hub's active lesson id.
pub fn learn_section() -> Signal<String> {
    LEARN_SECTION.with(|cell| {
        let mut cell = cell.borrow_mut();
        if cell.is_none() {
            *cell = Some(create_signal(String::from("welcome")));
        }
        cell.unwrap()
    })
}

/// Open the guided-tutorial hub on a specific lesson (sets the lesson, then navigates).
pub fn open_learn(section: &str) {
    learn_section().set(section.to_string());
    navigate(LEARN);
}

/// Go to the Learn hub on its first lesson.
pub fn to_learn() {
    open_learn("welcome");
}

/// Go to the Showcase — apps built with Pebbles.
pub fn to_showcase() {
    navigate(SHOWCASE);
}

/// Whether the compact (mobile) navigation drawer is open.
pub fn menu_open() -> Signal<bool> {
    MENU_OPEN.with(|cell| {
        let mut cell = cell.borrow_mut();
        if cell.is_none() {
            *cell = Some(create_signal(false));
        }
        cell.unwrap()
    })
}

/// Toggle the mobile navigation drawer.
pub fn toggle_menu() {
    menu_open().update(|o| *o = !*o);
}

/// Close the mobile navigation drawer (called on navigate / scrim tap).
pub fn close_menu() {
    menu_open().set(false);
}

/// A single sidebar entry: (route id, icon, label).
pub type Route = (&'static str, IconData, &'static str);

/// The category group a widget `route` belongs to — the sidebar on a widget screen
/// shows ONLY this group's items (its sibling components), not the whole catalog.
pub fn group_of(route: &str) -> Option<&'static NavGroup> {
    NAV.iter()
        .find(|g| g.routes.iter().any(|(r, _, _)| *r == route))
}

/// A labelled group of routes — the sidebar renders one section per group so
/// components of different categories are never jumbled together.
pub struct NavGroup {
    pub label: &'static str,
    pub routes: &'static [Route],
}

/// The categorized sidebar. Groups mirror the component taxonomy
/// (input / display / layout / navigation) plus foundations.
pub const NAV: &[NavGroup] = &[
    NavGroup {
        label: "GET STARTED",
        routes: &[("overview", tabler::LAYOUT_DASHBOARD, "Overview")],
    },
    NavGroup {
        label: "INPUT",
        routes: &[
            ("buttons", tabler::CLICK, "Buttons"),
            ("button-group", tabler::COLUMNS_2, "Button Group"),
            ("text-fields", tabler::CURSOR_TEXT, "Text Fields"),
            ("date-picker", tabler::CALENDAR_EVENT, "Date Picker"),
            ("select", tabler::CHEVRON_DOWN, "Select & Menus"),
            ("combobox", tabler::SEARCH, "Combobox"),
            ("command", tabler::FILE_CODE, "Command"),
            ("toggles", tabler::TOGGLE_RIGHT, "Toggles"),
            ("radio-group", tabler::CIRCLE_DOT, "Radio Group"),
            ("slider", tabler::ADJUSTMENTS_HORIZONTAL, "Slider"),
            ("dialog", tabler::MESSAGE, "Dialog"),
            ("windows", tabler::APP_WINDOW, "Windows & IPC"),
        ],
    },
    NavGroup {
        label: "DISPLAY",
        routes: &[
            ("badge", tabler::TAG, "Badge"),
            ("chips", tabler::ROSETTE_DISCOUNT_CHECK, "Chip"),
            ("alert", tabler::BELL, "Alert"),
            ("skeleton", tabler::BOX_MULTIPLE, "Skeleton"),
            ("kbd", tabler::KEYBOARD, "Kbd"),
            ("empty", tabler::INBOX, "Empty"),
            ("card", tabler::CREDIT_CARD, "Card"),
            ("avatar", tabler::USER, "Avatar"),
            ("separator", tabler::SEPARATOR_HORIZONTAL, "Separator"),
            ("progress", tabler::GAUGE, "Progress"),
            ("list", tabler::LIST, "List"),
            ("data-table", tabler::TABLE, "Data Table"),
            ("file-explorer", tabler::FOLDERS, "File Explorer"),
            ("typography", tabler::TYPOGRAPHY, "Typography"),
            ("fonts", tabler::PILCROW, "Fonts"),
            ("icons", tabler::SHAPE_3, "Icons"),
            ("images", tabler::PHOTO, "Images"),
        ],
    },
    NavGroup {
        label: "LAYOUT",
        routes: &[
            ("rows", tabler::LAYOUT_ROWS, "Row"),
            ("columns", tabler::COLUMNS_2, "Column"),
            ("stacks", tabler::STACK, "Stack"),
            ("wraps", tabler::TEXT_WRAP, "Wrap"),
            ("boxes", tabler::BOX, "Boxes & Sizing"),
            ("scrolling", tabler::ARROWS_VERTICAL, "Scroll & Motion"),
            ("canvas", tabler::SHAPE_3, "Canvas"),
            ("list-view", tabler::LIST_NUMBERS, "List View"),
            ("grid-view", tabler::LAYOUT_GRID, "Grid View"),
            ("split-view", tabler::COLUMNS_2, "Split View"),
            ("resizable", tabler::LAYOUT_SIDEBAR, "Resizable"),
            ("accordion", tabler::LIST_TREE, "Accordion"),
            ("collapsible", tabler::FOLD, "Collapsible"),
        ],
    },
    NavGroup {
        label: "MOTION & TRANSITIONS",
        routes: &[
            ("animated-container", tabler::WAND, "Animated Container"),
            ("animated-opacity", tabler::WAND, "Animated Opacity"),
            ("animated-scale", tabler::WAND, "Animated Scale"),
            ("animated-rotation", tabler::WAND, "Animated Rotation"),
            ("animated-slide", tabler::ARROWS_VERTICAL, "Animated Slide"),
            ("animated-align", tabler::COLUMNS_2, "Animated Align"),
            ("animated-padding", tabler::BOX, "Animated Padding"),
            ("animated-positioned", tabler::STACK, "Animated Positioned"),
            ("fade-transition", tabler::WAND, "Fade Transition"),
            ("scale-transition", tabler::WAND, "Scale Transition"),
            ("rotation-transition", tabler::WAND, "Rotation Transition"),
            (
                "slide-transition",
                tabler::ARROWS_VERTICAL,
                "Slide Transition",
            ),
            ("size-transition", tabler::BOX, "Size Transition"),
            (
                "positioned-transition",
                tabler::STACK,
                "Positioned Transition",
            ),
            (
                "decorated-box-transition",
                tabler::BRUSH,
                "Decorated Box Transition",
            ),
            ("animated-switcher", tabler::STACK, "Animated Switcher"),
            ("animated-cross-fade", tabler::STACK, "Animated Cross Fade"),
            ("dismissible", tabler::INBOX, "Dismissible"),
            ("animated-list", tabler::LIST_NUMBERS, "Animated List"),
            ("animated-grid", tabler::LAYOUT_GRID, "Animated Grid"),
            ("hero", tabler::WAND, "Hero"),
        ],
    },
    NavGroup {
        label: "DRAG & DROP / POINTER",
        routes: &[
            ("draggable", tabler::ARROWS_VERTICAL, "Draggable"),
            ("drag-target", tabler::INBOX, "Drag Target"),
            (
                "long-press-draggable",
                tabler::ARROWS_VERTICAL,
                "Long Press Draggable",
            ),
            ("ignore-pointer", tabler::BOX, "Ignore Pointer"),
            ("absorb-pointer", tabler::BOX, "Absorb Pointer"),
            (
                "interactive-viewer",
                tabler::LAYOUT_GRID,
                "Interactive Viewer",
            ),
            (
                "reorderable-list-view",
                tabler::LIST_NUMBERS,
                "Reorderable List View",
            ),
        ],
    },
    NavGroup {
        label: "LAYOUT (LONG-TAIL)",
        routes: &[
            ("indexed-stack", tabler::STACK, "Indexed Stack"),
            ("offstage", tabler::BOX, "Offstage"),
            ("visibility", tabler::BOX, "Visibility"),
            ("baseline", tabler::COLUMNS_2, "Baseline"),
            ("rotated-box", tabler::WAND, "Rotated Box"),
            ("unconstrained-box", tabler::BOX, "Unconstrained Box"),
            ("sized-overflow-box", tabler::BOX, "Sized Overflow Box"),
            (
                "fractional-translation",
                tabler::ARROWS_VERTICAL,
                "Fractional Translation",
            ),
            ("table-layout", tabler::LAYOUT_GRID, "Table (layout)"),
            ("custom-single-child", tabler::BOX, "Custom Single Child"),
            (
                "custom-multi-child",
                tabler::LAYOUT_GRID,
                "Custom Multi Child",
            ),
            ("flow", tabler::WAND, "Flow"),
            ("layout-builder", tabler::COLUMNS_2, "Layout Builder"),
        ],
    },
    NavGroup {
        label: "PAINTING & EFFECTS",
        routes: &[
            ("clip-rect", tabler::BOX, "Clip Rect"),
            ("clip-oval", tabler::BOX, "Clip Oval"),
            ("clip-path", tabler::BRUSH, "Clip Path"),
            ("color-filtered", tabler::BRUSH, "Color Filtered"),
            ("shader-mask", tabler::BRUSH, "Shader Mask"),
        ],
    },
    NavGroup {
        label: "INPUT & FORMS (MORE)",
        routes: &[
            ("choice-chip", tabler::INBOX, "Choice Chip"),
            ("filter-chip", tabler::INBOX, "Filter Chip"),
            ("action-chip", tabler::INBOX, "Action Chip"),
            ("stepper", tabler::LIST_NUMBERS, "Stepper"),
            ("selectable-text", tabler::SEARCH, "Selectable Text"),
        ],
    },
    NavGroup {
        label: "MATERIAL STAPLES",
        routes: &[
            ("fab", tabler::PLUS, "Floating Action Button"),
            ("grid-tile", tabler::LAYOUT_GRID, "Grid Tile"),
        ],
    },
    NavGroup {
        label: "ASYNC & A11Y",
        routes: &[
            ("stream-builder", tabler::WAND, "Stream Builder"),
            (
                "semantics-combinators",
                tabler::INFO_CIRCLE,
                "Semantics Combinators",
            ),
        ],
    },
    NavGroup {
        label: "SCAFFOLD & MOBILE",
        routes: &[
            ("media-query", tabler::INFO_CIRCLE, "Media Query"),
            ("safe-area", tabler::BOX, "Safe Area"),
            (
                "orientation-builder",
                tabler::COLUMNS_2,
                "Orientation Builder",
            ),
            ("scaffold-slots", tabler::LAYOUT_GRID, "Scaffold Slots"),
        ],
    },
    NavGroup {
        label: "LONG-TAIL",
        routes: &[
            ("placeholder", tabler::BOX, "Placeholder"),
            ("banner", tabler::INFO_CIRCLE, "Banner"),
            (
                "scroll-notification",
                tabler::ARROWS_VERTICAL,
                "Scroll Notification",
            ),
            ("list-body", tabler::LIST, "List Body"),
            ("list-tiles", tabler::CHECKLIST, "List Tiles"),
            ("draggable-sheet", tabler::LAYOUT_BOTTOMBAR, "Draggable Sheet"),
            ("scaffold-drawer", tabler::LAYOUT_SIDEBAR, "Scaffold Drawer"),
            ("default-text-style", tabler::TYPOGRAPHY, "Default Text Style"),
            ("mobile-runtime", tabler::DEVICE_MOBILE, "Mobile Runtime"),
            ("nested-scroll", tabler::ARROWS_VERTICAL, "Nested Scroll View"),
            ("pressable", tabler::CLICK, "Pressable / Ink"),
        ],
    },
    NavGroup {
        label: "NAVIGATION",
        routes: &[
            ("tabs", tabler::STACK_2, "Tabs"),
            ("breadcrumb", tabler::CHEVRON_RIGHT, "Breadcrumb"),
            ("menubar", tabler::MENU_2, "Menubar"),
            ("pagination", tabler::CHEVRONS_LEFT, "Pagination"),
            ("toolbar", tabler::LAYOUT_DASHBOARD, "Toolbar & Status"),
            ("chrome", tabler::APP_WINDOW, "App Shell"),
            ("routing", tabler::STACK, "Routing"),
        ],
    },
    NavGroup {
        label: "OVERLAYS",
        routes: &[
            ("tooltips", tabler::INFO_CIRCLE, "Tooltip"),
            ("popovers", tabler::ARROW_UP_LEFT, "Popover"),
            ("sheets", tabler::LAYOUT_SIDEBAR_RIGHT_EXPAND, "Sheet & Drawer"),
            ("toasts", tabler::BELL, "Toast"),
            ("context-menu", tabler::POINTER, "Context Menu"),
        ],
    },
    NavGroup {
        label: "FOUNDATIONS",
        routes: &[
            ("colors", tabler::PALETTE, "Colors"),
            ("styling", tabler::BRUSH, "Styling"),
            ("context", tabler::TREE, "Context & Scopes"),
        ],
    },
];
