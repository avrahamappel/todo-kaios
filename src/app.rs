use std::convert::AsRef;

use gloo_utils::document;
use wasm_bindgen::prelude::Closure;
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

fn input(nav: &Navigation, name: UseStateHandle<String>) -> Html {
    let focused = if let Navigation::Inserting = nav {
        true
    } else {
        false
    };

    let on_input = {
        let name = name.clone();
        Callback::from(move |evt: InputEvent| {
            name.set(
                evt.target()
                    .expect("InputEvent had no target")
                    .dyn_into::<HtmlInputElement>()
                    .expect("InputEvent target wasn't an HtmlInputElement")
                    .value(),
            )
        })
    };

    html! {
        <Input itype="text" label="New task" {focused} {on_input} />
    }
}

fn softkey(
    index: Option<usize>,
    state: UseReducerHandle<TodosState>,
    name: UseStateHandle<String>,
) -> Html {
    if let Some(index) = index {
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
            let name = name.clone();
            Callback::from(move |_| {
                if !name.is_empty() {
                    state.dispatch(TodosAction::Add(name.to_string()));
                    name.set("".into());
                }
            })
        };

        html! {
            <SoftKey center={"Insert"} {on_key_center} />
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
enum Navigation {
    Inserting,
    Indexing(usize),
}

impl Default for Navigation {
    fn default() -> Self {
        Self::Indexing(0)
    }
}

impl From<&Navigation> for Option<usize> {
    fn from(nav: &Navigation) -> Self {
        match nav {
            Navigation::Inserting => None,
            Navigation::Indexing(index) => Some(*index),
        }
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let state = use_reducer_eq(TodosState::load);
    let todos = (&*state).todos.to_owned();
    let nav_state = use_state_eq(|| Navigation::default());
    let nav = (&*nav_state).to_owned();
    let maybe_index = (&nav).into();
    let todo_name = use_state_eq(|| String::new());

    let input = input(&nav, todo_name.clone());

    let softkey = softkey(maybe_index, state.clone(), todo_name.clone());

    let navigate_callback = {
        let nav = nav.clone();
        let todos = todos.clone();
        Closure::<dyn Fn(KeyboardEvent)>::wrap(Box::new(move |evt: KeyboardEvent| {
            let new_nav = match nav {
                Navigation::Inserting => {
                    if evt.key().as_str() == "ArrowDown" {
                        Navigation::Indexing(0)
                    } else {
                        nav.clone()
                    }
                }
                Navigation::Indexing(index) => match evt.key().as_str() {
                    "ArrowUp" => match index {
                        0 => Navigation::Inserting,
                        _ => Navigation::Indexing(index - 1),
                    },
                    "ArrowDown" if index < todos.len() - 1 => Navigation::Indexing(index + 1),
                    _ => nav.clone(),
                },
            };

            nav_state.set(new_nav);
        }))
    };

    use_effect(move || {
        document()
            .add_event_listener_with_callback("keydown", navigate_callback.as_ref().unchecked_ref())
            .expect("Couldn't add event listener");

        move || {
            document()
                .remove_event_listener_with_callback(
                    "keydown",
                    navigate_callback.as_ref().unchecked_ref(),
                )
                .expect("Couldn't remove event listener");
        }
    });

    html! {
        <>
            <Header title="ToDo List" />

            {input}

            <Todos todos={todos.clone()} {maybe_index} />

            {softkey}
        </>
    }
}
