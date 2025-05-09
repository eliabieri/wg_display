# 🛠️ Implementing a new widget

## What is a widget

A widget represents a piece of information displayed on the display.  
It has a name and a corresponding value, that is updated dynamically.  
Additionally, a widget consumes a configuration, that is entered by the user via the dashboard.  

## WebAssembly

Widgets are [WebAssembly components](https://github.com/WebAssembly/component-model), that implement the interface defined by the `exports` of the [wg_display_widget_wit](https://eliabieri.github.io/wg_display_widget_wit/).  
They are loaded by the WG Display application and run using the [wasmtime](https://wasmtime.dev) WebAssembly runtime.

Using WebAssembly, as the basis for the widgets, has the following advantages:

- Widgets are sandboxed and can't directly access the host system
- Widgets can be written in any language that compiles to WebAssembly
- Widgets can run on any platform that is supported by `wasmtime`

## 🚦 How to get started

The starting point for implementing widgets are the template repositories.  
They contain the instructions on how to build and distribute the widget.

| Language                                                    | Remarks      |
| ----------------------------------------------------------- | ------------ |
| [Rust](https://github.com/eliabieri/wg_display_widget_rs)   | -            |
| [Python](https://github.com/eliabieri/wg_display_widget_py) | Experimental |

## 📦 Distribute the widget

See [Add your widget to the store](https://github.com/eliabieri/wg_display_widget_rs/blob/main/README.md#-add-your-widget-to-the-store) for more information.
