
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
  - [Overview](#overview)
  - [Build process](#build-process)
  - [Main Rust crates used in the project](#main-rust-crates-used-in-the-project)
  - [Other dependencies](#other-dependencies)
  - [Configuration frontend](#configuration-frontend)
  - [Cross compilation](#cross-compilation)
  - [Build process](#build-process-1)
  - [Tests](#tests)
  - [Continuous integration](#continuous-integration)
  - [Deployment](#deployment)
- [Results](#results)
- [Glossary](#glossary)
- [References](#references)
- [Appendix](#appendix)
  - [The main README.md of the project](#the-main-readmemd-of-the-project)
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

### Deliverables

The deliverables for this project are:

- The codebase of a working prototype of the application
- Documentation of the codebase
- A GitHub repository that forms the basis for growing a community around the project
- A project report that describes the project and the software architecture
- A presentation that explains the project and the software architecture

<div class="page"/>

## Requirements

### Functional

TODO

### Non-functional

- The sofware should be robust and crash as little as possible.
- Users should be able to configure the software to their needs.
- Configuration changes should be applied without restarting the software.
- The software should be well documented.
- The documentation should be fun to read.
- The project should be open source and available on GitHub.
- The project should be easy to contribute to.

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

### Repository settings

The GitHub project was configured to enforce several rules:

- No user should be allowed to push directly to a non feature branch.
  This was achieved by setting a branch protection rule that required a pull request before merging.
- All pull requests required all status checks to pass before they could be merged.
  This was achieved by setting a branch protection rule that required all status checks to pass before merging.
- Prevent force pushes on non feature branches.
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
Since Rust can be compiled to web assembly, it could also be used to write the configuration frontend.
Furthermore, the developer experience is very good. This is reflected in the [Stack Overflow developer survey](https://survey.stackoverflow.co/2022/), where Rust is ranked as the most loved programming language for the past seven years.
Last but not least, Rust has a very active community and a big ecosystem of `crates` (libraries) that simplified the development of the project greatly.

Given all this, Rust presented itself as the perfect choice for this project.

### Overview

Configuration backend

- Serve frontend (html, css, webasm)
- Provide REST API for frontend to read and write configuration parameters

Configuration frontend

- Provide UI and logic for user to configure widgets and other system aspects
- Interact with REST API provided by the backend

Renderer

- Render information to a terminal UI


### Build process



### Main Rust crates used in the project

- [cursive](https://github.com/gyscos/cursive) for rendering the display output
- [yew](https://crates.io/crates/yew) for the configuration frontend
- [rocket](https://crates.io/crates/rocket) for the configuration backend (server and REST API)
- [cross](https://github.com/cross-rs/cross) for simplified cross-compilation
- [tokio](https://tokio.rs) asynchronous runtime
- [sled](http://sled.rs) embedded database for storing the configuration

### Other dependencies

- [tailwindcss](https://tailwindcss.com) for styling the configuration frontend

### Configuration frontend

The configuration frontend is a web application that allows the user to configure the WG Display.  
The frontend was written using the [Yew](https://yew.rs/) framework.  
Yew is a component-based framework for writing web applications in Rust.  

### Cross compilation

Since this project is expected to run on all Raspberry Pi models, cross compilation was used to build the project for all supported targets.  

The following targets are supported:

- arm-unknown-linux-gnueabihf (Raspberry PI Zero 1 / Zero W / Zero WH)
- armv7-unknown-linux-gnueabihf (Raspberry PI 2 / 3 / 4 / Zero 2 W)
- native (whatever the build machine is)

To achieve this, the [cross](https://github.com/cross-rs/cross) project was used.  
This project allows to build Rust projects for different targets using prebuilt Docker images.  
Not having to manually install the required toolchains for each target is a huge advantage.  

### Build process

```mermaid
flowchart LR
   A[frontend crate sources - rs, html] -- tailwind-build --> B[output.css]
   B -- trunk build --> C[frontend dist - WASM, JS, HTML, CSS]
   A -- trunk build --> C

   C -- Rust Embed --> D

   D[app crate sources - rs] -- cargo build --> X[app - binary]
   E[common crate sources - rs] -- cargo build --> X
```

```makefile
# Make does not offer a recursive wildcard function, so here's one:
rwildcard=$(wildcard $1$2) $(foreach d,$(wildcard $1*),$(call rwildcard,$d/,$2))

frontend_dist = frontend/dist
tailwind_output_css = $(frontend_dist)/$(wildcard output-*.css)
yew_index_html = $(frontend_dist)/index.html
frontend_build = $(yew_index_html)

build_native_release = app/target/release/wg_display
build_native_release_debug = app/target/debug/wg_display

app:

.PHONY: clean
clean:
 rm -rf app/target
 rm -rf common/target
 rm -rf frontend/dist

## Build the application
dependencies = \
 $(call rwildcard,app/src/,*.rs) \
 $(call rwildcard,common/src/,*.rs) \
 $(call rwildcard,frontend/src,*.rs) \
 $(call rwildcard,app/src/,*.rs) \
 $(call rwildcard,common/src/,*.rs) \
 $(call rwildcard,app/,Cargo.*) \
 $(call rwildcard,common/,Cargo.*) \
 $(call rwildcard,frontend/,Cargo.*)

# Generate docs
docs: $(dependencies) $(frontend_build)
 cd app && cargo doc --no-deps
 cd common && cargo doc --no-deps
 cd frontend && cargo doc --no-deps

# Build complete app for the native platform
$(build_native_release): $(dependencies) $(frontend_build)
 cd app && cargo build --release
app: $(build_native_release)

# Build complete app for the native platform in debug mode
$(build_native_release_debug): $(dependencies) $(frontend_build)
 cd app && cargo build
app_debug: $(build_native_release_debug)

# Build complete app for arm (Raspberry Pi 2/3/4)
target/armv7-unknown-linux-gnueabihf/wg_display: $(dependencies) $(frontend_build)
 cd app && cross build --release --target armv7-unknown-linux-gnueabihf
app_armv7: target/armv7-unknown-linux-gnueabihf/wg_display

# Build complete app for arm (Raspberry Pi 0/1)
target/arm-unknown-linux-gnueabihf/wg_display: $(dependencies) $(frontend_build)
 cd app && cross build --release --target arm-unknown-linux-gnueabihf
app_arm: target/arm-unknown-linux-gnueabihf/wg_display

## Build frontend using trunk
dependencies = \
 $(call rwildcard,frontend/src/,*.rs) \
 frontend/index.html \
 frontend/package.json
$(tailwind_output_css): $(dependencies)
 # Force regeneration
 rm -rf $(tailwind_output_css)
 cd frontend && npm run tailwind-build

$(frontend_build): $(tailwind_output_css) $(dependencies)
 cd frontend && trunk build --release
```

TODO

### Tests

TODO

### Continuous integration

Continuous integration was implemented using GitHub Actions.  
There were the following requirements:

- The tests should be run on every push to a feature branch.
- The project should be built for all supported targets on every push to a feature branch.
- A new release should be created for every new version tag on the main branch.

To achieve this, three seperate workflows were created.

The following workflow definition is used to build and publish a new release on every version tag on the main branch:

```yaml
name: Build release
on:
  push:
    tags:
      # Push events to matching v*, i.e. v1.0, v20.15.10
      - 'v*'
env:
  CARGO_TERM_COLOR: always
jobs:
  build:
    runs-on: ubuntu-latest
    permissions:
      contents: write
    steps:
    - uses: actions/checkout@v3
    - name: Add WASM target
      run: rustup target add wasm32-unknown-unknown
    - name: Install cargo dependencies
      run: cargo install cross --locked &&
        cargo install --locked trunk
    - name: Install tailwindcss
      run: cd frontend && npm install
    - name: Build for Raspberry Pi 2/3/4)
      run: make app_armv7
    - name: Build for Raspberry Pi 0/1
      run: make app_arm
    - name: Rename artifacts
      run: |
        mv app/target/armv7-unknown-linux-gnueabihf/release/wg_display app/target/armv7-unknown-linux-gnueabihf/release/wg-display-armv7-unknown-linux-gnueabihf
        mv app/target/arm-unknown-linux-gnueabihf/release/wg_display app/target/arm-unknown-linux-gnueabihf/release/wg-display-arm-unknown-linux-gnueabihf
    - uses: ncipollo/release-action@v1
      with:
          artifacts: "app/target/armv7-unknown-linux-gnueabihf/release/wg-display-armv7-unknown-linux-gnueabihf, app/target/arm-unknown-linux-gnueabihf/release/wg-display-arm-unknown-linux-gnueabihf"
          artifactErrorsFailBuild: true
```

### Deployment

TODO

<div class="page"/>

## Results

TODO

<div class="page"/>

## Glossary

- Raspberry Pi: A small single-board computer developed in the UK by the Raspberry Pi Foundation.
- WG: German abbreviation for "Wohngemeinschaft" (shared flat)
- Commit: A commit is a snapshot of the repository at a point in time.  
  It contains all the changes that were made to the repository since the last commit.
- Branch: Used to develop features in isolation from each other.
- Pull request: A pull request is a request to merge a branch into another branch.

<div class="page"/>

## References

TODO

<div class="page"/>

## Appendix

### The main README.md of the project

:[README.md](../README.md)

<div class="page"/>

### Documentation on how to write a new widget

:[widget.md](write_new_widget.md)

<div class="page"/>

## Declaration of Authorship

![declaration_of_authorship](declaration_of_authorship.png)
