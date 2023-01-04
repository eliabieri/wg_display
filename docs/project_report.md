
# WG Display - Project Report

Report for the course Project 2  
Author: Elia Bieri  
Supervisor: Michael Röthlin  
Date: 03.01.2022  

![WG Display](./images/wg_display.jpg)

<div class="page"/>

## Table of contents

- [Abstract](#abstract)
- [Introduction](#introduction)
  - [Motivation](#motivation)
  - [Deliverables](#deliverables)
- [Requirements](#requirements)
  - [Functional](#functional)
  - [Non-functional](#non-functional)
- [Method](#method)
  - [Project management](#project-management)
  - [Repository settings](#repository-settings)
- [System architecture](#system-architecture)
  - [Programming language](#programming-language)
  - [Components](#components)
  - [Configuration frontend](#configuration-frontend)
  - [Display renderer](#display-renderer)
  - [Web server](#web-server)
  - [Configuration persistence](#configuration-persistence)
  - [Cross compilation](#cross-compilation)
  - [Build process](#build-process)
  - [Tests](#tests)
  - [Continuous integration](#continuous-integration)
  - [Deployment](#deployment)
- [Results](#results)
- [Future extensions](#future-extensions)
- [Glossary](#glossary)
- [Appendix](#appendix)
  - [README.md](#readmemd)
  - [Documentation on how to write a new widget](#documentation-on-how-to-write-a-new-widget)
- [Declaration of Authorship](#declaration-of-authorship)

<div class="page"/>

## Abstract

The WG Display is a device that shows information relevant to people living together.
The information is displayed on a screen that is mounted on the wall or placed on a counter.  
Earlier prototypes of such a device were devloped over the years, but they never reached a state where they could be used by other people.

This project aims to develop a new version of the WG Display that is more robust, easier to configure and better documented.

The result of this project is a working prototype of the WG Display that is user friendy, extensible, can be configured by the users and is well documented.

## Introduction

### Motivation

Over the years living with together with different people, the need arose to have a central place for displaying certain information that is relevant to all room mates. This information could be anything from the current weather, the next bus departures from the nearest public transport station to the temperature of the Aare river. The idea is to have a central place where all this information is displayed. This place is the WG Display.

Since we had quite a bit of experience with the Raspberry Pi, we decided to use it as the main controller.  
Together with a 5" display case that we had lying around, we were able to quickly build a prototype.
The software was implemented in Python, which allowed us to quickly develop the first version of the software.  

Over time, the software was extended to support more features.  
Guests coming over to our place started to take notice and wanted to have a WG Display of their own.  

The problem was, that our solution was "too hacked together".  
There was no way to configure it to the user's needs.  
We also had occasional problems with the software crashing, which was hard to debug.  

That's how the idea of a complete rewrite of the software arose.  
The need for such a display was clearly validated over the years and we had enough time to determine the shortcomings of the previous solution.  
The new software should be more robust, easier to configure and better documented.

<div class="page"/>

### Deliverables

The deliverables for this project are:

- The codebase of a working prototype of the application
- Documentation of the codebase that is fun to read
- A GitHub repository configured so that it can form the basis for growing a community around the project
- A project report that describes the project and the software architecture
- A presentation that explains the project and the software architecture

## Requirements

### Functional

- The software must be easy to deploy (compiles to single binary)
- The software must allow the user to configure the displayed widgets
- The user must be able to configure the device via a web interface
- The configuration must be persisted across restarts
- Configuration changes must be applied without restarting the software
- The software must be deployable on all Raspberry Pi models
- The sofware must run on various screen sizes (3.5", 5", 7")
- Prebuilt binaries must be available for download

### Non-functional

- The sofware shall be robust and crash as little as possible
- The software shall be well documented
- The documentation shall be fun to read
- The project shall be open source and available on GitHub
- The project shall be easy to contribute to
- Contribution guides shall be available

<div class="page"/>

## Method

### Project management

Since the project was developed in a way that allowed for contributions from multiple people, GitHub was used not only as a code repository, but also as a project management tool.  

The GitHub project board was used as a task management tool.

![GitHub project board](./images/github_project.png)

This has the advantage, that the project board is tightly integrated with the code repository.  
Tasks can be connected to pull requests and issues.  

Tasks were assigned to a milestone, that represented the the end of the Project 2 course.  

![milestone](./images/milestone.png)
Since tasks were subdivided into smaller tasks, the milestone view gave a nice overview of the progress of the project.

<div class="page"/>

### Repository settings

The GitHub project was configured to enforce several rules:

- No user should be allowed to push directly to a non feature branch.
  This was achieved by setting a branch protection rule that required a pull request before merging.
- All pull requests required all status checks to pass before they could be merged.
  This was achieved by setting a branch protection rule that required all status checks to pass before merging.
- Prevent merge commits from being pushed to non feature branches.
  This ensures a linear commit history.

With this being setup, the project should be ready for contributions from multiple people.

<div class="page"/>

## System architecture

### Programming language

The programming language of choice for this project was Rust.

Rust is a systems programming language that is designed to be fast, reliable and secure.  
It's extensive type system ensures that the code is safe and robust.  
The language is also very well suited for writing concurrent code, which is a requirement for this project.
Since Rust can be compiled to [WebAssembly](https://www.rust-lang.org/what/wasm), it could also be used to write the configuration frontend.
Furthermore, the developer experience is very good. This is reflected in the [Stack Overflow developer survey](https://survey.stackoverflow.co/2022/), where Rust is ranked as the most loved programming language for the past seven years.
Last but not least, Rust has a very active community and a big ecosystem of `crates` (libraries) that simplified the development of the project greatly.

Given all this, Rust presented itself as the perfect choice for this project.

### Components

The codebase is generally structured around three main crates.  

There's the frontend crate. It contains all the sources for the configuration frontend.  
The frontend is a web application that is written using the [Yew](https://yew.rs/) framework.

The app crate contains the main application logic.  
It is subdevided into several modules, each of which is responsible for a specific task.
The renderer module contains the logic responsible for rendering the display output.
The server module contains the logic responsible for serving the configuration frontend and providing a REST API for accessing the system configuration.  
A third module, called shared, holds the configuration persistence logic, that is shared between the renderer and the server.  

Finally, there's the common crate. As it's name implies, it contains code that is shared between the frontend and the app crate.  
It's main content are models (structs) that hold the configuration data and an enumeration that represents the individual widgets and their metadata.

The following diagram shows the crate and module structure of the project.

```text
├── app
│   └── src
│       ├── renderer
│       ├── server
│       └── shared
├── common
│   └── src
└── frontend
    └── src
        └── components
```

<div class="page"/>

### Configuration frontend

The configuration frontend is a web application that allows the user to configure the WG Display.  
The frontend was written using the [Yew](https://yew.rs/) framework.  
Yew is a component-based framework for writing web applications in Rust.  

The components are written in Rust and HTML.  
In order for them to live in a single file, `Yew` provides a macro called `html!` that allows to write HTML in Rust.  

```rust
#[derive(Properties, PartialEq)]
pub struct ConfigCardProps {
    pub children: Children,
}

#[function_component(ConfigCardComponent)]
pub fn config_card_component(props: &ConfigCardProps) -> Html {
    html! {
        <div class="p-4 my-3">
            { for props.children.iter() }
        </div>
    }
}
```

The example above shows a component that renders children components with some padding.  
Components can receive properties from their parent component.  

To style the frontend, the CSS utility framework [Tailwind CSS](https://tailwindcss.com) was used.

The following screenshot shows the configuration frontend:

![frontend](images/dashboard.jpeg)

<div class="page"/>

### Display renderer

The `renderer` module is responsible for rendering the display output using [cursive](https://github.com/gyscos/cursive) which is a `crate` for building terminal-based user interfaces (TUIs).  
The output consists of a number of widgets.  
`Widget` is the term used by this project to describe the individual pieces of information that are displayed on the screen.  
Widgets have a name and a corresponding value, that is updated dynamically.  

```rust
/// Base trait for all widgets
/// Every widget must implement this trait
#[async_trait]
pub trait Widget {
    fn new() -> Self
    where
        Self: Sized;

    /// Returns the meta data of the widget
    /// This is used to identify the widget on the display and on the frontend dashboard application
    fn get_meta_data(&self) -> WidgetMetaData;

    /// Returns the content of the widget
    /// Widgets may use newlines to display multiple lines
    fn get_content(&self) -> &str;

    /// Updates the widget content
    /// This method is called periodically by the renderer
    /// The widget must implement its own timeout logic to prevent unnecessary updates
    async fn update(&mut self, config: &WidgetConfiguration);
}
```

Every widget must implement the `Widget` trait.  

The renderer first loads the configuration from the embedded database.  
It then instantiates all widgets that are enabled by the user.  
After that, it starts a loop in which it calles the `update` method of each widget once a second.  
This gives the widgets the opportunity to update their content.  
Most widgets may not need to update their content every second, so they can implement their own timeout logic.  

After updating the widgets, the renderer renders the content of all widgets to the display.

<div class="page"/>

### Web server

The `server` module is responsible for serving the configuration frontend and providing a REST API for accessing the system configuration.  

The module uses the [rocket](https://crates.io/crates/rocket) crate for this purpose.  
This makes it very easy to implement the REST API.

```rust
/// Saves the system configuration
#[post("/config", format = "json", data = "<config>")]
async fn save_config(config: json::Json<SystemConfiguration>) {
    Persistence::save_config(config.into_inner());
}

/// Returns the system configuration
#[get("/config")]
fn get_config() -> Option<json::Value> {
    Some(json::json!(Persistence::get_config()))
}
```

### Configuration persistence

The configuration is stored in an embedded database called [sled](http://sled.rs).
This database is a key-value store that is optimized for speed and low memory usage.
It could satisfy all the requirements of this project, was easy to use and is actively maintained.

The [serde](https://serde.rs) crate was used to serialize the configuration to JSON before storing it in the database.

### Cross compilation

Since this project is expected to run on all Raspberry Pi models, cross compilation was used to build the project for all supported targets.  

The following targets are supported:

- `arm-unknown-linux-gnueabihf` (Raspberry PI Zero 1 / Zero W / Zero WH)
- `armv7-unknown-linux-gnueabihf` (Raspberry PI 2 / 3 / 4 / Zero 2 W)
- native (whatever the build machine is)

To achieve this, the [cross](https://github.com/cross-rs/cross) project was used.  
This project allows to build Rust projects for different targets using prebuilt Docker images.  
Not having to manually install the required toolchains for each target is a huge advantage.  

<div class="page"/>

### Build process

A goal of the project was to compile down to a single binary.  
In order for this to work, the frontend artifacts need to be embedded into the binary.  
Fortunately, there's a crate called [Rust Embed](https://crates.io/crates/rust-embed) that provides a custom derive macro for embedding files into a binary.

```rust
#[derive(RustEmbed)]
#[folder = "../frontend/dist"]
struct Asset;
```

The files are then accessible at runtime through the `Asset` struct.  

```rust
let filename = "index.html";
let asset = Asset::get(&filename)?;
// asset now contains the contents of index.html
```

Since the frontend artifacts first need to be built before they can be embedded, the build process is like a dependency graph:

```mermaid
flowchart LR
   A[frontend crate sources - rs, html] -- tailwind-build --> B[output.css]
   B -- trunk build --> C[frontend dist - WASM, JS, HTML, CSS]
   A -- trunk build --> C

   C -- Rust Embed --> D

   D[app crate sources - rs] -- cargo build --> X[app - binary]
   E[common crate sources - rs] -- cargo build --> X
```

First, tailwind is used to build the CSS file from the frontend sources that contain Tailwind classes.  
Then, the frontend sources are compiled using [trunk](https://trunkrs.dev), a WASM webapplication bundler for Rust.

Lastly, the main `app` crate can be compiled.  
The `app` crate depends on the `common` crate and embeds the previously built frontend artifacts.

The result is a single selfcontained binary called `app`.

In order to track these dependencies during the build process, a `Makefile` was used.  

Below you can find a simplified excerpt of it:

```makefile
# Build complete app for the native platform
$(build_native_release): $(dependencies) $(frontend_build)
 cd app && cargo build --release
app: $(build_native_release)

# Build complete app for armv7
target/armv7-unknown-linux-gnueabihf/wg_display: $(dependencies) $(frontend_build)
 cd app && cross build --release --target armv7-unknown-linux-gnueabihf
app_armv7: target/armv7-unknown-linux-gnueabihf/wg_display

## Build Tailwind CSS
$(tailwind_output_css): $(dependencies)
 cd frontend && npm run tailwind-build

## Build frontend artifacts using trunk
$(frontend_build): $(tailwind_output_css) $(dependencies)
 cd frontend && trunk build --release
```

<div class="page"/>

### Tests

TODO

### Continuous integration

Continuous integration was implemented using GitHub Actions.  
There were the following requirements:

- The tests should be run on every push to a feature branch.
- The project should be built for all supported targets on every push to a feature branch.
- A new release should be created for every new version tag on the main branch.

To achieve this, three seperate workflows were created.

![GitHub release](images/github_release.png)

The above image shows a release that was automatically created.  
It includes the individal commit messages going into the release and the cross compiled binaries for all supported targets.

### Deployment

Having made the decision to compile the project down to a single binary, the deployment process is very simple.  
The binary can simply be copied to the Raspberry Pi and executed.  

This process is described in the `README` of the project.  
It can be [found](#readmemd) in the appendix.

<div class="page"/>

## Results

<div class="page"/>

## Future extensions

The current state of the project provides a good foundation for future extensions.  
Several features were not implemented due to time constraints but would greatly improve the user experience.

- **Adding more widgets**  
  Currently, only a few widgets are implemented.  
  Implementing a few more widgets could help to make the project more popular.
- **Supportting more widget output formats**  
  Currently, widgets can only output their content as text.  
  Supporting more output formats would allow for more interesting widgets.  
- **User authentication**  
  The current state of the project does not provide any authentication.  
  This means that anyone with access to the network can change the configuration.  
  This is not a problem for a "WG" but could be a problem when the display is installed in larger networks.  
  A possible solution would be to implement a simple authentication mechanism using a username and password.  
- **Configuring WiFi credentials**  
  Currently, there is no way for the user to configure the WiFi credentials.  
  This means that the WiFi credentials need to be configured manually on the Raspberry Pi.  
  This could be part of the configuration page.  
  The Raspberry Pi would initially have to create it's own WiFi network for the user to connect to and configure the WiFi credentials.
- **Updating the application**  
  Currently, the application needs to be updated manually.  
  This could be done by adding a button to the configuration page that would trigger an update.  
  The update would then be downloaded and installed automatically.  
  This would also require the application to be able to update itself.  
  This could be achieved by using a crate like [self_update](https://crates.io/crates/self_update).
- **Dynamically loading widgets**  
  Currently, the widgets are statically defined in the `app` crate.  
  Newly added widgets require a new release of the application.  
  This could be changed by dynamically loading the widgets from a directory.  
  This could be achieved by using a crate like [libloading](https://crates.io/crates/libloading).

<div class="page"/>

## Glossary

- Raspberry Pi: A small single-board computer developed in the UK by the Raspberry Pi Foundation.
- WG: German abbreviation for "Wohngemeinschaft" (shared flat)
- Commit: A commit is a snapshot of the repository at a point in time.  
  It contains all the changes that were made to the repository since the last commit.
- Branch: Used to develop features in isolation from each other.
- Pull request: A pull request is a request to merge a branch into another branch.
- Crate: A crate is a compilation unit in Rust.  
  A crate can be either a binary or a library.
- Cargo: The Rust package manager.
- Makefile: A file that contains a set of directives used by a program called make for automatically building a software program.
- Cross compilation: Cross compilation is the process of compiling a program on one computer to run on a different computer.
- WebAssembly: WebAssembly (abbreviated Wasm) is a binary instruction format for a stack-based virtual machine.  
  Wasm is designed as a portable compilation target for programming languages, enabling deployment on the web for client and server applications.

<div class="page"/>

## Appendix

### README.md

:[README.md](../README.md)

<div class="page"/>

### Documentation on how to write a new widget

:[widget.md](write_new_widget.md)

<div class="page"/>

## Declaration of Authorship

![declaration_of_authorship](declaration_of_authorship.png)
