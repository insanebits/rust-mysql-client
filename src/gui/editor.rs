///
/// SQL Editor panel
///

use gtk::{
    ScrolledWindow,
    TextView,
};
use gtk::traits::*;

pub struct Editor {
    pub widget: ScrolledWindow,
}

impl Editor {
    pub fn new() -> Editor {
        let editor = TextView::new().unwrap();
    
        let editor_scrolled_window = ScrolledWindow::new(
        	None, None
	    ).unwrap();
	
        editor_scrolled_window.set_hexpand(true);
        editor_scrolled_window.set_vexpand(true);
        editor_scrolled_window.add(&editor);
        
        // at least 300 width
        editor_scrolled_window.set_size_request(300, -1);
        
        Editor {
            widget: editor_scrolled_window,
        }
    }
}
