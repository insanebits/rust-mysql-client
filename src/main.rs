//! # Basic Sample
//!
//! This sample demonstrates how to create a toplevel `window`, set its title, size and position, how to add a `button` to this `window` and how to connect signals with actions.

#![crate_type = "bin"]

extern crate glib;
extern crate gtk;

// client
extern crate mysql;

extern crate mysql_connector;

use std::default::Default;
use mysql::conn::MyOpts;
use mysql::conn::pool::MyPool;
use mysql::value::from_row;
use mysql::error::MyResult;
use mysql::conn::QueryResult;

// import client stuff
use mysql_connector::DbServer;
use mysql_connector::Database;

use gtk::traits::*;
use gtk::signal::Inhibit;

fn append_text_column(tree: &gtk::TreeView) {
    let column = gtk::TreeViewColumn::new().unwrap();
    let cell = gtk::CellRendererText::new().unwrap();

    column.pack_start(&cell, true);
    column.add_attribute(&cell, "text", 0);
    tree.append_column(&column);
}

fn get_server_metadata() -> DbServer {
  let port = 3306;
  println!("PORT IS {}", port); 
 
  let opts = MyOpts {
    user: Some("root".to_string()),
    pass: Some("".to_string()),
    ..Default::default()
  };
   
  let pool = MyPool::new(opts).unwrap();    
  let mut server =  DbServer {
    name: "localhost".to_string(),
    databases: Vec::new(),
    pool: pool
  };

  server.load_metadata();
  
  server
}

fn main() {
    gtk::init().unwrap_or_else(|_| panic!("Failed to initialize GTK."));

    let window = gtk::Window::new(gtk::WindowType::Toplevel).unwrap();

    window.set_title("First GTK+ Program");
    window.set_border_width(10);
    window.set_window_position(gtk::WindowPosition::Center);
    window.set_default_size(350, 70);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(true)
    });
    
    let left_tree = gtk::TreeView::new().unwrap();
    let column_types = [glib::Type::String];
    let left_store = gtk::ListStore::new(&column_types).unwrap();
    let left_model = left_store.get_model().unwrap();

    left_tree.set_model(&left_model);
    left_tree.set_headers_visible(true);
    append_text_column(&left_tree);

    // print out when a row is selected
    
    let mysql_server: DbServer = get_server_metadata();

    let left_selection = left_tree.get_selection().unwrap();
    let left_model1 = left_model.clone();
    left_selection.connect_changed(move |tree_selection| {
        let mut iter = gtk::TreeIter::new();
        tree_selection.get_selected(&left_model1, &mut iter);
        if let Some(path) = left_model1.get_path(&iter) {
            println!("selected row {}", path.to_string().unwrap());
            let index = path.to_string().unwrap().parse::<usize>().unwrap();
            
            //let selected_database: Database = mysql_server.databases[index];
            
            println!("selected text {}", mysql_server.databases[index].name); 
        }
    });
    


    for database in mysql_server.databases {
    
        let mut iter = gtk::TreeIter::new();
        left_store.append(&mut iter);
        left_store.set_string(&iter, 0, &database.name);

        // select this row as a test

        if let Some(path) = left_model.get_path(&iter) {
            left_selection.select_path(&path);
        }
    }
    
    
    let split_pane = gtk::Box::new(gtk::Orientation::Horizontal, 10).unwrap();
    split_pane.set_size_request(-1, -1);
    split_pane.add(&left_tree);
    
    window.add(&split_pane);

    window.show_all();
    gtk::main();
}
