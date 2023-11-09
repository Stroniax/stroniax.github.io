use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{js_sys::Array, window, Document, Element, HtmlElement, MouseEvent};

pub fn init_tooltip() {
    let document = window().unwrap().document().unwrap();

    let tooltips = document
        .query_selector_all("[tooltip]")
        .expect("Unable to query tooltips.");

    let on_mouse_enter = Closure::<dyn Fn(MouseEvent)>::new(|event: MouseEvent| {
        let target = event.target().unwrap().dyn_into::<Element>().unwrap();
        let tooltip = target.get_attribute("tooltip").unwrap();

        let tooltip_node = window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("div")
            .expect("Unable to create tooltip node.");

        tooltip_node.set_class_name("tooltip");
        tooltip_node.set_inner_html(tooltip.as_str());

        // let target_rect = target.get_bounding_client_rect();

        // tooltip_node.set_style("top", format!("{}px", target_rect.top()));
        // tooltip_node.set_style("left", format!("{}px", target_rect.left()));

        target.append_child(&tooltip_node).unwrap();
    });

    let on_mouse_leave = Closure::<dyn Fn(MouseEvent)>::new(|event: MouseEvent| {
        event
            .target()
            .expect("mouseleave event should have target.")
            .dyn_into::<Element>()
            .expect("target should be an element.")
            .query_selector(".tooltip")
            .expect("Unable to query tooltips.")
            .map(|node| {
                node.dyn_into::<HtmlElement>().unwrap().remove();
            });
    });

    for i in 0..tooltips.length() {
        let node = tooltips.item(i).expect("Unable to get tooltip node.");

        node.add_event_listener_with_callback(
            "mouseenter",
            on_mouse_enter.as_ref().unchecked_ref(),
        )
        .expect("Unable to add mouseenter event listener.");
        node.add_event_listener_with_callback(
            "mouseleave",
            on_mouse_leave.as_ref().unchecked_ref(),
        )
        .expect("Unable to add mouseleave event listener.");
    }

    on_mouse_enter.forget();
    on_mouse_leave.forget();
}
