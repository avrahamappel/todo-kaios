use std::ops::Deref;

use gloo_utils::{body, document};
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement, NodeList};
use yew::prelude::*;

#[derive(Default)]
enum NavigationType {
    #[default]
    None,
}

impl TryFrom<String> for NavigationType {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s {
            _ => Err(format!("Can't make a navigation type out of {}", s)),
        }
    }
}

#[derive(Default)]
struct NavigationState {
    ntype: NavigationType,
    index: u32,
}

pub fn use_navigation() -> UseStateHandle<NavigationState> {
    let current = use_state(|| NavigationState::default());

    fn get_all_elements() -> NodeList {
        document()
            .query_selector_all("[nav-selectable]")
            .expect("No elements")
    }

    let select_element = |maybe_element: Option<&Element>, set_index: u32| {
        if let Some(element) = maybe_element {
            let elements = get_all_elements();
            for index in 0..elements.length() {
                let el = elements
                    .get(index)
                    .expect(&format!("Couldn't find element {}", index))
                    .dyn_ref::<HtmlElement>()
                    .expect("Wasn't an Element");

                let select_this_element = el;
                el.set_attribute("nav-selected", (&**el == element).to_string().as_str());
                el.set_attribute("nav-index", index.to_string().as_str());
                // Was originally as below, unsure of intention
                // if select_this_element {
                if true {
                    select_this_element.scroll_into_view();
                    if el.node_name() == "INPUT" {
                        el.focus();
                    } else {
                        el.blur();
                    }
                }
            }
            current.set(NavigationState {
                ntype: element.tag_name().try_into().unwrap(),
                index: set_index,
            });
        } else {
            set_navigation(0);
        }
    };

    let set_navigation = |index: u32| {
        select_element(get_all_elements().get(index).unwrap_or(**body()));
    };

    let get_the_index_of_the_selected_element = || {
        if let Ok(Some(element)) = document().query_selector("[nav-selected=true]") {
            element
                .get_attribute("nav-index")
                .map(|a| a.parse::<u32>().unwrap_or(0))
                .unwrap_or(0)
        } else {
            0
        }
    };

    let on_keydown = |evt| {
        if evt.key != "ArrowDown" && evt.key != "ArrowUp" {
            return;
        }

        let all_elements = get_all_elements();
        let current_index = get_the_index_of_the_selected_element();

        let set_index = match evt.key {
            "ArrowDown" => {
                let go_to_first_element = current_index + 1 > all_elements.length() - 1;
                Some(if go_to_first_element {
                    0
                } else {
                    current_index + 1
                })
            }
            "ArrowUp" => {
                let go_to_last_element = current_index == 0;
                Some(if go_to_last_element {
                    all_elements.length() - 1
                } else {
                    current_index - 1
                })
            }
            _ => None,
        };

        if let Some(index) = set_index {
            return select_element(all_elements.get(index).or(all_elements.get(0)), index);
        }
    };

    use_effect(|| {
        document().add_event_listener_with_callback("keydown", on_keydown);

        set_navigation(0);

        || document().remove_event_listener_with_callback("keydown", on_keydown)
    });

    current
}
