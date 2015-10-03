//! # Basic Sample
//!
//! This sample demonstrates how to create a toplevel `window`, set its title, size and position, how to add a `button` to this `window` and how to connect signals with actions.

#![crate_type = "bin"]

// Third party libraries used, every possible dependency should be defined in here
extern crate glib;
extern crate gtk;
extern crate mysql;
extern crate config;

// Top level modules
mod gui;
mod connector;

fn main() {
	// initialize gtk library
    gtk::init().unwrap_or_else(|_| panic!("Failed to initialize GTK."));
	// create window
	let window = gui::MainWindow::new();
    // load components
    window.setup();
	// show window
    window.show();
    // run gtk event loop, essentially infinite loop
    gtk::main();
}
