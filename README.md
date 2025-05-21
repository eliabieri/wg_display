<div align="center">
    <img src="docs/images/logo.png" alt="WG Display Logo" style="height: 80px">
    <br>
    <h1>WG Display</h1>
    <strong>
        🦀 A highly customizable information display for your Raspberry Pi, built in Rust and extensible with WebAssembly widgets 🦀
    </strong>
    <p>Create your personal dashboard with community-driven or self-made widgets. Open-source and easy to set up.</p>
    <br/>
</div>

![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/eliabieri/wg_display/cargo_test.yml?label=test&logo=github)
![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/eliabieri/wg_display/build.yml?logo=github)

![GitHub release (latest SemVer)](https://img.shields.io/github/v/release/eliabieri/wg_display?logo=github)
![GitHub commits since latest release (by SemVer)](https://img.shields.io/github/commits-since/eliabieri/wg_display/latest/main?logo=github)

## ⭐️ What WG Display can show you

- 🚂 The next public transport connections between two stations
- 🏊🏻 The current temperature of the Aare River and whether it is a good idea to take a dip
- ⏰ The current date and time
- 🧖🏽‍♀️ The current occupancy in the "Sauna Lorrainebad" in Bern, CH
- 🕺🏾 The next event at the Cafete Club
- Got another idea? Write your own widget!

## ✨ Features

- 🦀 **Fully built in Rust:** For performance and reliability.
- 🔧 **Extensible with WebAssembly:** Add new functionalities by writing your own widgets.
  - 🧩 **WebAssembly Component Model:** Widgets are true WebAssembly components, ensuring sandboxing and language independence (Rust, Python, TinyGo, etc.).
  - 🔗 **Clear Host-Widget Interface (WIT):** Communication between the display host and widgets is defined by a WIT (WebAssembly Interface Type) contract. This includes providing widgets with capabilities like HTTP requests, random number generation, clock access, and logging.
  - 🌍 **WASI Integration:** Standard WASI (WebAssembly System Interface) capabilities (like filesystem access, environment variables, if permitted by host) are provided to widgets for common system-level tasks.
- 🚀 **Easy Deployment:** Compiles to a single binary for straightforward setup.
- 🤑 **Raspberry Pi Focused:** Optimized for Raspberry Pi (64-bit capable), even a Pi Zero 2 W.
- ⚙️ **User-Friendly Configuration:** Widgets can be configured via a web interface.

## 📚 Table of contents

- [⭐️ What WG Display can show you](#️-what-wg-display-can-show-you)
- [✨ Features](#-features)
- [📚 Table of contents](#-table-of-contents)
- [🚀 Getting started](#-getting-started)
- [🛠️ Assembling the hardware](#️-assembling-the-hardware)
- [🔨Building from source](#building-from-source)
  - [Prerequisites](#prerequisites)
  - [Prerequisites for cross-compilation](#prerequisites-for-cross-compilation)
  - [Building the project](#building-the-project)
- [👏 Writing your own widget](#-writing-your-own-widget)
- [🔩 Architecture and Technical Details](#-architecture-and-technical-details)
  - [WebAssembly Integration: Components, WIT, and Lifecycle](#webassembly-integration-components-wit-and-lifecycle)
  - [Host Capabilities and WASI](#host-capabilities-and-wasi)
  - [Error Handling and Logging (Widgets)](#error-handling-and-logging-widgets)
  - [Configuration Management: Frontend-Backend Interaction](#configuration-management-frontend-backend-interaction)
- [📖 Documentation (rustdocs)](#-documentation-rustdocs)
- [🧪 Testing](#-testing)
- [🔮 What comes next](#-what-comes-next)
- [🔒 Safety](#-safety)
- [♻️ Updating the dependencies](#️-updating-the-dependencies)
- [🦾 Developing on target](#-developing-on-target)

![WG Display image front](docs/images/wg_display.jpg)

---

![Configuration dashboard](docs/images/dashboard.jpeg)
The web interface allows the users to configure system aspects like the background color used on the display or various configuration options of the different widgets.

## 🚀 Getting started

1. Flash the latest Raspberry Pi OS Lite (64-bit) image to an SD card.  
   You can use the [Raspberry Pi Imager](https://www.raspberrypi.com/software/).
   It allows you to configure the Wi-Fi credentials and enable SSH (you'll need it in the next step).
2. SSH into the Raspberry Pi and run the installation script
   ```bash
   # SSH into the Raspberry Pi
   ssh pi@raspberrypi.local
   # Run the installation script (after you've logged in over SSH)
   curl -sL https://raw.githubusercontent.com/eliabieri/wg_display/main/install_on_raspberry.py | python3
   ```

The configuration dashboard should be available at [wgdisplay.local](http://wgdisplay.local)

## 🛠️ Assembling the Hardware

WG Display is designed for Raspberry Pi and commonly available display hats. Even a Raspberry Pi Zero 2 W is sufficient, as the application is very resource-efficient (typically around 3% CPU on a Raspberry Pi 3B).

Here are some tested display options:

*   **[5" MIPI DSI Display](https://t.ly/fWl3)**
    *   ✨ Requires no driver
    *   ✨ Includes stand
    *   📐 Large enough to display ~6 widgets
    *   💲💲💲 (Higher price)
*   **[3.5" HAT Display](https://t.ly/DfWJ)**
    *   ✨ Includes enclosure for wall mounting
    *   ⚠️ Requires a [driver](https://github.com/goodtft/LCD-show/blob/master/MHS35-show)
    *   📐 Large enough to display ~3 widgets
    *   💲 (Lower price)
*   **[3.5" HAT HDMI Display](https://t.ly/l2Rd)**
    *   ✨ Requires no driver
    *   📐 Large enough to display ~3 widgets
    *   💲💲 (Mid price)

Since WG Display renders to the terminal, most other displays compatible with Raspberry Pi should also work.

## 🔨 Building from Source

### Prerequisites

First, install [rustup](https://rustup.rs) then

```bash
# Install WebAssembly target
rustup target add wasm32-wasip2
# Install trunk for building the frontend
cargo install --locked trunk
# Install NPM dependencies
cd frontend && npm install
```

### Prerequisites for cross-compilation

First, install [docker](https://www.docker.com), then

```bash
cargo install cross --git https://github.com/cross-rs/cross
```

### Building the project

```bash
# Native build
make
# Cross compilation (Raspberry Pi 3/4/Zero 2 W with 64-bit OS)
make app_aarch64
```

Then simply copy over the generated binary to the target and run it.

```text
💡 To run it at boot, simply add the path to the binary to the end of the ~/.bashrc file.
```

## 👏 Writing Your Own Widget

Want your WG Display to show information like:

- 🥳 Upcoming events in your favorite nightclub?
- 🚮 The trash collection schedule for your municipality?
- 🍺 Beers on sale in your local supermarket?

WG Display widgets are WebAssembly components that implement a specific interface defined in WIT (WebAssembly Interface Type). This allows you to write widgets in various languages (Rust, Python, TinyGo, etc.) that can:

- Receive configuration from the user.
- Access capabilities provided by the host, such as making HTTP requests, getting current time, logging, and generating random numbers, all defined in the [official WIT definition](https://eliabieri.github.io/wg_display_widget_wit/).
- Return formatted data to be shown on the display.

You have two main options to get a new widget:

1.  **[Write your own widget](docs/write_new_widget.md):** The linked guide provides further details on the development process and points to template repositories (Rust, Python) to get you started quickly.
2.  **Request a widget:** If you're not able to write it yourself, feel free to open a [feature request](https://github.com/eliabieri/wg_display/issues/new) and tag it with the `widget request` label.

## 🔩 Architecture and Technical Details

This section provides a deeper look into the internal workings of WG Display, focusing on its WebAssembly architecture, widget lifecycle, and other technical aspects.

### WebAssembly Integration: Components, WIT, and Lifecycle

WG Display's extensibility is built upon the **WebAssembly (Wasm) Component Model** and the **Wasmtime** runtime. This modern approach allows for sandboxed, language-agnostic, and efficiently managed widgets.

#### The Component Model and WIT

*   **Interface Definition (WIT):** At the core is the `widget` world, defined using WIT (WebAssembly Interface Type) and documented at [eliabieri.github.io/wg_display_widget_wit/](https://eliabieri.github.io/wg_display_widget_wit/). This WIT contract precisely defines:
    *   **Imports:** Functions and types the host (WG Display application) provides to widgets (e.g., HTTP requests, logging, clock access, random number generation).
    *   **Exports:** Functions widgets must implement (e.g., `get-name`, `get-version`, `run`).
*   **Code Generation (`wit-bindgen`):** Inside the WG Display application (specifically in `app/src/widgets/running/runtime.rs`), the `wasmtime::component::bindgen!` macro utilizes `wit-bindgen`. This tool processes the WIT definition and generates the necessary Rust code (structs, traits, and functions) that allows the host to seamlessly call Wasm widget exports and for widgets to call host imports. This avoids manual boilerplate for data type conversion and function calls across the Wasm boundary.
*   **Sandboxing and Capabilities:** The Component Model, along with Wasmtime's runtime, ensures strong sandboxing:
    *   **Memory Isolation:** Each widget runs in its own isolated linear memory. The host cannot arbitrarily access widget memory, nor can widgets access host memory or other widgets' memory, except through explicitly defined interface calls.
    *   **Capability-Based Security:** Widgets have no inherent capabilities. They can only interact with the outside world (e.g., make HTTP requests, get time) through the functions explicitly imported via the WIT interface. This means the host has fine-grained control over what a widget can do.

#### Widget Lifecycle

The journey of a widget from source to execution involves several stages:

1.  **Development:** Widget authors write their logic in a language that compiles to WebAssembly (e.g., Rust, Python via bindings, TinyGo) and implements the exports defined in the `widget` WIT world. Template repositories are provided to simplify this.

2.  **Installation:**
    *   Triggered via the `/install_widget` API endpoint, either from a direct URL or the Widget Store.
    *   The `WidgetManager` downloads the raw Wasm component binary.
    *   **Pre-compilation:** The Wasmtime `Engine` then pre-compiles this binary using `engine.precompile_component()`. This step converts the portable Wasm code into an engine-specific, optimized format. The result, along with a `compatibility_hash` (to detect engine updates), is stored as a `CompiledWidget`. This pre-compilation significantly speeds up future instantiations.

3.  **Instantiation (Loading):**
    *   When a widget needs to be run, its `CompiledWidget` data is retrieved.
    *   The `Runtime::instantiate_widget` method first checks the `compatibility_hash`. If the engine has been updated in a way that breaks compatibility, the widget is marked as needing recompilation (re-installation would be required as the original Wasm is not stored by the host).
    *   The pre-compiled artifact is deserialized using `unsafe { Component::deserialize(...) }`. This is considered safe as long as the bytes originate from a trusted `engine.precompile_component()` call.
    *   Finally, `Widget::instantiate(...)` creates an instance of the component. This involves:
        *   Setting up the WASI context for the instance.
        *   Linking the component's imports to the host's corresponding implementations (both the custom `widget` world functions and the standard WASI functions).

4.  **Execution:**
    *   The display rendering logic (or a scheduler) calls `Runtime::run_widget(...)`.
    *   The host first checks the widget's preferred update frequency using `widget.call_get_run_update_cycle_seconds()`. If it's not yet time to update, the `run` call is skipped.
    *   The host prepares a `WidgetContext` (containing the last invocation time and user-defined JSON configuration).
    *   The widget's main `widget.call_run(&mut self.store, &context)` export is invoked.
    *   The widget performs its logic and returns a `WidgetResult` (containing the string data to be displayed).

This lifecycle ensures that widgets are efficiently loaded and run within a secure, controlled environment, with clear contracts for interaction.

### Host Capabilities and WASI

Widgets operate with a combination of custom capabilities provided by the WG Display host and standard interfaces provided by WASI.

#### Custom Host Imports (Implemented in `app/src/widgets/host_api/`)

Besides the standard WASI interfaces, the host provides a set of custom capabilities to widgets, defined in the `widget` WIT world. These are implemented in the `app/src/widgets/host_api/` directory:

*   **`widget:widget/http` (`http.rs`):** Allows widgets to make HTTP(S) requests.
    *   Implemented using the blocking `reqwest::blocking::Client`. This means HTTP requests made by a widget will block that widget's execution thread until the request completes or times out.
    *   Supports GET, POST, PUT, HEAD, DELETE methods.
    *   Basic error reporting: network or HTTP errors from `reqwest` are returned to the widget as an empty error type (`Err(())` in the WIT `result<response>`).
*   **`widget:widget/logging` (`logging.rs`):** Enables widgets to log messages through the host.
    *   Uses the standard Rust `log` crate (e.g., `log::info!`).
    *   Messages are prefixed with "WIDGET:" and the widget-provided context string, integrating them into the main application logs.
*   **`widget:widget/clocks` (`clocks.rs`):** Provides access to the current time.
    *   Implemented using `std::time::SystemTime::now()`.
*   **`widget:widget/random` (`random.rs`):** Allows widgets to generate random numbers.
    *   Implemented using `rand::thread_rng().next_u64()`, providing cryptographically secure random numbers.

#### WASI Integration

WG Display leverages the WebAssembly System Interface (WASI) to provide widgets with access to common system-level capabilities in a controlled manner. This is achieved through Wasmtime's WASI support.

*   **Default Context:** A default WASI context is provided to each widget instance (`WasiCtxBuilder::new().build()`). This grants access to a standard set of WASI interfaces, including:
    *   **`wasi:clocks/wall-clock`**: Access to the system's wall clock.
    *   **`wasi:clocks/monotonic-clock`**: Access to a monotonic clock.
    *   **`wasi:random/random`**: Access to a cryptographically secure random number generator.
    *   **`wasi:io/streams` (for stdio)**: Standard input, output, and error streams (`stdin`, `stdout`, `stderr`) are inherited from the host process. Widget print statements or error outputs will typically appear in the WG Display application's logs or console output.
    *   **`wasi:cli/environment`**: Widgets can access environment variables passed down from the host process. By default, all host environment variables are accessible.

*   **Limitations & Scope:**
    *   **Filesystem Access:** The default WASI context does **not** grant any filesystem access to widgets. To enable this, the host would need to explicitly configure `preopened_dir` on the `WasiCtxBuilder`, which is currently not done. This enhances security by preventing widgets from arbitrarily accessing files.
    *   **Direct Network Access (WASI Sockets):** WASI's socket networking capabilities (`wasi:sockets/*`) are not enabled by default and are not explicitly configured for widgets. Instead, widgets are provided with a specific HTTP capability via the `widget:widget/http` custom interface.
    *   **Command-Line Arguments:** Widgets do not receive command-line arguments via `wasi:cli/argv` as they are not executed as standalone CLI applications.

This setup ensures that widgets operate in a well-defined environment, having access to essential OS-level features through WASI while being restricted in areas like direct filesystem or socket network access, relying instead on host-mediated capabilities for specific interactions (like HTTP).

### Error Handling and Logging (Widgets)

Effective error handling and logging are crucial for developing and operating widgets. WG Display provides mechanisms for both.

#### Widget Execution Errors

*   **Traps:** If a widget encounters a critical error during its execution (e.g., division by zero, out-of-bounds memory access, unhandled panic in Rust-based widgets), it will cause a WebAssembly trap.
    *   The Wasmtime runtime, used by WG Display, catches these traps.
    *   When the host calls a widget export (like `run`, `get_name`, etc.), the result is typically a `wasmtime::Result<T>`. If a trap occurs, this `Result` will be an `Err(wasmtime::Error)`.
    *   The host application (`app/src/widgets/running/runtime.rs`) is structured to handle these `wasmtime::Error` results. Detailed trap information might primarily appear in the host application's main log output or console.
*   **User-Friendly Errors in Data:** Widgets can also proactively report issues by returning an error message within their `WidgetResult.data` string (the primary output of the `run` function). This allows widgets to communicate configuration problems or operational failures directly to the end-user on the display.

#### Widget-Initiated Logging

Widgets can actively log information using the `widget:widget/logging` interface provided by the host.

*   **Interface:** The WIT definition includes:
    ```wit
    interface logging {
        use types.{level}; // enum { debug, info, warn, error }
        log: func(level: level, context: string, message: string);
    }
    ```
*   **Host Implementation (`host_api/logging.rs`):**
    *   The host implements this `log` function using the standard Rust `log` crate.
    *   Messages are automatically prefixed with "WIDGET:" and the widget-provided `context` string (e.g., `INFO WIDGET: MyWidget: Successfully fetched data`).
*   **Visibility:** These logs become part of the main WG Display application's log output, directed by the host's logging configuration.

This dual approach helps in diagnosing issues both during widget development and in production.

### Configuration Management: Frontend-Backend Interaction

WG Display features a web-based interface for configuring system settings and individual widgets. This involves communication between the Yew-based frontend and the Rocket-based backend API.

*   **Backend API Endpoints (`app/src/server/mod.rs`):**
    *   `GET /system_config`: Retrieves the current `SystemConfiguration` (includes installed widgets, their JSON configurations, and system settings like background color).
    *   `POST /system_config`: Saves the entire `SystemConfiguration`.
    *   `GET /config_schema/<widget_name>`: Fetches the widget-specific JSON schema for its configuration options.
    *   `POST /widget_config/<widget_name>`: Saves the JSON configuration for a specific widget.
    *   Others: `/install_widget`, `/deinstall_widget`, `/get_store_items`.

*   **Frontend Interaction (Yew):**
    1.  **Load Config:** On dashboard load, fetches from `/system_config`.
    2.  **Dynamic Forms:** For widget configuration, fetches schema from `/config_schema/<widget_name>`, then likely uses a library like `jsonform.js` (present in project assets) to render the form using this schema and current widget config values.
    3.  **Save Config:** Submits widget config to `/widget_config/<widget_name>` or system config to `/system_config`.

*   **Persistence (`Persistence` module):** The backend uses this module to save/load the `SystemConfiguration` (likely to a local JSON file).

This separation allows a dynamic frontend driven by widget-defined schemas, with the backend managing storage.

## 📖 Documentation (rustdocs)

The rustdocs can be built using

```bash
make docs
```

This generates three separate documentations, one for each crate:

*   **Backend & Core (`app` crate):** `app/target/doc/app/index.html`
*   **Common Types (`common` crate):** `common/target/doc/common/index.html`
*   **Frontend (Yew - `frontend` crate):** `frontend/target/doc/frontend/index.html` (primarily documents the Rust parts of the frontend).

### Frontend Technical Overview

For a detailed explanation of the frontend architecture, components, build process, and core concepts, please see the [Frontend Technical Overview](docs/frontend_technical_overview.md).

## 🧪 Testing

Widgets should provide unit tests for their functionality where adequate.  
Asynchronous functions can be tested using the [tokio_test::block_on](https://docs.rs/tokio-test/latest/tokio_test/fn.block_on.html) function.

## 🔮 What comes next

- [X] Add installation script
- [X] Dynamically load widgets
- [X] Template repository for widgets written in Rust
- [ ] Smoothen up web interface (e.g. add loading indicator, allow updating of widgets)
- [ ] Template repository for widgets written in JS
- [ ] Allow user to configure Wi-Fi credentials via web interface
- [ ] Starting the binary through systemd
- [ ] Implement an update mechanism
- [ ] Implement authentication for the web interface

## 🔒 Safety

This project uses `#[forbid(unsafe_code)]` in all crates to ensure that no `unsafe` Rust is ever added to the project

## ♻️ Updating Dependencies

To update the dependencies in all crates, simply run:
```bash
scripts/cargo_update.sh
```

## 🦾 Developing on Target

During development, you might need to run the application on a target device. The following script serves as a template to copy the binary to your Raspberry Pi and restart the application.

**Note:** You'll likely need to adjust paths (like the `scp` source path) and potentially the target architecture in the `make` command to fit your setup.

```bash
#!/bin/sh
set -e

# Prerequisites on target:
# - Hostname set (e.g., to 'wgdisplay.local' or your Pi's IP)
# - SSH access enabled and public key added to authorized_keys for passwordless login
# - Consider if root SSH login is needed or if 'sudo' commands can be run passwordlessly by 'pi' user

# Example for ARMv7 (Raspberry Pi 2/3 with 32-bit OS) - adjust if needed
# For 64-bit OS on Pi 3/4/Zero2W, you might use 'make app_aarch64'
# and the corresponding binary path.
# make app_arm 
# SOURCE_BINARY_PATH="app/target/arm-unknown-linux-gnueabihf/release/app"

# Example for aarch64 (Raspberry Pi 3/4/Zero2W with 64-bit OS)
make app_aarch64
SOURCE_BINARY_PATH="app/target/aarch64-unknown-linux-gnu/release/app"


# Replace 'pi@wgdisplay.local' with your actual target
TARGET_HOST="pi@wgdisplay.local"
TARGET_APP_NAME="wgdisplay_dev" # Name of the app on the target
TARGET_BINARY_PATH="/home/pi/\${TARGET_APP_NAME}" # Example target path

echo "Deploying to \$TARGET_HOST..."
# Stop previous instances by name
ssh \$TARGET_HOST "sudo pkill -9 \$TARGET_APP_NAME || true; sudo pkill -9 wgdisplay || true" 
scp "\$SOURCE_BINARY_PATH" "\${TARGET_HOST}:\${TARGET_BINARY_PATH}"
ssh \$TARGET_HOST "sudo chmod +x \${TARGET_BINARY_PATH} && nohup sudo \${TARGET_BINARY_PATH} > /tmp/\${TARGET_APP_NAME}.log 2>&1 &" 
echo "Done. Application started in background on target. Log: /tmp/\${TARGET_APP_NAME}.log"
```
