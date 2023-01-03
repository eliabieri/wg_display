## Goal

### General idea

The goal of this project is to build a small, wall mountable device, that gives an overview over certain information previously selected by the user.  
These pieces of information are provided by widgets, that can be activated or deactivated by the user.
The information is displayed on a small 5" monitor that is connected to a Raspberry Pi on its back.  

### What it is

- It tries to be as minimal as possible, both in its functionality, and it's visual appearance
- It should be easily extensible
- It is completely open-source and free from any tracking and analytics code

### What it's not

- It does not want to be a replacement for a Google Home or your smartphone.

### Example widgets

The following ideas could be packages into a widget

- Public transport information (when does the next bus depart from the closest bus stop)
- The current temperature of the Aare (a river in Bern, CH, popular for swimming)
- The capacity of the Sauna Lorrainebad
- The next event in a local nightclub

## MVP delivered in course `Project 2`

- Software is running stable on Raspberry Pi with attached screen
- Several widgets showcasing that is possible are implemented (Aare temperature, nightclub line-up, trash calendar, public transport)
- The software is well documented and released as open-source
- Contributors have enough information to develop their own extension
- A web dashboard allowing the user to configure the widgets is implemented. The user shall be able to enable and disable widgets, configure their settings

## Techstack

### Languages

This project aims to be completely implemented in Rust.  
This decision was taken for various reasons:

- The applications needs to be rock solid and should consume as little resources as possible
- Rust provides a fantastic developer experience
- Rust has a big ecosystem, providing crates for all aspects of the software

### Main Rust crates

- [cursive](https://github.com/gyscos/cursive) for rendering the display output
- [yew](https://crates.io/crates/yew) for the dashboard frontend
- [rocket](https://crates.io/crates/rocket) for the dashboard backend
- [cross](https://github.com/cross-rs/cross) for simplified cross-compilation
- [tokio](https://tokio.rs) asynchronous runtime
- [sled](http://sled.rs) embedded database for storing the configuration

### Other dependencies

- [tailwindcss](https://tailwindcss.com) for styling the dashboard

## System architecture

### Configuration backend

#### Responsibilities

- Serve frontend (html, css, webasm)
- Provide REST API for frontend to read and write configuration parameters

### Configuration frontend

#### Responsibilities

- Provide UI and logic for user to configure displayed information, WiFi credentials and more
- Interact with REST API provided by the backend

### Renderer

#### Responsibilities

- Render information to a terminal UI

### Scheduler

#### Responsibilities

- Schedule webserver, backend, widget updates and render intents

## Hardware components

The hardware build should be as simple as possible.  
This can be achieved by using commodity hardware that can just be plugged together.

### Main board

The software runs on a Raspberry Pi since it has enough computing power for our needs, is energy efficient and readily available.  
All models can be used, but this project focuses on models featuring an armv7 instruction set or higher, as this greatly simplifies cross compilation.

### Display

For the display, a DSI (Display Serial Interface) display was chosen.  
It has the advantage of not needing a driver, which further simplifies the deployment.
The following displays have been evaluated:

- [Maxgeek 4.3/5.0/7 Zoll 800*480 TFT MIPI DSI](https://de.aliexpress.com/item/1005003739648722.html?gatewayAdapt=glo2deu)
