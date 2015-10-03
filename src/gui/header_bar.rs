///
/// Header Widget
///

use gtk;
use gtk::traits::*;


pub struct HeaderBar
{
    pub widget: gtk::HeaderBar
}

impl HeaderBar {
    pub fn new() -> HeaderBar {
        
        // header
        let header = gtk::HeaderBar::new().unwrap();
        
        header.set_title("MySQL Browser");
        
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
        
        header.pack_end(&header_execute_button);
        
        
        HeaderBar {
            widget: header
        }
    }
}
