use yew::prelude::*;

use crate::todo::Todo;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub todos: Vec<Todo>,
}

#[function_component(Todos)]
pub fn todos(props: &Props) -> Html {
    let todos: Html = props
        .todos
        .iter()
        .enumerate()
        .map(|(index, todo)| {
            html! {
                <span class={classes!("todo", todo.completed.then_some( Some("completed") ))}
                      nav-selectable="true"
                      key={index}>{todo.name}</span>
            }
        })
        .collect();

    html! {
        <div class="todos">
            {todos}
        </div>
    }
}
