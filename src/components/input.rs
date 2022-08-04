use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::virtual_dom::AttrValue;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub itype: AttrValue,
    pub label: String,
    pub focused: bool,
    pub on_input: Callback<InputEvent>,
}

#[function_component(Input)]
pub fn input(
    Props {
        itype,
        label,
        focused,
        on_input,
    }: &Props,
) -> Html {
    let node_ref = use_node_ref();

    if let Some(input) = node_ref.cast::<HtmlInputElement>() {
        if *focused {
            input.focus();
        } else {
            input.blur();
        }
    }

    html! {
        <div class="input">
            <input type={itype.clone()} ref={node_ref} oninput={on_input} />
            <label>{label}</label>
        </div>
    }
}
