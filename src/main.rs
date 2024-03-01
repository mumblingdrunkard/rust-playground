#![allow(warnings)]

mod controller;
mod event;
mod model;
mod observer;
mod view;

use std::cell::RefCell;
use std::rc::Rc;

use controller::Controller;
use model::Model;
use view::View;

fn main() {
    let mut model = Model::new(42);

    let mut view = View::new();

    let mut controller = Controller::new(&mut model, &mut view);

    controller.run();
}
