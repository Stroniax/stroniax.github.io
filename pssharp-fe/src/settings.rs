use wasm_bindgen::{
    prelude::{wasm_bindgen, Closure},
    JsCast,
};
use web_sys::{window, Document, HtmlElement, MouseEvent};

use crate::dark_mode;

pub fn open_settings(document: &Document) {
    let div = document
        .create_element("div")
        .expect("Could not create div element for app settings modal.")
        .dyn_into::<HtmlElement>()
        .unwrap();
    div.set_id("backdrop");

    div.set_inner_html(r#"
<div id="webapp-settings-modal">
    <div class="flex-row flex-align-center justify-between">
        <h3 class="text-2x">Settings</h3>
            
        <button class="button-rounded bg-burgundy text-2x button-events" style="width: 2.5rem; height: 2.5rem;" onclick="closeSettings()">
            X
        </button>
    </div>

    <div class="flex-col gap-2">

        <div class="flex-row justify-between rounded-sm bg-default">
            <button id="dark-mode-on" class="flex-1 bg-dark rounded-sm-left">
                Dark
            </button>

            <button id="dark-mode-off" class="flex-1 bg-light border-left border-right">
                Light
            </button>

            <button id="dark-mode-system" class="flex-1 bg-system rounded-sm-right">
                Use System Preference<span id="dark-mode-system-preference"></span>
            </button>
        </div>

        <button id="settings-reset" class="action-button bg-warn">
            Reset Settings
        </button>

        <div class="flex-row">
            <button id="settings-close" class="action-button" style="background-color: #3a3a3a;">
                Close
            </button>
        </div>
    </div>
</div>"#);

    // set the dark-mode-system-preference span to the current system preference
    let dark_mode_system_preference =
        match dark_mode::get_preference_from_system(&window().unwrap()) {
            dark_mode::DarkModePreference::Light => "(Light)",
            dark_mode::DarkModePreference::Dark => "(Dark)",
        };

    div.query_selector("[id='dark-mode-system-preference']")
        .expect("Could not get dark-mode-system-preference span.")
        .expect("Could not get dark-mode-system-preference span.")
        .set_inner_html(dark_mode_system_preference);

    let on_backdrop_click = Closure::<dyn Fn(MouseEvent)>::new(|event: MouseEvent| {
        event
            .target()
            .expect("Could not get target of event.")
            .dyn_into::<HtmlElement>()
            .expect("Could not convert event target to HtmlElement.")
            .remove();
    });
    div.add_event_listener_with_callback("click", on_backdrop_click.as_ref().unchecked_ref())
        .expect("Could not add event listener to backdrop.");
    on_backdrop_click.forget();

    document
        .body()
        .expect("Could not get body element.")
        .append_child(&div)
        .expect("Could not append div to body.");
}

pub fn reset_settings() {
    window()
        .expect("Could not get window.")
        .local_storage()
        .expect("Could not get local storage.")
        .expect("Could dnot get local storage.")
        .clear()
        .expect("Could not clear local storage.");

    crate::dark_mode::set_styles_from_preference();
}

#[wasm_bindgen(js_name = closeSettings)]
pub fn close_settings() {
    window()
        .expect("Could not get window.")
        .document()
        .expect("Could not get document.")
        .get_element_by_id("backdrop")
        .map(|element| {
            element
                .dyn_into::<HtmlElement>()
                .expect("Could not get backdrop as HtmlElement.")
                .remove()
        });
}

pub fn init_settings() {
    // add 'click' event listener to the 'id="settings-trigger"' element
    let settings_trigger = window()
        .expect("Could not get window.")
        .document()
        .expect("Could not get document.")
        .get_element_by_id("settings-trigger")
        .expect("Could not get element with id 'settings-trigger'.");

    let on_settings_trigger_click = Closure::<dyn Fn(MouseEvent)>::new(|_| {
        open_settings(
            &window()
                .expect("Could not get window.")
                .document()
                .expect("Could not get document."),
        );
    });

    settings_trigger
        .add_event_listener_with_callback(
            "click",
            on_settings_trigger_click.as_ref().unchecked_ref(),
        )
        .expect("Could not add event listener to settings trigger.");

    on_settings_trigger_click.forget();
}
