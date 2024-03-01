use crate::model::Model;
use crate::observer::Observer;
use crate::view::View;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Controller<'a> {
    model: &'a mut Model,
    view: &'a View<'a>,
}

impl<'a> Controller<'a> {
    pub fn new(model: &'a mut Model, view: &'a mut View<'a>) -> Self {
        let observer = ViewObserver { model };
        view.input_submitted.subscribe(Box::new(observer));
        let controller = Controller { model, view };
        return controller;
    }

    pub fn run(&mut self) {
        loop {
            self.view.render(self.model);
        }
    }
}

struct ViewObserver<'a> {
    model: &'a mut Model,
}

impl Observer<i32> for ViewObserver<'_> {
    fn invoke(&self, arg1: i32) {
        println!("Input received (x = {})", arg1);
    }
}
