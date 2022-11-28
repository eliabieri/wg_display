# Writing a new widget

## What is a widget

A widget represents an information displayed on the display.  
It has a name and a corresponding value, that is updated dinamically.  
Additionally, a widget consumes a configuration, that is entered by the user via the dashboard.  

## What is needed to build a widget

- A Yew component that visualizes the configuration options in the dashboard.
- An implementation of the `Widget` trait. In here, the value of the widget is fetched or computed.

```text
💡 Simple widgets can use the DefaultWidgetConfig component, that only allows for the widget to be enabled or disabled
```

## Learning by example
