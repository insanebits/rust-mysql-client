///
/// Database browser sidebar 
///

use gtk;
use gtk::traits::*;
use glib;
use connector::Server;



pub struct DatabaseBrowser{
    pub widget: gtk::ScrolledWindow
}

impl DatabaseBrowser {
    pub fn new(mysql_server: &Server) -> DatabaseBrowser
    {
        let left_tree = gtk::TreeView::new().unwrap();
        left_tree.set_hexpand(true);
        
        
        let column_types = [glib::Type::String, glib::Type::String];
        let left_store = gtk::TreeStore::new(&column_types).unwrap();
        let left_model = left_store.get_model().unwrap();


        left_tree.set_model(&left_model);
        left_tree.set_headers_visible(true);

        // add column
        let column = gtk::TreeViewColumn::new().unwrap();
        let cell = gtk::CellRendererText::new().unwrap();
        column.pack_start(&cell, true);
        column.add_attribute(&cell, "text", 0);
        left_tree.append_column(&column);
    
        // print out when a row is selected
        
        let mut database_index = Vec::new();
        for database in &mysql_server.databases {
        	database_index.push(database.name.clone());
        }
        
        let mut database_object_map:Vec<Vec<String>> = Vec::new();
        
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
        
        let split_pane = gtk::Box::new(gtk::Orientation::Horizontal, 10).unwrap();
        
        split_pane.set_hexpand(true);
        split_pane.add(&left_tree);
        
        let split_pane_scrolled_window = gtk::ScrolledWindow::new(None, None).unwrap();
        // at least 150px width
        split_pane_scrolled_window.set_size_request(150, -1);
	
        split_pane_scrolled_window.set_hexpand(true);
        split_pane_scrolled_window.set_vexpand(true);
        split_pane_scrolled_window.add(&split_pane);
        
        DatabaseBrowser {
            widget: split_pane_scrolled_window
        }
    }
}
