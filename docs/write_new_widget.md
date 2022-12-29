# Writing a new widget

## What is a widget

A widget represents a piece information displayed on the display.  
It has a name and a corresponding value, that is updated dynamically.  
Additionally, a widget consumes a configuration, that is entered by the user via the dashboard.  

## What is needed to build a widget

- A Yew component that visualizes the configuration options in the dashboard.
- An implementation of the `Widget` trait. In here, the value of the widget is fetched or computed.

```text
💡 Simple widgets that don't need any special configuration, can use the DefaultWidgetConfig component, which only allows for the widget to be enabled or disabled
```

```text
💡 Widgets that need a configuration input from the user should instead use the WidgetConfig component and pass in a custom component that renders the desired configuration unput
```

## Learning by example

```html
<ConfigCardComponent>
    <DefaultWidgetConfigComponent
        widget_name={WidgetName::Time.as_str()}
        config={system_config.widget_config.time_config.clone()}
    on_change={update_time_config}
    />
</ConfigCardComponent>
```