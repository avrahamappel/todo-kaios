use gloo_utils::document;
use yew::prelude::*;

use crate::hooks::use_navigation;

struct Todo {
    name: String,
    completed: bool,
}

#[function_component(App)]
pub fn app() -> Html {
    let todos = use_state_eq(|| vec![]);
    let current = use_navigation();

    let onKeyCenter = || {
        let currentElement = document()
            .query_selector("[nav-selected=true]")
            .expect("Nothing currently selected")
            .unwrap();

        let currentNavigationIndex = currentElement
            .get_attribute("nav-index")
            .expect("No attribute `nav-index`")
            .parse::<u32>()
            .expect("`nav-index` couldn't be parsed to a number");

        let isATask = currentNavigationIndex > 0;
        if isATask {
            todos.set(|prevState| {
                let current = vec![..prevState];
                current[currentNavigationIndex - 1].completed =
                    !current[currentNavigationIndex - 1].completed;
                return current;
            });
        } else if currentElement.value.length {
            let toDo = Todo {
                name: currentElement.value,
                completed: false,
            };
            todos.set(|prevState| vec![..prevState, toDo]);
            currentElement.value = "";
        }
    };

    let onKeyRight = || {
        let currentIndex = document
            .querySelector("[nav-selected=true]")
            .getAttribute("nav-index")
            .expect("No element `nav-index`")
            .parse::<u32>();
        if currentIndex > 0 {
            todos.set(|prevState| {
                let current = vec![..prevState];
                current.splice(currentIndex - 1, 1);
                let goToPreviousElement = current.length as bool;
                current.set(if goToPreviousElement {
                    currentIndex - 1
                } else {
                    0
                });
                return current;
            });
        }
    };

    html! {
         <>
      <Header title="ToDo List" />

      <Input type="text" label="New task" />
      <ToDos toDos={toDos} />

      <Softkey
      center={if         *current.type == "INPUT" { "Insert" } else { "Toggle"} }
        onKeyCenter={onKeyCenter}
        right={if         current.type === "SPAN" { "Delete" } else { ""} }
        onKeyRight={onKeyRight}
      />
    </>
    }
}
