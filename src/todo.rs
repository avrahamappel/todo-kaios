#[derive(Clone, PartialEq)]
pub struct Todo {
    pub name: String,
    pub completed: bool,
}

impl Todo {
    pub fn new(name: String) -> Todo {
        Self {
            name,
            completed: false,
        }
    }
}

pub fn sample_todos() -> [Todo; 3] {
    [
        Todo {
            name: "Order Pizza".into(),
            completed: false,
        },
        Todo {
            name: "Pay Bill".into(),
            completed: true,
        },
        Todo {
            name: "Write Code".into(),
            completed: false,
        },
    ]
}
