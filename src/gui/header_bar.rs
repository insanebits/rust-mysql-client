///
/// Header Widget
///

use gtk;
use gtk::traits::*;


pub struct HeaderBar
{
    pub widget: gtk::Box
}

impl HeaderBar {
    pub fn new() -> HeaderBar {
        
        let header = gtk::Box::new(gtk::Orientation::Vertical, 0).unwrap();
        
        // header
        let header_bar = gtk::HeaderBar::new().unwrap();
        
        header_bar.set_title("MySQL Browser");
        
        let header_execute_button = gtk::Button::new().unwrap();
        let header_execute_image = gtk::Image::new_from_icon_name("system-search", 32).unwrap();
        header_execute_button.add(&header_execute_image);
        // TODO header button needs some access to editor
        // or maybe editor sets up it's own list of buttons
        // then it needs access to the header
        /*
        header_execute_button.connect_clicked(move |_| {
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
    	
    	// bar which holds list of items
        let menu_bar = gtk::MenuBar::new().unwrap();
        
        menu_bar.add(&HeaderBar::create_file_menu());
     	menu_bar.add(&HeaderBar::create_edit_menu());
      	menu_bar.add(&HeaderBar::create_help_menu());

        header.add(&menu_bar);
        header.add(&header_bar);
        
        HeaderBar {
            widget: header
        }
    }
    
    fn create_file_menu() -> gtk::MenuItem {
    	// menu which holds list of actions
        let menu = gtk::Menu::new().unwrap();
        
        // populate menu with items
        menu.append(&gtk::MenuItem::new_with_label("TODO 1").unwrap());
    	menu.append(&gtk::MenuItem::new_with_label("TODO 2").unwrap());
    	menu.append(&gtk::MenuItem::new_with_label("TODO 3").unwrap());
    	
    	// top level item
    	let parent_item = gtk::MenuItem::new_with_label("File").unwrap();
    	parent_item.set_submenu(&menu);
    	
    	parent_item
    }
    
    fn create_edit_menu() -> gtk::MenuItem {
    	// menu which holds list of actions
        let menu = gtk::Menu::new().unwrap();
        
        // populate menu with items
        menu.append(&gtk::MenuItem::new_with_label("TODO 1").unwrap());
    	menu.append(&gtk::MenuItem::new_with_label("TODO 2").unwrap());
    	menu.append(&gtk::MenuItem::new_with_label("TODO 3").unwrap());
    	
    	// top level item
    	let parent_item = gtk::MenuItem::new_with_label("Edit").unwrap();
    	parent_item.set_submenu(&menu);
    	
    	parent_item
    }
    
    fn create_help_menu() -> gtk::MenuItem {
    	// menu which holds list of actions
        let menu = gtk::Menu::new().unwrap();
        
        // populate menu with items
        menu.append(&gtk::MenuItem::new_with_label("TODO 1").unwrap());
    	menu.append(&gtk::MenuItem::new_with_label("TODO 2").unwrap());
    	menu.append(&gtk::MenuItem::new_with_label("TODO 3").unwrap());
    	
    	// top level item
    	let parent_item = gtk::MenuItem::new_with_label("Help").unwrap();
    	parent_item.set_submenu(&menu);
    	
    	parent_item
    }
}
