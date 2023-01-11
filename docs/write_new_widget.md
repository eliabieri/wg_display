# Writing a new widget

## What is a widget

A widget represents a piece information displayed on the display.  
It has a name and a corresponding value, that is updated dynamically.  
Additionally, a widget consumes a configuration, that is entered by the user via the dashboard.  

## What is needed to build a widget

- A Yew component that visualizes the configuration options in the dashboard.
- An implementation of the `Widget` trait. In here, the value of the widget is fetched or computed.

```text
💡 Simple widgets that don't need any special  
configuration, can use the DefaultWidgetConfig component,  
which only allows for the widget to be enabled or disabled
```

```text
💡 Widgets that need a configuration input from the  
user should instead use the WidgetConfig component  
and pass in a custom  component that renders  
the desired configuration unput
```

## 🚦 Step by step

### 1. Initial steps

In the `common` trait, the meta data of the widget needs to be registered.  
Simply extend the [WidgetMetaData](../common/src/widget_meta_data.rs) enum implementation for your widget.

Next, extend the [WidgetConfiguration](../common/src/models.rs) struct to hold the configuration for your widget. If you expect no user configuration besides the enabled / disabled state, use the [BaseWidgetConfig](../common/src/models.rs).

Else, implement your own configuration struct, that houses a base configuration and your extensions.  
An example:

```rust
pub struct PublicTransportConfig {
    pub base_config: BaseWidgetConfig,
    pub from: String,
    pub to: String,
    pub num_connections_to_show: u8,
}
```

<div class="page"/>

```text
💡 Configuration structs need to  
#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
```

Lastly, you need to extend the `Reducible` implementation for the [SystemConfiguration](../common/src/models.rs) to allow setting your configuration through a `SystemConfigurationAction`.  
It should be straight forward by simply looking at the other implementations.

### 2. Adding a default configuration component

If you expect no user configuration besides the enabled / disabled state, adding a configuration component is simple. Simply add the following fragment (filled with your values) to the [MainComponent](../frontend/src/main.rs):  

```html
<ConfigCardComponent>
    <DefaultWidgetConfigComponent
        meta_data={WidgetMetaData::YourWidget}
        config={system_config.widget_config
                .your_widget_config.clone()}
        on_change={Callback::from(captures::capture!(
                   clone system_config, |config| {
            system_config.dispatch(
                SystemConfigurationAction::SetYourConfig(config)
            );
    }))}
    />
</ConfigCardComponent>
```

### 2. Implementing a custom configuration component

If you do need the user to be able to configure something, you fist need to create a new file in the [frontend/src/components](../frontend/src/components/) directory. It will house your configuration component implementation.

Next, implement your HTML component.  
You can use normal HTML elements like `<input>` or `<select>`.  
To style ✨ your component, use [Tailwind CSS](https://tailwindcss.com).
The [PublicTransportConfigComponent](../frontend/src/components/public_transport_config.rs) serves as a good example here.

<div class="page"/>

Implementing the change callbacks is a bit more tricky.  
Let's assume you have a `<select>` element, that allows the user to select a value.

```rust
let on_changed = move |event: Event| {
    let input = event
        .target()
        .and_then(|t| t.dyn_into::<HtmlSelectElement>().ok());
    if let Some(user_supplied_value) = input {
        on_change.emit(YourConfig {
            base_config: base.clone(),
            user_supplied_value: user_supplied_value.clone(),
        });
    }
};
```

We first need to get the `target` from the `event` and cast it into a `HtmlSelectElement`. We then unpack the value of the `HtmlSelectElement` and use it to emit a new configuration to the `on_change` Callback field.
Have a look at the [yew events docs](https://yew.rs/docs/concepts/html/events) for more information on how to handle events from other HTML elements.

Finally, we need to add the component to the [MainComponent](../frontend/src/main.rs).

### 3. Implementing the widget

For the final part, you need to implement the actual widget that will be rendered to the screen. This is done by implementing the [Widget](../app/src/renderer/widgets/base.rs) trait. There's not much magic here.  

The widget features an asynchronous `update` method, that is called every other second. In here, the widget is expected to fetch or compute the value it should display. The value should be stored in a field of the widget struct, so it can be returned in the `get_content` method.

```text
💡 Most widgets don't need to recompute their value every second.  
It is thus their responsibility, to implement a caching mechanism.  
Have a look at the existing widgets for inspiration.
```

```text
⚠️ In case an error occurs, the widget should  
set its value to a short error message.
```

You then need to register your widget: add it to [app/src/renderer/widgets/mod.rs](../app/src/renderer/widgets/mod.rs) and extend the [config_to_widgets](../app/src/renderer/config_to_widgets.rs) function.

🥳 Thats it! You've built your own widget
