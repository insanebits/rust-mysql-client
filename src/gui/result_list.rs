///
/// Dynamic database result list
///

use gtk;
use glib;
use std;
use gtk::traits::*;

use mysql::conn::QueryResult;

pub struct ResultList
{
	pub widget: gtk::ScrolledWindow
}

impl ResultList {
	pub fn new(response: QueryResult) -> ResultList {
		// setup new table
		let table = gtk::TreeView::new().unwrap();    
		
		// calc what type of columns there will be
		let mut column_types = Vec::new();
		for result_col in response.columns_ref() {
			column_types.push(glib::Type::String);
		}
		
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
		
		// create container to hold it 
		let container = gtk::ScrolledWindow::new(None, None).unwrap();
		container.add(&table);
		
		ResultList {
			widget: container,
		}
	}
}
