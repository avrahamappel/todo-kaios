use gloo_utils::document;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::hooks;

#[derive(PartialEq)]
struct Todo {
    name: String,
    completed: bool,
}

#[function_component(App)]
pub fn app() -> Html {
    let todos = use_state_eq(|| -> Vec<Todo> { vec![] });
    let current = hooks::use_navigation();

    let on_key_center = || {
        let current_element = document()
            .query_selector("[nav-selected=true]")
            .expect("Nothing currently selected")
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .expect("Wasn't an input element");

        let current_navigation_index = current_element
            .get_attribute("nav-index")
            .expect("No attribute `nav-index`")
            .parse::<usize>()
            .expect("`nav-index` couldn't be parsed to a number");

        let is_a_task = current_navigation_index > 0;
        if is_a_task {
            Callback::from(|_| {
                let current = *todos.clone();
                current[current_navigation_index - 1].completed =
                    !current[current_navigation_index - 1].completed;
                todos.set(current);
            });
        } else if current_element.value().len() > 0 {
            let todo = Todo {
                name: current_element.value(),
                completed: false,
            };
            Callback::from(|_| todos.push(todo));
            current_element.set_value("");
        }
    };

    let on_key_right = || {
        let current_index = document()
            .query_selector("[nav-selected=true]")
            .expect("Nothing currently selected")
            .unwrap()
            .get_attribute("nav-index")
            .expect("No element `nav-index`")
            .parse::<usize>()
            .expect("`nav-index` couldn't be parsed to a number");

        if current_index > 0 {
            Callback::from(|_| {
                let cur = *todos.clone();
                cur.remove(current_index - 1);
                let go_to_previous_element = cur.len() != 0;
                hooks::set_navigation(if go_to_previous_element {
                    (current_index - 1) as u32
                } else {
                    0
                });
                todos.set(cur);
            });
        }
    };

    html! {
         <>
      <Header title="ToDo List" />

      <Input type="text" label="New task" />
      <ToDos {todos} />

      <Softkey
      center={if         *current.type == "INPUT" { "Insert" } else { "Toggle"} }
        {on_key_center}
        right={if         *current.type == "SPAN" { "Delete" } else { ""} }
        {on_key_right}
      />
    </>
    }
}
