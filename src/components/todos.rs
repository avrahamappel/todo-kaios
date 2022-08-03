use yew::prelude::*;

use crate::todo::Todo;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub todos: Vec<Todo>,
    pub current_index: usize,
}

#[function_component(Todos)]
pub fn todos(
    Props {
        todos,
        current_index,
    }: &Props,
) -> Html {
    let todos: Html = todos
        .iter()
        .enumerate()
        .map(|(index, todo)| {
            let classes = classes!(
                "todo",
                todo.completed.then_some("completed"),
                (*current_index == index).then_some("selected")
            );
            html! {
                <span class={classes} key={index}>{todo.name.as_str()}</span>
            }
        })
        .collect();

    html! {
        <div class="todos">
            {todos}
        </div>
    }
}
