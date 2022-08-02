use gloo_utils::document;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub left: String,
    #[prop_or(Box::new(|_| {}))]
    pub on_key_left: Box<dyn Fn(KeyboardEvent)>,
    #[prop_or_default]
    pub center: String,
    #[prop_or(Box::new(|_| {}))]
    pub on_key_center: Box<dyn Fn(KeyboardEvent)>,
    #[prop_or_default]
    pub right: String,
    #[prop_or(Box::new(|_| {}))]
    pub on_key_right: Box<dyn Fn(KeyboardEvent)>,
}

#[function_component(SoftKey)]
pub fn softkey(props: &Props) -> Html {
    let handle_keydown = Closure::new(|evt: KeyboardEvent| match evt.key().as_ref() {
        "SoftLeft" => (props.on_key_left)(evt),
        "Enter" => (props.on_key_center)(evt),
        "SoftRight" => (props.on_key_right)(evt),
    })
    .as_ref()
    .unchecked_ref();

    use_effect(|| {
        document().add_event_listener_with_callback("keydown", handle_keydown);
        || {
            document().remove_event_listener_with_callback("keydown", handle_keydown);
        }
    });

    html! {
        <div class="softkey">
            <label class="left">{props.left}</label>
            <label class="center">{props.center}</label>
            <label class="right">{props.right}</label>
        </div>
    }
}
