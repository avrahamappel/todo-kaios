use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub title: String,
}

#[function_component(Header)]
pub fn header(props: &Props) -> Html {
    html! {
        <header class="header">
            <span>{props.title}</span>
        </header>
    }
}
