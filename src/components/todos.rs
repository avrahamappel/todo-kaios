use yew::prelude::*;

use crate::todo::Todo;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub todos: Vec<Todo>,
    pub maybe_index: Option<usize>,
}

#[function_component(Todos)]
pub fn todos(Props { todos, maybe_index }: &Props) -> Html {
    let todos: Html = todos
        .iter()
        .enumerate()
        .map(|(i, todo)| {
            let classes = classes!(
                "todo",
                todo.completed.then_some("completed"),
                maybe_index.and_then(|index| (index == i).then_some("selected"))
            );
            html! {
                <span class={classes} key={i}>{todo.name.as_str()}</span>
            }
        })
        .collect();

    html! {
        <div class="todos">
            {todos}
        </div>
    }
}
