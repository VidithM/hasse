use std::vec::Vec;
use fltk::{app, prelude::*, window::Window};

#[path="graph.rs"] mod graph;

use graph::{Graph};

pub fn show(adj : &mut Graph) -> () {
    let app = app::App::default();
    let mut disp = Window::new(100, 100, 400, 300, "Hi");
    disp.end();
    disp.show();
    app.run().unwrap();
}

fn render(adj : &mut Graph) -> () {}

fn handleInput(adj : &mut Graph) -> () {}