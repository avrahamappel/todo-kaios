use gloo_utils::document;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub left: String,
    #[prop_or(Callback::from(|_| {}))]
    pub on_key_left: Callback<KeyboardEvent>,
    #[prop_or_default]
    pub center: String,
    #[prop_or(Callback::from(|_| {}))]
    pub on_key_center: Callback<KeyboardEvent>,
    #[prop_or_default]
    pub right: String,
    #[prop_or(Callback::from(|_| {}))]
    pub on_key_right: Callback<KeyboardEvent>,
}

#[function_component(SoftKey)]
pub fn softkey(props: &Props) -> Html {
    let props = props.clone();
    let Props {
        left,
        right,
        center,
        on_key_left,
        on_key_right,
        on_key_center,
    } = props;

    let handle_keydown =
        Closure::new::<Box<dyn FnMut(KeyboardEvent)>>(Box::new(move |evt| {
            match evt.key().as_ref() {
                "SoftLeft" => on_key_left.emit(evt),
                "Enter" => on_key_center.emit(evt),
                "SoftRight" => on_key_right.emit(evt),
                _ => {}
            }
        }));

    use_effect(move || {
        document()
            .add_event_listener_with_callback("keydown", handle_keydown.as_ref().unchecked_ref())
            .expect("Couldn't add event listener");

        move || {
            document()
                .remove_event_listener_with_callback(
                    "keydown",
                    handle_keydown.as_ref().unchecked_ref(),
                )
                .expect("Couldn't remove event listener");
        }
    });

    html! {
        <div class="softkey">
            <label class="left">{left}</label>
            <label class="center">{center}</label>
            <label class="right">{right}</label>
        </div>
    }
}
