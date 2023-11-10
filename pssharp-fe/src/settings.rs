use wasm_bindgen::{
    prelude::{wasm_bindgen, Closure},
    JsCast,
};
use web_sys::{window, Document, Element, HtmlElement, MouseEvent};

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
            
        <button id="settings-close-x" class="button-rounded bg-burgundy text-2x button-events" style="width: 2.5rem; height: 2.5rem;">
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

    set_dark_mode_system_preference_label(&div);
    register_close_handlers(&div);
    register_reset_handler(&div);
    register_dark_mode_handlers(&div);

    document
        .body()
        .expect("Could not get body element.")
        .append_child(&div)
        .expect("Could not append div to body.");
}

fn register_dark_mode_handlers(div: &HtmlElement) {
    let on_dark_mode_on = Closure::<dyn Fn(MouseEvent)>::new(|_| {
        dark_mode::set_app_preference(dark_mode::DarkModePreference::Dark);
    });

    div.query_selector("[id='dark-mode-on']")
        .expect("Could not get dark-mode-on button.")
        .expect("Could not get dark-mode-on button.")
        .add_event_listener_with_callback("click", on_dark_mode_on.as_ref().unchecked_ref())
        .expect("Could not add event listener to dark-mode-on button.");

    on_dark_mode_on.forget();

    let on_dark_mode_off = Closure::<dyn Fn(MouseEvent)>::new(|_| {
        dark_mode::set_app_preference(dark_mode::DarkModePreference::Light);
    });

    div.query_selector("[id='dark-mode-off']")
        .expect("Could not get dark-mode-off button.")
        .expect("Could not get dark-mode-off button.")
        .add_event_listener_with_callback("click", on_dark_mode_off.as_ref().unchecked_ref())
        .expect("Could not add event listener to dark-mode-off button.");

    on_dark_mode_off.forget();

    let on_dark_mode_system = Closure::<dyn Fn(MouseEvent)>::new(|_| {
        dark_mode::reset_preference();
    });

    div.query_selector("[id='dark-mode-system']")
        .expect("Could not get dark-mode-system button.")
        .expect("Could not get dark-mode-system button.")
        .add_event_listener_with_callback("click", on_dark_mode_system.as_ref().unchecked_ref())
        .expect("Could not add event listener to dark-mode-system button.");

    on_dark_mode_system.forget();
}

fn register_reset_handler(div: &HtmlElement) {
    let on_reset_settings = Closure::<dyn Fn(MouseEvent)>::new(|_| {
        reset_settings();
    });

    div.query_selector("[id='settings-reset']")
        .expect("Could not get settings-reset button.")
        .expect("Could not get settings-reset button.")
        .add_event_listener_with_callback("click", on_reset_settings.as_ref().unchecked_ref())
        .expect("Could not add event listener to settings-reset button.");

    on_reset_settings.forget();
}

fn set_dark_mode_system_preference_label(div: &HtmlElement) {
    let dark_mode_system_preference =
        match dark_mode::get_preference_from_system(&window().unwrap()) {
            dark_mode::DarkModePreference::Light => "(Light)",
            dark_mode::DarkModePreference::Dark => "(Dark)",
        };

    div.query_selector("[id='dark-mode-system-preference']")
        .expect("Could not get dark-mode-system-preference span.")
        .expect("Could not get dark-mode-system-preference span.")
        .set_inner_html(dark_mode_system_preference);
}

fn register_close_handlers(div: &HtmlElement) {
    let on_close_settings = Closure::<dyn Fn(MouseEvent)>::new(|event: MouseEvent| {
        match event
            .target()
            .unwrap()
            .dyn_into::<HtmlElement>()
            .unwrap()
            .id()
            .as_str()
        {
            "backdrop" | "settings-close" | "settings-close-x" => {
                close_settings();
            }
            _ => {}
        };
    });

    div.query_selector("[id='settings-close']")
        .expect("Could not get settings-close button.")
        .expect("Could not get settings-close button.")
        .add_event_listener_with_callback("click", on_close_settings.as_ref().unchecked_ref())
        .expect("Could not add event listener to settings-close button.");
    div.query_selector("[id='settings-close-x']")
        .expect("Could not get settings-close-x button.")
        .expect("Could not get settings-close-x button.")
        .add_event_listener_with_callback("click", on_close_settings.as_ref().unchecked_ref())
        .expect("Could not add event listener to settings-close-x button.");
    div.add_event_listener_with_callback("click", on_close_settings.as_ref().unchecked_ref())
        .expect("Could not add event listener to backdrop.");

    on_close_settings.forget();
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
