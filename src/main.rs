#![feature(proc_macro)]
extern crate gtk;
#[macro_use]
extern crate relm;
extern crate relm_attributes;
#[macro_use]
extern crate relm_derive;
extern crate meval;

use self::Msg::*;
use gtk::Orientation::{Horizontal, Vertical};
use gtk::{
    ButtonExt, EditableExt, EditableSignals, EntryExt, GtkWindowExt, Inhibit,
    OrientableExt, WidgetExt,
};
use relm::Widget;
use relm_attributes::widget;

#[derive(Clone)]
pub struct Model {
    expression: String,
}

#[derive(Msg)]
pub enum Msg {
    Number(i32),
    Operator(String),
    Change(String),
    Calculate,
    Erase,
    Clear,
    Quit,
}

#[widget]
impl Widget for Win {
    fn model() -> Model {
        Model {
            expression: String::new(),
        }
    }

    fn update(&mut self, event: Msg) {
        if self.model.expression == "Error" {
            self.model.expression.clear();
        }
        match event {
            Number(n) => self.model.expression = format!("{}{}", self.model.expression, n),
            Operator(op) => self.model.expression = format!("{}{}", self.model.expression, op),
            Change(change) => self.model.expression = change,
            Calculate => {
                let evaluated = meval::eval_str(&self.model.expression);
                match evaluated {
                    Ok(number) => self.model.expression = number.to_string(),
                    Err(e) => {
                        println!("{}", e.to_string());
                        self.model.expression = String::from("Error")
                    }
                }
            }
            Erase => {
                self.model.expression.pop();
                self.model.expression = format!("{}", self.model.expression);
                ()
            }
            Clear => self.model.expression = String::new(),
            Quit => gtk::main_quit(),
            _ => println!("To be implemented"),
        }
    }

    view! {
        gtk::Window {
            title: "Max's shit calculator",
            //set smallest possible size with gui elements
            property_default_height: 0,
            property_default_width: 0,
            resizable: false,

            gtk::Box {
                orientation: Vertical,

                gtk::Box {
                    gtk::Entry {
                        activate => Calculate,
                        changed(entry) => Change(entry.get_text().unwrap()),
                        text: &self.model.expression.to_string(),
                        position: self.model.expression.len() as i32,
                        width_chars: 20,
                    },
                },


                gtk::Box {

                    orientation: Vertical,

                    gtk::Box {
                        orientation: Horizontal,

                        gtk::Button {
                            clicked => Number(1),
                            label: "1",
                        },
                        gtk::Button {
                            clicked => Number(2),
                            label: "2",
                        },
                        gtk::Button {
                            clicked => Number(3),
                            label: "3",
                        },
                        gtk::Button {
                            clicked => Clear,
                            label: "C",
                        }

                    },

                    gtk::Box {
                        orientation: Horizontal,

                        gtk::Button {
                            clicked => Number(4),
                            label: "4",
                        },
                        gtk::Button {
                            clicked => Number(5),
                            label: "5",
                        },
                        gtk::Button {
                            clicked => Number(6),
                            label: "6",
                        },
                        gtk::Button {
                            clicked => Erase,
                            label: "⌫",
                        },

                    },

                    gtk::Box {
                        orientation: Horizontal,

                        gtk::Button {
                            clicked => Number(7),
                            label: "7",
                        },
                        gtk::Button {
                            clicked => Number(8),
                            label: "8",
                        },
                        gtk::Button {
                            clicked => Number(9),
                            label: "9",
                        },
                        gtk::Button {
                            clicked => Operator("+".to_string()),
                            label: "+",
                        },
                    },

                    gtk::Box {
                        orientation: Horizontal,

                        gtk::Button {
                            clicked => Number(0),
                            label: "0",
                        },
                        gtk::Button {
                            clicked => Operator(".".to_string()),
                            label: ".",
                        },
                        gtk::Button {
                            clicked => Operator("%".to_string()),
                            label: "%",
                        },
                        gtk::Button {
                            clicked => Operator("-".to_string()),
                            label: "-",
                        },
                    },

                    gtk::Box {
                        orientation: Horizontal,

                        gtk::Button {
                            clicked => Operator("*".to_string()),
                            label: "*",
                        },
                        gtk::Button {
                            clicked => Operator("/".to_string()),
                            label: "/",
                        },
                        gtk::Button {
                            clicked => Operator("^".to_string()),
                            label: "^",
                        },
                        gtk::Button {
                            clicked => Calculate,
                            label: "=",
                        },
                    },
                },
            },
            delete_event(_, _) => (Quit, Inhibit(false)),
        }
    }
}

fn main() {
    Win::run(()).unwrap();
}
