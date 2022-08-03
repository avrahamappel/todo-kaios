use yew::prelude::*;
use yew::virtual_dom::AttrValue;

#[derive(PartialEq, Eq, Properties)]
pub struct Props {
    pub itype: AttrValue,
    pub label: String,
}

#[function_component(Input)]
pub fn input(Props { itype, label }: &Props) -> Html {
    html! {
        <div class="input">
            <input type={itype.clone()} nav-selectable="true" />
            <label>{label}</label>
        </div>
    }
}
