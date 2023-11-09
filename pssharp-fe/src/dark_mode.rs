use crate::console_log;
use wasm_bindgen::{
    prelude::{wasm_bindgen, Closure},
    JsCast, JsValue,
};
use web_sys::{window, HtmlElement, Storage, Window};

#[derive(Debug)]
#[wasm_bindgen]
pub enum DarkModePreference {
    Dark,
    Light,
}

#[derive(Debug)]
#[wasm_bindgen]
pub enum DarkModeSource {
    System,
    Storage,
}

fn get_preference_from_storage(storage: &Storage) -> Option<DarkModePreference> {
    let storage_preference = storage
        .get_item("dark-mode")
        .expect("Could not get dark mode preference from local storage");

    match storage_preference {
        Some(val) if val == "true" => Some(DarkModePreference::Dark),
        Some(_) => Some(DarkModePreference::Light),
        None => None,
    }
}

pub fn get_preference_from_system(window: &Window) -> DarkModePreference {
    match window
        .match_media("(prefers-color-scheme: dark)")
        .unwrap()
        .unwrap()
        .matches()
    {
        true => DarkModePreference::Dark,
        false => DarkModePreference::Light,
    }
}

#[wasm_bindgen(js_name = getDarkModePreference)]
pub fn get_preference() -> DarkModePreference {
    let window = window().expect("no global `window` exists");

    let storage = window
        .local_storage()
        .expect("Could not get local storage")
        .expect("Could not get local storage");

    get_preference_from_storage(&storage).unwrap_or_else(|| get_preference_from_system(&window))
}

#[wasm_bindgen(js_name = setDarkModeAppPreference)]
pub fn set_app_preference(preference: DarkModePreference) {
    let window = window().expect("no global `window` exists");

    let storage = window
        .local_storage()
        .expect("Could not get local storage")
        .expect("Could not get local storage");

    let storage_preference = match preference {
        DarkModePreference::Dark => "true",
        DarkModePreference::Light => "false",
    };

    storage
        .set_item("dark-mode", storage_preference)
        .expect("Could not set dark mode preference in local storage");

    set_styles_from_preference();
}

#[wasm_bindgen(js_name = resetDarkModePreference)]
pub fn reset_preference() {
    let window = window().expect("no global `window` exists");

    let storage = window
        .local_storage()
        .expect("Could not get local storage")
        .expect("Could not get local storage");

    storage
        .remove_item("dark-mode")
        .expect("Could not remove dark mode preference from local storage");

    set_styles_from_preference();
}

fn set_dark_mode_styles(body: &HtmlElement, dark_mode: DarkModePreference) {
    match dark_mode {
        DarkModePreference::Dark => {
            body.class_list().add_1("dark").unwrap();
            body.class_list().remove_1("light").unwrap();
        }
        DarkModePreference::Light => {
            body.class_list().add_1("light").unwrap();
            body.class_list().remove_1("dark").unwrap();
        }
    }
}

pub(crate) fn set_styles_from_preference() {
    let window = window().expect("no global `window` exists");

    let body = window
        .document()
        .expect("Could not get document")
        .body()
        .expect("Could not get body");

    let preference = get_preference();

    set_dark_mode_styles(&body, preference);
}

pub struct DarkModeWatch<'a> {
    window: &'a Window,
    storage_change_closure: Closure<dyn Fn()>,
}

impl<'a> DarkModeWatch<'a> {
    fn new(window: &'a Window, storage_change_closure: Closure<dyn Fn()>) -> Self {
        Self {
            window,
            storage_change_closure,
        }
    }

    pub fn forget(self) {
        self.storage_change_closure.forget();
    }

    pub fn cancel(self) {
        self.window
            .remove_event_listener_with_callback(
                "storage",
                self.storage_change_closure.as_ref().unchecked_ref(),
            )
            .expect("Could not unsubscribe from window storage");
    }

    pub fn window(&self) -> &'a Window {
        self.window
    }
}

pub(crate) fn watch_storage(window: &Window) -> DarkModeWatch<'_> {
    let on_storage_change = Closure::<dyn Fn()>::new(|| {
        set_styles_from_preference();
    });

    window
        .add_event_listener_with_callback("storage", on_storage_change.as_ref().unchecked_ref())
        .expect("Could not subscribe to window storage");

    DarkModeWatch::new(&window, on_storage_change)
}
