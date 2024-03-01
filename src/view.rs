use read_input::prelude::input;
use read_input::InputBuild;
use std::cell::RefCell;
use std::rc::Rc;

use crate::event::Event;
use crate::model::Model;

pub struct View<'a> {
    pub input_submitted: Event<'a, i32>,
}

impl View<'_> {
    pub fn new() -> Self {
        return Self {
            input_submitted: Event::<i32>::new(),
        };
    }

    pub fn render(&self, model: &Model) {
        println!();
        println!("Render... (x = {})", &model.x);
        let input: i32 = input().repeat_msg("Input = ").get();
        self.input_submitted.invoke(input);
    }
}
