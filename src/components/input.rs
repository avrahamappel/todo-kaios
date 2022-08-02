use yew::prelude::*;
use yew::virtual_dom::AttrValue;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub itype: AttrValue,
    pub label: String,
}

#[function_component(Input)]
pub fn input(props: &Props) -> Html {
    html! {
        <div class="input">
            <input type={props.itype} nav-selectable="true" />
            <label>{props.label}</label>
        </div>
    }
}
