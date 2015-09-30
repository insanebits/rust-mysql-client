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
use std::cell::RefCell;

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

/**
 * Configures result table and adds to parent container
 * Returns reference to the model for dynamic modification
 *
 */
fn setup_result_table<'a>(container: &gtk::Box, table: &gtk::TreeView, store: &gtk::ListStore ) -> () {
    let table_model = store.get_model().unwrap();

    table.set_model(&table_model);
    table.set_headers_visible(false);
    
    container.add(table);
}

fn display_query_results<'a>(response: QueryResult, table_store: &'a gtk::ListStore, columns_cell: &'a RefCell<Vec<gtk::TreeViewColumn>>, table: &'a gtk::TreeView) -> () {

    let mut columns = columns_cell.borrow_mut();
    
    // empty data
    table_store.clear();

    // remove columns from the view
    for column in columns.iter() {
        table.remove_column(column);
    }

    // empty vector
    columns.clear();

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
        
        columns.push(column);
       
        // column is no longer valid since we moved it into a vector
        let column = columns.last().unwrap();

        table.append_column(&column);

	    col_no = col_no + 1;
    }

    col_no = 0i32;

    
    for row in response
    {
	    for col in row.unwrap() {
		    let mut iter = gtk::TreeIter::new();
		    table_store.append(&mut iter);
		    table_store.set_string(&iter, col_no, &col.into_str());
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
    
    let left_tree = gtk::TreeView::new().unwrap();
    let column_types = [glib::Type::String];
    let left_store = gtk::TreeStore::new(&column_types).unwrap();
    let left_model = left_store.get_model().unwrap();

    left_tree.set_model(&left_model);
    left_tree.set_headers_visible(true);
    append_text_column(&left_tree);

    // print out when a row is selected
    
    let mysql_server: DbServer = get_server_metadata();
    
    let mut database_index = Vec::new();
    for database in &mysql_server.databases {
    	database_index.push(database.name.clone());
    }
    
    let mut database_object_map:Vec<Vec<String>> = Vec::new();;
    
    for database in &mysql_server.databases {
    	database_object_map.push(database.tables.clone());
    } 

    let left_selection = left_tree.get_selection().unwrap();
    let left_model1 = left_model.clone();
    left_selection.connect_changed(move |tree_selection| {
        let mut iter = gtk::TreeIter::new();
        tree_selection.get_selected(&left_model1, &mut iter);
        if let Some(path) = left_model1.get_path(&iter) {
            
            let selection = path.to_string().unwrap();
            let selection_parts = selection.split(":").collect::<Vec<&str>>();
            
            if selection_parts.len() == 3 {
            	let database_key = selection_parts.first().unwrap().parse::<usize>().unwrap();
            	let table_key = selection_parts.last().unwrap().parse::<usize>().unwrap();
            	
            	println!("Database {}", database_index[database_key]);
            	println!("Table {}", &database_object_map[database_key][table_key]);
            }
        }
    });

    for database in &mysql_server.databases {
    
        let mut iter = gtk::TreeIter::new();
        left_store.append(&mut iter, None);
        left_store.set_string(&iter, 0, &database.name);
        
        let mut child_iter = gtk::TreeIter::new();
        
        let parent = Some(&iter);
        
     	left_store.append(&mut child_iter, parent);
    	left_store.set_string(&child_iter, 0, "Tables");
        
        for table in &database.tables {
        	let mut table_iter = gtk::TreeIter::new();
        	left_store.append(&mut table_iter, Some(&child_iter));
        	left_store.set_string(&table_iter, 0, &table);
        }

        // select this row as a test

        if let Some(path) = left_model.get_path(&iter) {
            left_selection.select_path(&path);
        }
    }
    
    // text view
    
    let editor = gtk::TextView::new().unwrap();
    
    let editor_scrolled_window = gtk::ScrolledWindow::new(
    	None, None
	).unwrap();
	
    editor_scrolled_window.set_hexpand(true);
    editor_scrolled_window.set_vexpand(true);
    editor_scrolled_window.add(&editor);
    
    // header
    let header = gtk::HeaderBar::new().unwrap();
    
    header.set_title("Header goes here");
    
    let header_execute_button = gtk::Button::new().unwrap();
    let header_execute_image = gtk::Image::new_from_icon_name("system-search", 32).unwrap();
    header_execute_button.add(&header_execute_image);
    
    header.pack_end(&header_execute_button);
    
    // footer
    
    let footer = gtk::Box::new(gtk::Orientation::Vertical, 10).unwrap();
    
    let table = gtk::TreeView::new().unwrap();
    let table_store = gtk::ListStore::new(&[glib::Type::String]).unwrap();
    
    // list of current columns, since there is no get_columns method
    // AS of gtk 0.0.4
    let table_cols:Vec<gtk::TreeViewColumn> = Vec::new();
    let table_cols_cell = RefCell::new(table_cols);
    
    setup_result_table(&footer, &table, &table_store);
    
    let scrollable_footer = gtk::ScrolledWindow::new(None, None).unwrap();
    scrollable_footer.set_hexpand(true);
    scrollable_footer.set_vexpand(true);
    scrollable_footer.add(&footer);
    
    header_execute_button.connect_clicked(move |_| {
    	let text_start = editor.get_buffer().unwrap().get_start_iter().unwrap();
    	let text_end = editor.get_buffer().unwrap().get_start_iter().unwrap();
    	
    	// move iterator to end to capture all text buffer
    	text_end.forward_to_end();
    	
    	let query = text_start.get_text(&text_end).unwrap();
        let response = mysql_server.pool.prep_exec(query, ());
    	
        match response {
	        Ok(result) => {
		        display_query_results(result, &table_store, &table_cols_cell, &table);
	        }
	        Err(e) => {
		        println!("Error: {}", e);
	        }
        }
    });
    
    let main_box = gtk::Box::new(gtk::Orientation::Vertical, 0).unwrap();
    let panels = gtk::Grid::new().unwrap();
    
    let split_pane = gtk::Box::new(gtk::Orientation::Horizontal, 10).unwrap();
    split_pane.set_size_request(-1, -1);
    split_pane.add(&left_tree);
    
    let split_pane_scrolled_window = gtk::ScrolledWindow::new(None, None).unwrap();
	
    split_pane_scrolled_window.set_hexpand(true);
    split_pane_scrolled_window.set_vexpand(true);
    split_pane_scrolled_window.add(&split_pane);
    
    panels.add(&split_pane_scrolled_window);
    panels.add(&editor_scrolled_window);
    
    main_box.add(&header);
    main_box.add(&panels);
    main_box.add(&scrollable_footer);
    
    window.add(&main_box);

    window.show_all();
    gtk::main();
}
