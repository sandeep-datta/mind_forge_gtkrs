// Copyright (C) 2017+ Sandeep Datta

extern crate gtk;

#[macro_use]
mod utils;

use gtk::{BoxExt};
use gtk::prelude::*;
use gtk::Orientation::{Horizontal, Vertical};

const X_SPACING: i32 = 10;
const Y_SPACING: i32 = 10;

struct Node {
    text: String,
    children: Vec<Node>,
}

impl Node {
    fn new(text: String, children: Vec<Node>) -> Node {
        Node {text: text, children: children}
    }
    
    fn to_mind_map(&self) -> gtk::Box {
        let hbox = gtk::Box::new(Horizontal, X_SPACING);
        hbox.add(&gtk::Label::new(Some(self.text.as_ref())));
        
        let vbox = gtk::Box::new(Vertical, Y_SPACING);
        
        for child in &self.children {
            vbox.add(&child.to_mind_map());
        }
        
        hbox.add(&vbox);
        
        hbox
    }

}

fn get_sample_tree() -> Node {
    let sample = Node::new("Root node".to_string(), vec![
        Node::new("Child node 1".to_string(), vec![
            Node::new("Child node 1.1".to_string(), vec![])
        ]),
        Node::new("Child node 2".to_string(), vec![]),
    ]);
    
    sample
}


fn main() {
    if gtk::init().is_err() {
        println_err!("ERROR: Failed to initialize GTK.");
        return;
    }
    
    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    
    window.set_title("Mindforge");
    
    window.connect_delete_event(|_, _| {
       gtk::main_quit();
       Inhibit(false)
    });
    
    let node = get_sample_tree();
    
    window.add(&node.to_mind_map());
    
    window.show_all();
    gtk::main();
}
