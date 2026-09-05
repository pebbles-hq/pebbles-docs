# Pebbles — landing & gallery

The showcase app for **[Pebbles](https://github.com/pebbles-hq/pebbles)**, a
Flutter-style, desktop-first GUI framework for Rust. It's the interactive
**gallery** — every widget in the catalog, live and styled to shadcn — and the app
this landing site embeds under its **learn / docs** section (the way Flutter's site
hosts its widget catalog).

> Looking for quick starter apps (counter, todo, …)? Those live in the framework
> repo under [`pebbles/examples/`](https://github.com/pebbles-hq/pebbles/tree/main/examples).
> This repo is the *big* showcase; that's the *small* "how easy is it" set.

## Run it

The gallery depends on the framework by **git** (tracks `pebbles` `main`), so it
builds standalone:

```bash
cargo run                 # native (desktop)
```

For the web build, install the Pebbles CLI and Trunk and run it in `-d web` mode
(same as any Pebbles app):

```bash
cargo install --git https://github.com/pebbles-hq/pebbles pebbles-cli
cargo install --locked trunk
pebbles run -d web        # serves the wasm bundle (needs a WebGPU browser)
```

## What's inside

- `src/screens/` — one screen per widget/feature group (the catalog).
- `src/{app,state,ui,styles}.rs` — the routed shell, the global route signal, and
  the small `screen()`/`doc()` presentation helpers.
- `src/soak.rs` — the navigation leak-soak test (hops every route, asserts the
  element/render/timer census returns to baseline — the framework's lifecycle
  tripwire).
- `src/capture.rs` + `demo/build_demo.py` — headless demo capture
  (`GALLERY_CAPTURE=<dir> cargo run --release` rasterizes windows off-screen via
  vello; the script composites the strip/GIF — no display server needed).
- `assets/` — the Pebbles brand assets used by the showcase.

## License

Apache-2.0 (same as the framework).
