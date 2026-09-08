//! The **Learn** hub — a guided, hands-on curriculum (React/Solid/Flutter "Learn"
//! style): a lesson sidenav on the left and a teaching content pane on the right.
//! Where the Docs hub is reference-shaped, Learn is a path — read top to bottom and
//! you build a real app. It reuses the docs hub's sidenav + content builders.

use pebbles::prelude::*;

use crate::screens::docs::{DocGroup, code, h2, p, prose, sidenav};

/// The lesson path. Each entry is `(id, title)`; groups are chapters.
const LEARN: &[DocGroup] = &[
    DocGroup {
        label: "START HERE",
        items: &[
            ("welcome", "Welcome"),
            ("thinking", "Thinking in Pebbles"),
            ("first-app", "Your First App"),
        ],
    },
    DocGroup {
        label: "TUTORIAL · TODO APP",
        items: &[
            ("todo-setup", "1 · Set Up the App"),
            ("todo-list", "2 · Show a List"),
            ("todo-add", "3 · Add Items"),
            ("todo-toggle", "4 · Toggle Done"),
            ("todo-derive", "5 · Counts & Filters"),
            ("todo-style", "6 · Make It Pretty"),
        ],
    },
    DocGroup {
        label: "GO FURTHER",
        items: &[
            ("sharing-state", "Sharing State"),
            ("screens", "Multiple Screens"),
            ("ship-web", "Ship to the Web"),
            ("next", "Where to Next"),
        ],
    },
];

fn lesson(id: &str) -> AnyWidget {
    match id {
        "welcome" => prose(
            "Welcome",
            "This is the hands-on path. By the end you'll have built a real todo app and understand how Pebbles thinks — components, signals, and events.",
            vec![
                p("You don't need to know Rust deeply, but a little helps. If you've used React, SolidJS, or Flutter, the ideas will feel familiar — Pebbles borrows the best parts of each."),
                p("Work through the lessons top to bottom. Each one is short, ends with runnable code, and builds on the last. When you want a reference instead of a lesson, switch to the Docs tab."),
                h2("What you'll build"),
                p("A todo app: type a task, press Enter, check items off, watch a live \"remaining\" count, and give it a themed coat of paint — all in a couple hundred lines."),
            ],
        ),
        "thinking" => prose(
            "Thinking in Pebbles",
            "Three ideas carry the whole framework. Internalize these and everything else is detail.",
            vec![
                h2("1. UI is a function of state"),
                p("Your screen is a tree of widgets returned from plain functions. You never mutate widgets — you change state, and the affected parts of the tree re-render themselves."),
                h2("2. State is a signal"),
                p("A signal is a value you can read and write. Reading it inside a component subscribes that component; writing it re-renders exactly the subscribers and nothing else. No diffing, no manual wiring."),
                h2("3. Events are closures"),
                p("There's no event system to learn. A button's `on_pressed` is a Rust closure that usually writes a signal. State changes, the UI follows."),
                code("// state → UI, one direction:\nlet name = create_signal(String::from(\"world\"));\ntext(format!(\"Hello, {}\", name.get()));   // reads → subscribes\nbutton(\"Shout\").on_pressed(move || name.set(\"WORLD\".into())); // writes → re-renders"),
            ],
        ),
        "first-app" => prose(
            "Your First App",
            "The smallest complete Pebbles app: a window with a counter.",
            vec![
                p("Create a project (or add `pebbles` to an existing one — see Docs → Installation), then make `main.rs` look like this:"),
                code("use pebbles::prelude::*;\n\nfn app() -> impl IntoWidget {\n    let count = create_signal(0);\n    center(column(children![\n        text(count.get().to_string()).size(48.0),\n        button(\"Add one\").on_pressed(move || count.update(|n| *n += 1)),\n    ]))\n}\n\nfn main() -> Result<(), Box<dyn std::error::Error>> {\n    App::new(component(app)).title(\"Hello\").run()\n}"),
                p("Run `cargo run`. Click the button — the number goes up. You just wrote a component (`app`), a signal (`count`), and an event (`on_pressed`). Everything else you'll learn is a variation on these three."),
            ],
        ),
        "todo-setup" => prose(
            "1 · Set Up the App",
            "Start the todo app with a title and an empty screen we'll fill in.",
            vec![
                p("A todo is just some text and a done flag. Model it as a plain struct, and hold the whole list in one signal — a `Vec<Todo>`."),
                code("#[derive(Clone)]\nstruct Todo { text: String, done: bool }\n\nfn app() -> impl IntoWidget {\n    let todos = create_signal(Vec::<Todo>::new());\n    container()\n        .padding(EdgeInsets::all(24.0))\n        .child(column(children![\n            text(\"Todos\").size(28.0).bold(),\n            gap_h(16.0),\n            // list + input go here in the next lessons\n        ]))\n}"),
                p("`todos` is the single source of truth. Everything on screen is derived from it, and every change goes through it."),
            ],
        ),
        "todo-list" => prose(
            "2 · Show a List",
            "Render one row per todo. When the list changes, the rows follow.",
            vec![
                p("Read the signal, map each todo to a row widget, and drop them into a column. Because we read `todos.get()` here, this rebuilds automatically whenever the list changes."),
                code("let rows: Vec<AnyWidget> = todos.get().iter().map(|t| {\n    row(children![\n        text(if t.done { \"✓\" } else { \"○\" }),\n        gap_w(8.0),\n        text(t.text.clone()),\n    ]).into_widget()\n}).collect();\n\ncolumn(rows)"),
                p("For long lists, swap the plain `column` for `list_view` — it only builds the rows that are on screen. We'll keep it simple here."),
            ],
        ),
        "todo-add" => prose(
            "3 · Add Items",
            "A text field and a button that append to the list.",
            vec![
                p("Bind a text field to its own signal, then on submit push a new todo and clear the field. `update` mutates the vec in place and notifies subscribers."),
                code("let draft = create_signal(String::new());\n\nrow(children![\n    expanded(text_field().placeholder(\"What needs doing?\").bind(draft)),\n    gap_w(8.0),\n    button(\"Add\").on_pressed(move || {\n        let text = draft.get();\n        if !text.trim().is_empty() {\n            todos.update(|list| list.push(Todo { text, done: false }));\n            draft.set(String::new());\n        }\n    }),\n])"),
                p("Type, press Add, and a new row appears — because the list view from lesson 2 reads the same `todos` signal."),
            ],
        ),
        "todo-toggle" => prose(
            "4 · Toggle Done",
            "Make each row tappable to flip its done flag.",
            vec![
                p("Wrap the row in a `pressable`. On tap, update the todo at that index. Capture the index with `enumerate` so the closure knows which one to flip."),
                code("todos.get().iter().enumerate().map(|(i, t)| {\n    pressable(row(children![\n        text(if t.done { \"✓\" } else { \"○\" }),\n        gap_w(8.0),\n        text(t.text.clone()),\n    ]))\n    .on_tap(move || todos.update(|list| list[i].done = !list[i].done))\n    .into_widget()\n}).collect::<Vec<_>>()"),
                p("Tap a row — the marker toggles. One signal, updated in place; the list re-renders itself."),
            ],
        ),
        "todo-derive" => prose(
            "5 · Counts & Filters",
            "Derive a live \"remaining\" count without recomputing it by hand.",
            vec![
                p("A `create_memo` is a value computed from other signals. It recomputes only when its inputs change and caches the result — perfect for a count or a filtered view."),
                code("let remaining = create_memo(move || todos.get().iter().filter(|t| !t.done).count());\n\ntext(format!(\"{} left\", remaining.get())).color(theme().colors.muted_foreground)"),
                p("Add the memo once and read it wherever you like; it stays correct as todos are added and toggled, with no manual bookkeeping."),
            ],
        ),
        "todo-style" => prose(
            "6 · Make It Pretty",
            "Give the app a themed, card-like look with the style system.",
            vec![
                p("Wrap each row in a `container` with a decoration, or define a reusable `Style` and apply it with `.styled(..)`. Pull colors from `theme().colors` so the app respects light and dark mode for free."),
                code("let c = theme().colors;\ncontainer()\n    .decoration(BoxDecoration::new()\n        .color(c.card)\n        .radius(BorderRadius::all(10.0))\n        .border(Border::new(c.border, 1.0)))\n    .padding(EdgeInsets::all(12.0))\n    .child(/* the row */)"),
                p("That's a complete app: input, list, toggle, a derived count, and a themed look. Everything grew out of one `todos` signal."),
            ],
        ),
        "sharing-state" => prose(
            "Sharing State",
            "Lift a signal to app scope and any component can use it — no prop-drilling.",
            vec![
                p("Local state lives in the component. When two distant parts of the tree need the same state, create the signal once at app scope (before any component runs) and read it wherever you need it. It's the *same* `create_signal` — just owned globally."),
                code("// created once at startup, shared everywhere:\nfn cart() -> Signal<Vec<Item>> { /* lazy global signal */ }\n\n// any component:\ntext(format!(\"{} items\", cart().get().len()));"),
                p("For bigger shapes, reach for `create_store`, which updates subscribers field-by-field instead of on every change."),
            ],
        ),
        "screens" => prose(
            "Multiple Screens",
            "Add navigation with a route signal and `route_view`.",
            vec![
                p("Hold the current route in a global signal and map each route to a component. Navigating is just writing the signal, so it composes with everything else you've learned."),
                code("route_view(route().get())\n    .route(\"todos\", || component(todo_screen))\n    .route(\"settings\", || component(settings_screen))\n    .fallback(|| component(todo_screen))"),
                p("Wrap it all in a `scaffold` with a `side_nav` and a `top_panel` and you have an app shell — exactly what this gallery uses."),
            ],
        ),
        "ship-web" => prose(
            "Ship to the Web",
            "The same app runs in a browser via WebGPU — no rewrite.",
            vec![
                p("Install the CLI and Trunk (Docs → Installation), then serve the wasm build. It opens in a WebGPU browser with live reload."),
                code("pebbles run -d web"),
                p("Branch on the platform only when you must — for a web-specific path, guard it with the compile-time API:"),
                code("if platform::is_web() { /* web-only */ } else { /* native */ }"),
            ],
        ),
        "next" => prose(
            "Where to Next",
            "You've built an app and learned the model. Here's where to go.",
            vec![
                p("• Browse the Components tab for the full shadcn-flavored UI set, and Widgets for layout, motion, and painting primitives — each one is live and copy-paste ready."),
                p("• Use the Docs tab as a reference when you need the details of a concept."),
                p("• Read the source of this gallery: it's a real Pebbles app that exercises every widget."),
                p("That's it — go build something."),
            ],
        ),
        _ => prose("Coming soon", "This lesson isn't written yet.", vec![]),
    }
}

/// The Learn page: shared top nav, the lesson sidenav, and a content pane.
pub fn learn() -> Element {
    let section = crate::state::learn_section();
    let active = section.get();

    let content_pane = expanded(scroll_view(
        container().padding(EdgeInsets::symmetric(40.0, 34.0)).child(
            container()
                .constraints(BoxConstraints::loose(Size::new(760.0, f64::INFINITY)))
                .child(lesson(&active)),
        ),
    ));

    column(children![
        crate::site_nav::top_nav(),
        expanded(
            row(children![sidenav(LEARN, &active, section), content_pane])
                .cross_axis_alignment(CrossAxisAlignment::Stretch),
        ),
    ])
    .cross_axis_alignment(CrossAxisAlignment::Stretch)
    .main_axis_size(MainAxisSize::Max)
    .into_widget()
}
