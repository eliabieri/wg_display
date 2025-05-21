# Frontend Technical Overview

This document provides a technical overview of the WG Display frontend, covering its architecture, core concepts, build process, and key components. The frontend is a single-page application (SPA) responsible for user interaction with the WG Display system, including widget installation from a store or URL, de-installation, configuration of individual widgets, and modification of system-wide settings like the display background color.

## 1. Core Technologies

The frontend is built using a modern Rust and WebAssembly stack:

*   **Yew Framework:** A Rust/Wasm framework for building client-side web applications with a component-based architecture inspired by React. (Using version ~0.20)
*   **Trunk:** A WASM web application bundler for Rust. It simplifies the build process, asset management (CSS, images), and integration with `index.html`.
*   **TailwindCSS:** A utility-first CSS framework used for styling the user interface. CSS classes are applied directly in the Yew `html!` macros. (Using version ~3.3)
*   **`yew-router`:** Provides client-side routing capabilities for navigating between different views/pages within the SPA.
*   **`gloo-net`:** Used for making HTTP requests to the backend Rocket server to fetch data and submit changes.
*   **`web-sys` and `js-sys`:** Provide raw bindings to browser Web APIs for direct DOM manipulation or other browser functionalities when needed.
*   **`serde` (via `common` crate):** For serialization and deserialization of data structures (like `SystemConfiguration`, `WidgetStoreItem`) exchanged with the backend.
*   **(Assumed) `jsonform.js`:** The presence of `assets/js/jsonform.js` and the dynamic nature of widget configuration strongly suggest its use within `assets/html/widget_config.html` to render forms from JSON schemas provided by widgets.

## 2. Project Structure (`frontend/` directory)

The `frontend/` directory contains all the source code and assets for the Yew-based single-page application:

*   `Cargo.toml`: Defines the Rust crate for the frontend, listing dependencies like `yew`, `yew-router`, `gloo-net`, and the local `common` crate.
*   `index.html`: The main HTML entry point for the application. It's processed by Trunk, which injects the compiled Wasm, JavaScript shims, and CSS links. Trunk directives (like `data-trunk rel="css"`) are used here for asset management.
*   `input.css`: The source CSS file for TailwindCSS, where custom styles or Tailwind directives (like `@tailwind base;`) are defined.
*   `tailwind.config.js`: Configuration file for TailwindCSS, specifying content paths (including `.rs` files for Yew templates) to scan for class usage.
*   `package.json`: Defines Node.js dependencies (primarily TailwindCSS and `concurrently` for development scripts) and scripts for building TailwindCSS (`tailwind`, `tailwind-build`) and running the development server (`serve`, `trunk`).
*   `src/`: Contains the Rust source code for the Yew application.
    *   `main.rs`: The application entry point, initializes the Yew environment and mounts the root component.
    *   `components/`: Contains reusable Yew components used across different pages (e.g., `ConfigCardComponent`, `ErrorDisplay`).
    *   `pages/`: Contains top-level Yew components that represent different views or "pages" of the application (e.g., `Home`, `Install`, `ConfigSchema`).
    *   `routing/`: Defines the client-side routes using `yew-router` (`router.rs` with `Route` enum and switch function).
*   `assets/`: Contains static assets.
    *   `assets/css/`: Global CSS files (though Tailwind is primary).
    *   `assets/fonts/`: Font files.
    *   `assets/html/`: Static HTML files, notably `widget_config.html`, which is served by the backend for widget configuration and likely uses JavaScript for dynamic form rendering.
    *   `assets/js/`: JavaScript files, including `jsonform.js` and its dependencies (`underscore.js`, `jsonform-defaults.js`).
    *   `assets/logo.png`: The application logo.
*   `dist/` (generated): The output directory where Trunk places the compiled application (Wasm, JS, CSS, copied assets). This directory is typically served by the backend in production or by `trunk serve` during development.

## 3. Build Process

The frontend build process combines Rust/Wasm compilation via Yew and Trunk, with CSS generation via TailwindCSS.

*   **Trunk (Wasm Bundler):**
    *   Trunk is the primary tool for building the Yew WebAssembly application. It's invoked via `trunk serve` for development (often through `npm run trunk`) or `trunk build --release` for production builds (typically via the root `Makefile`).
    *   **Wasm Compilation:** Trunk compiles the Rust code in `src/` into a WebAssembly (`.wasm`) binary.
    *   **JS Interop:** It generates necessary JavaScript glue code for loading and running the Wasm module in the browser.
    *   **Asset Management:** Trunk processes `index.html` to manage assets:
        *   It includes linked CSS files (like `./output.css` from Tailwind) via `<link data-trunk rel="css" ...>`.
        *   It copies directories (like `assets/`) into the final distribution folder (`dist/`) via `<link data-trunk rel="copy-dir" ...>`.
        *   It injects the necessary scripts into `index.html` to load and run the Wasm application.
    *   **Development Server:** `trunk serve` provides a live-reloading development server, recompiling on changes to Rust code or assets specified in `index.html`.

*   **TailwindCSS (CSS Framework):**
    *   TailwindCSS is used for styling. The configuration is in `tailwind.config.js`, which specifies that Rust source files (`.rs`) and HTML files should be scanned for Tailwind utility classes.
    *   The main CSS source file is `input.css`, which typically contains Tailwind's base, components, and utilities directives (`@tailwind base; @tailwind components; @tailwind utilities;`).
    *   **Development:** During development (`npm run serve`, which calls `npm run tailwind`), TailwindCSS runs in "watch" mode, recompiling `output.css` whenever changes are detected in `input.css`, `tailwind.config.js`, or the scanned source files.
    *   **Production:** For production builds (`npm run tailwind-build`, likely called by the root `Makefile`), TailwindCSS generates a minified `output.css` file.
    *   The generated `output.css` is then picked up by Trunk via the link in `index.html`.

*   **Orchestration:**
    *   For local development, `npm run serve` (defined in `package.json`) uses `concurrently` to run `trunk serve` and the TailwindCSS watch process simultaneously.
    *   For production builds, the root `Makefile` likely calls `npm run tailwind-build` and then `trunk build --release` to generate optimized, distributable frontend assets in the `frontend/dist/` directory. These assets are then embedded into the main WG Display application binary by the backend server.

## 4. Architecture and Core Concepts

This section details the internal architecture of the Yew application.

### Application Entry Point (`main.rs`)

The entry point for the Yew application is the `main()` function within `frontend/src/main.rs`.

*   **Initialization:** `yew::Renderer::<Main>::new().render();` creates a new Yew renderer for the root component `Main` and mounts it to the body of the `index.html` page. This function is primarily for development builds using `trunk serve`.
*   **Root Component (`Main`):** The `Main` functional component serves as the root of the application's component tree. Its primary responsibility in this project is to set up the `BrowserRouter` from `yew-router`, which enables client-side routing for the entire application. It then uses a `Switch` component to render the appropriate page based on the current URL.

### Routing (`yew-router`)

Client-side routing allows navigation between different views (pages) of the SPA without requiring full page reloads from the server. WG Display uses the `yew-router` crate for this.

*   **Route Definition (`frontend/src/routing/router.rs`):**
    *   An enum, `Route`, is defined and derives `yew_router::Routable`.
    *   Path patterns are associated with each enum variant using the `#[at("/path")]` attribute. Example:
        ```rust
        #[derive(Clone, Routable, PartialEq)]
        pub enum Route {
            #[at("/")]
            Home,
            #[at("/install")]
            Install,
            #[at("/config_schema/:widget_name")] // Parameterized route
            ConfigSchema { widget_name: String },
            #[not_found]
            #[at("/404")]
            NotFound,
        }
        ```
    *   Parameterized routes can capture segments from the URL path and pass them as props.
*   **Router Setup (`frontend/src/main.rs`):**
    *   The root component `Main` wraps its content with `<BrowserRouter>`.
*   **Switching Content (`frontend/src/routing/router.rs`):**
    *   A `<Switch<Route> render={...} />` component uses a `switch` function that takes the matched `Route` and returns the appropriate Yew page component (`Html`).
*   **Navigation:**
    *   Programmatic: `use_navigator().unwrap().push(&Route::SomeVariant)`.
    *   Declarative: `<Link<Route> to={Route::SomeVariant}>`.

### Yew Framework: Core Concepts

The frontend heavily utilizes the Yew framework's core features:

*   **Functional Components:** Defined with `#[function_component(ComponentName)]`, taking `Props` and returning `Html`.
*   **Hooks:**
    *   `use_state`: For simple component-local state.
    *   `use_reducer`: For complex state logic (e.g., `SystemConfiguration` in `Home`).
    *   `use_effect_with_deps`: For side effects like data fetching.
    *   `Callback::from` (for `use_callback` patterns): For event handlers.
    *   `use_navigator`: For routing.
    *   `use_clipboard` (`yew-hooks`): For clipboard interaction.
*   **HTML Macro (`html!`):** JSX-like syntax for defining component render logic.
*   **Props and Callbacks:** For parent-child component communication.

### State Management

*   **Component-Local State (`use_state`):** For UI-specific data not widely shared.
*   **Reducer-Managed State (`use_reducer`):** The main `SystemConfiguration` (including widget lists, their JSON configs, and global settings) is managed by a `use_reducer` hook in the `Home` page. Actions (`SystemConfigurationAction`) update this state, which propagates via props.
*   **No Explicit Global State Manager:** Beyond `SystemConfiguration` in `Home`, there isn't an app-wide global state manager evident in the reviewed code.

### Backend API Interaction (`gloo-net`)

Communication with the Rocket backend is handled using `gloo_net::http::Request`:

*   **Requests:** `Request::get(url).send().await`, `Request::post(url).json(&data).send().await`. Handled within `wasm_bindgen_futures::spawn_local`.
*   **Responses:** Deserialized with `.json::<T>().await` or read as text with `.text().await`. Status codes are checked.
*   **Error Handling:** Network/HTTP errors from `gloo-net` are caught and typically update a local `error` state, displayed via `ErrorDisplay` component. Backend-generated errors can be read from the response body.

### Dynamic Widget Configuration Forms

Widget configuration is highly dynamic:

*   **Schema-Driven:** Widgets provide a JSON Schema for their options via `/config_schema/<widget_name>`.
*   **Frontend Orchestration:**
    1.  `Home` page links to `/widget_configuration/<widget_name>`.
    2.  Backend serves a static HTML page: `frontend/assets/html/widget_config.html`.
    3.  **JavaScript Logic (in `widget_config.html`):** This static page's JavaScript is assumed to:
        *   Extract `widget_name` from URL.
        *   Fetch JSON schema from `/config_schema/<widget_name>`.
        *   Fetch current widget configuration JSON.
        *   Use `jsonform.js` (from `assets/js/`) to render an HTML form from the schema and current values.
        *   On submission, POST the new JSON configuration to `/widget_config/<widget_name>`.
*   **Yew's Role:** Yew navigates to this static page and provides the backend APIs. The Yew component `pages/config_schema.rs` is a utility for copying schemas, not for user form display.

## 5. Key Pages and Components

This section highlights some of the main Yew pages and reusable components.

### Key Pages (`src/pages/`)

*   **`home.rs` (`<Home />`):** Main dashboard at `/`. Manages `SystemConfiguration` (widgets, background color). Displays widget cards, allows de-installation, links to widget config and install page.
*   **`install.rs` (`<Install />`):** Page at `/install`. Fetches/displays widget store items. Allows installation from store or URL via `/install_widget` API. Handles de-installation here too. Navigates to `Home` on success.
*   **`config_schema.rs` (`<ConfigSchema />`):** Utility page at `/config_schema/:widget_name`. Fetches and copies the widget's JSON config schema to the clipboard. Does not render the user-facing form.

### Reusable Components (`src/components/`)

*   **`background_color_config.rs` (`<BackgroundColorConfigComponent />`):** UI for changing system background color; dispatches to `SystemConfiguration` reducer.
*   **`config_card.rs` (`<ConfigCardComponent />`):** Styled wrapper for configuration sections.
*   **`divider.rs` (`<DividerComponent />`):** Visual divider with text.
*   **`error_display.rs` (`<ErrorDisplay />`):** Displays error messages passed via props.

### Static HTML for Widget Configuration (`assets/html/widget_config.html`)

*   **Role:** Served by backend for `/widget_configuration/<widget_name>`. Crucial for the actual user interface for configuring a specific widget.
*   **Functionality (JavaScript-driven):** Uses JavaScript (likely `jsonform.js`) to fetch the widget's JSON schema and current config, dynamically render the form, and submit updated configurations back to the server.
