// Copyright (C) 2017+ Sandeep Datta

extern crate gtk;

#[macro_use]
mod utils;

use gtk::{Align, BoxExt};
use gtk::prelude::*;
use gtk::Orientation::{Horizontal, Vertical};

const X_SPACING: i32 = 30;
const Y_SPACING: i32 = 30;

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
        
        let lbl = gtk::Label::new(Some(self.text.as_ref()));
        lbl.set_valign(Align::Center);
        
        hbox.add(&lbl);
        
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
            Node::new("Child node 1.1".to_string(), vec![]),
            Node::new("Child node 1.2".to_string(), vec![])
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
    let scrolled_window = gtk::ScrolledWindow::new(None, None);
    
    window.set_title("Mindforge");
    
    window.connect_delete_event(|_, _| {
       gtk::main_quit();
       Inhibit(false)
    });
    
    let vbox = gtk::Box::new(Vertical, Y_SPACING);
    let node = get_sample_tree();
    
    vbox.add(&node.to_mind_map());
    
    vbox.set_valign(Align::Center);
    vbox.set_halign(Align::Center);
    
    scrolled_window.add(&vbox);
    
    window.add(&scrolled_window);
    window.set_default_size(600, 400);
    // window.maximize();
    window.show_all();
    gtk::main();
}
