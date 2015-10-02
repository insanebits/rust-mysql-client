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
use mysql::conn::QueryResult;
// import client stuff
use mysql_connector::DbServer;

use std::io::prelude::*;

use gtk::traits::*;
use gtk::signal::Inhibit;

use gui::traits::Panel;

// where all the widgets will be kept
mod gui;

fn get_server_metadata () -> DbServer {
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

/**
 * Creates new ListView and replaces existing
 * Make sure container only has single table or it will get removed
 **/
fn display_query_results<'a>(response: QueryResult, table_container: &'a gtk::Box) -> () {

	// setup new table
	let table = gtk::TreeView::new().unwrap();    
    
    // calc what type of columns there will be
    let mut column_types = Vec::new();
    for result_col in response.columns_ref() {
    	column_types.push(glib::Type::String);
    }
    
    println!("Creating store");
    let table_store = gtk::ListStore::new(&column_types).unwrap();
    let table_model = table_store.get_model().unwrap();

    table.set_model(&table_model);
    table.set_headers_visible(false);
    
    let mut col_no = 0i32;
    
    for result_col in response.columns_ref()
    {
	    let virtual_col = std::str::from_utf8(&result_col.name).unwrap();
	    
	    println!("Col: {}", virtual_col);
	    println!("Col no: {}", col_no);

        let column = gtk::TreeViewColumn::new().unwrap();
        let cell = gtk::CellRendererText::new().unwrap();
        column.pack_start(&cell, true);
        column.add_attribute(&cell, "text", col_no);
        column.set_title(&virtual_col);

        table.append_column(&column);
        
	    col_no = col_no + 1;
    }
    
    println!("No of columns: {}", table_store.get_model().unwrap().get_n_columns());
    
    table_container.add(&table);
    
    for row in response
    {
       	col_no = 0i32;
       	
       	// create new row
        let mut iter = gtk::TreeIter::new();
	    table_store.append(&mut iter);
		    
		// fill columns
	    for col in row.unwrap() {
		    table_store.set_string(&iter, col_no, &col.into_str());
		    println!("Seting col: {} with value: {}", col_no, col.into_str()); 
		    col_no = col_no + 1;
	    }
    }    
}

fn main() {
    gtk::init().unwrap_or_else(|_| panic!("Failed to initialize GTK."));

    let window = gtk::Window::new(gtk::WindowType::Toplevel).unwrap();

    window.set_title("First GTK+ Program");
    window.set_border_width(10);
    window.set_window_position(gtk::WindowPosition::Center);
    window.set_default_size(640, 480);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(true)
    });
    
    
    let mysql_server: DbServer = get_server_metadata();
    
    
    // footer
    let footer = gtk::Box::new(gtk::Orientation::Vertical, 10).unwrap();
    
    let scrollable_footer = gtk::ScrolledWindow::new(None, None).unwrap();
    scrollable_footer.set_hexpand(true);
    scrollable_footer.set_vexpand(true);
    scrollable_footer.add(&footer);
    
    let mut mysql_conn = mysql_server.pool.get_conn().unwrap();
    
    /*
    header_execute_button.connect_clicked(move |&mut self| {
       
    	let text_start = editor.get_buffer().unwrap().get_start_iter().unwrap();
    	let text_end = editor.get_buffer().unwrap().get_start_iter().unwrap();
    	// move iterator to end to capture all text buffer
    	text_end.forward_to_end();
    	
    	let query = text_start.get_text(&text_end).unwrap();
    	
        let response = mysql_conn.prep_exec(query, ());
    	
        match response {
	        Ok(result) => {
		        display_query_results(result, &footer);
	        }
	        Err(e) => {
		        println!("Error: {}", e);
	        }
        }
    });
    */
    
    let main_box = gtk::Box::new(gtk::Orientation::Vertical, 0).unwrap();
    let panels = gtk::Box::new(gtk::Orientation::Horizontal, 0).unwrap();
    
    //TODO maybe create container wrapper to get rid of .widget calls every time
    panels.add(&gui::DatabaseBrowser::new(&mysql_server).widget);
    panels.add(&gui::Editor::new().widget);
    
    
    main_box.add(&gui::HeaderBar::new().widget);
    main_box.add(&panels);
    main_box.add(&scrollable_footer);
    
    window.add(&main_box);

    window.show_all();
    gtk::main();
}
