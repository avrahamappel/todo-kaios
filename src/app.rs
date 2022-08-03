use gloo_utils::document;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::components::{Header, Input, SoftKey, Todos};
use crate::todo::{self, Todo};

enum TodosAction {
    Toggle(usize),
    Add(String),
    Remove(usize),
}

#[derive(PartialEq)]
struct TodosState {
    todos: Vec<Todo>,
}

impl TodosState {
    fn load() -> Self {
        let todos = todo::sample_todos().into();
        Self { todos }
    }
}

impl Reducible for TodosState {
    type Action = TodosAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        let mut todos = self.todos.clone();
        todos = match action {
            TodosAction::Toggle(index) => {
                if let Some(todo) = todos.get_mut(index) {
                    todo.completed = !todo.completed
                }
                todos
            }
            TodosAction::Add(name) => {
                todos.push(Todo::new(name));
                todos
            }
            TodosAction::Remove(index) => {
                todos.remove(index);
                todos
            }
        };

        Self { todos }.into()
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let state = use_reducer_eq(TodosState::load);
    let todos = &state.todos;
    let index_state = use_state_eq(|| 0);
    let index = *index_state;

    let softkey = if todos.get(index).is_some() {
        let on_key_center = {
            let state = state.clone();
            Callback::from(move |_| state.dispatch(TodosAction::Toggle(index)))
        };

        let on_key_right = {
            let state = state.clone();
            Callback::from(move |_| state.dispatch(TodosAction::Remove(index)))
        };

        html! {
            <SoftKey center={"Toggle"} {on_key_center} right={"Delete"} {on_key_right} />
        }
    } else {
        let on_key_center = {
            let state = state.clone();
            Callback::from(move |_| {
                let current_element = document()
                    .query_selector("[nav-selected=true]")
                    .expect("Nothing currently selected")
                    .unwrap()
                    .dyn_into::<HtmlInputElement>()
                    .expect("Wasn't an input element");

                if !current_element.value().is_empty() {
                    state.dispatch(TodosAction::Add(current_element.value()));
                    current_element.set_value("");
                }
            })
        };

        html! {
            <SoftKey center={ "Insert" } {on_key_center} />
        }
    };

    html! {
        <>
            <Header title="ToDo List" />

            <Input itype="text" label="New task" />
            <Todos todos={todos.clone()} current_index={index} />

            {softkey}
        </>
    }
}
