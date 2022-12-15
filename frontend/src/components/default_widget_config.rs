use yew::prelude::*;

use common::models::BaseWidgetConfig;
use common::widget_meta_data::WidgetMetaData;

#[derive(Properties, PartialEq)]
pub struct DefaultWidgetConfigProps {
    pub meta_data: WidgetMetaData,
    pub config: BaseWidgetConfig,
    pub on_change: Callback<BaseWidgetConfig>,
}

#[function_component(DefaultWidgetConfigComponent)]
pub fn default_widget_config_component(props: &DefaultWidgetConfigProps) -> Html {
    let on_change = props.on_change.clone();
    let enabled = props.config.enabled;
    let toggle_enabled = move |_: MouseEvent| {
        on_change.emit(BaseWidgetConfig { enabled: !enabled });
    };

    html! {
        <div class="flex justify-between items-center">
            <div>
                <div class="text-white text-md font-medium">{{ props.meta_data.name()}}</div>
                <div class="text-slate-300 text-sm">{{ props.meta_data.description()}}</div>
            </div>
            <div class="pl-8">
                <input type={"checkbox"} checked={props.config.enabled} onclick={toggle_enabled}
                    class="h-4 w-4 rounded-md border-gray-300 text-indigo-600 focus:ring-indigo-500"
                />
            </div>
        </div>
    }
}
