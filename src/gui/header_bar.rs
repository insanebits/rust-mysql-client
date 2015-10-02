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
        
        header.set_title("Header goes here");
        
        let header_execute_button = gtk::Button::new().unwrap();
        let header_execute_image = gtk::Image::new_from_icon_name("system-search", 32).unwrap();
        header_execute_button.add(&header_execute_image);
        
        header.pack_end(&header_execute_button);
        
        HeaderBar {
            widget: header
        }
    }
}
