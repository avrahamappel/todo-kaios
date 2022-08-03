use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub title: String,
}

#[function_component(Header)]
pub fn header(Props { title }: &Props) -> Html {
    html! {
        <header class="header">
            <span>{title.as_str()}</span>
        </header>
    }
}
