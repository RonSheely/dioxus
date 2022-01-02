#![allow(non_upper_case_globals, non_snake_case)]
use dioxus::prelude::*;
use im_rc::HashMap;
use std::rc::Rc;

fn main() {
    dioxus::desktop::launch(app);
}

#[derive(PartialEq)]
pub enum FilterState {
    All,
    Active,
    Completed,
}

#[derive(Debug, PartialEq)]
pub struct TodoItem {
    pub id: u32,
    pub checked: bool,
    pub contents: String,
}

fn app(cx: Scope) -> Element {
    let draft = use_state(&cx, || "".to_string());
    let todos = use_state(&cx, || HashMap::<u32, Rc<TodoItem>>::new());
    let filter = use_state(&cx, || FilterState::All);

    let todolist = todos
        .iter()
        .filter(|(_id, item)| match *filter {
            FilterState::All => true,
            FilterState::Active => !item.checked,
            FilterState::Completed => item.checked,
        })
        .map(|(id, todo)| {
            rsx!(TodoEntry {
                key: "{id}",
                todo: todo.clone()
            })
        })
        .collect::<Vec<_>>();

    let items_left = todolist.len();
    let item_text = match items_left {
        1 => "item",
        _ => "items",
    };

    cx.render(rsx!(
        div { id: "app",
            style { [include_str!("./assets/todomvc.css")] }
            div {
                header {
                    class: "header",
                    h1 { "todos" }
                    input {
                        class: "new-todo",
                        placeholder: "What needs to be done?",
                        value: "{draft}",
                        oninput: move |evt| draft.set(evt.value.clone()),
                    }
                }
                todolist,
                (!todos.is_empty()).then(|| rsx!(
                    footer {
                        span {
                            strong {"{items_left}"}
                            span {"{item_text} left"}
                        }
                        ul { class: "filters",
                            li { class: "All", a { href: "", onclick: move |_| filter.set(FilterState::All), "All" }}
                            li { class: "Active", a { href: "active", onclick: move |_| filter.set(FilterState::Active), "Active" }}
                            li { class: "Completed", a { href: "completed", onclick: move |_| filter.set(FilterState::Completed), "Completed" }}
                        }
                    }
                ))
            }
            footer { class: "info",
                p {"Double-click to edit a todo"}
                p { "Created by ", a {  href: "http://github.com/jkelleyrtp/", "jkelleyrtp" }}
                p { "Part of ", a { href: "http://todomvc.com", "TodoMVC" }}
            }
        }
    ))
}

#[derive(PartialEq, Props)]
pub struct TodoEntryProps {
    todo: Rc<TodoItem>,
}

pub fn TodoEntry(cx: Scope<TodoEntryProps>) -> Element {
    let is_editing = use_state(&cx, || false);
    let contents = use_state(&cx, || String::from(""));
    let todo = &cx.props.todo;

    cx.render(rsx! {
        li {
            "{todo.id}"
            input {
                class: "toggle",
                r#type: "checkbox",
                "{todo.checked}"
            }
           is_editing.then(|| rsx!{
                input {
                    value: "{contents}",
                    oninput: move |evt| contents.set(evt.value.clone())
                }
            })
        }
    })
}
