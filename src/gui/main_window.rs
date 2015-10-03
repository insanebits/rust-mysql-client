///
/// Main application window class
///

use gtk;
use gtk::traits::*;
use gui;
use connector;
use config;
use std::path::Path;
use std;

pub struct MainWindow {
	window: gtk::Window,
	header: gtk::Box,
	body: gtk::Box,
	footer: gtk::Box
}

impl MainWindow {
	pub fn new() -> MainWindow {
		let window = gtk::Window::new(gtk::WindowType::Toplevel).unwrap();

		window.set_title("First GTK+ Program");
		window.set_border_width(10);
		window.set_window_position(gtk::WindowPosition::Center);
		window.set_default_size(640, 480);

		window.connect_delete_event(|_, _| {
		    gtk::main_quit();
		    gtk::signal::Inhibit(true)
		});
		
		MainWindow {
			window: window,
			header: gtk::Box::new(gtk::Orientation::Vertical, 10).unwrap(),
			body: gtk::Box::new(gtk::Orientation::Vertical, 10).unwrap(),
			footer: gtk::Box::new(gtk::Orientation::Vertical, 10).unwrap(),
		}
	}
	
	pub fn setup(&self) {
		let main_container = gtk::Box::new(gtk::Orientation::Vertical, 0).unwrap();
		// setup parts of the window
		self.setup_header(&main_container);
		self.setup_body(&main_container);
		self.setup_footer(&main_container);
		// done setuping, add it to the window
		self.window.add(&main_container);
	}
	
	fn setup_header(&self, container: &gtk::Box) -> () {
		container.add(&gui::HeaderBar::new().widget);
	}
	
	fn setup_body(&self, container: &gtk::Box) -> () {
		let panels = gtk::Box::new(gtk::Orientation::Horizontal, 0).unwrap();
    	
    	let config_result = config::reader::from_file(Path::new("database.conf"));
    	
    	match config_result {
    		Err(e) => panic!("Error reading config: {}", e.desc),
    		Ok(_) => {} 
    	}
    	
    	let mysql_server = connector::Server::new(config_result.unwrap());
    
		//TODO maybe create container wrapper to get rid of .widget calls every time
		panels.add(&gui::DatabaseBrowser::new(&mysql_server).widget);
		panels.add(&gui::Editor::new().widget);
		
		container.add(&panels);
	}
	
	fn setup_footer(&self, container: &gtk::Box) -> () {
		 // footer
		let footer = gtk::Box::new(gtk::Orientation::Vertical, 10).unwrap();
		
		let scrollable_footer = gtk::ScrolledWindow::new(None, None).unwrap();
		scrollable_footer.set_hexpand(true);
		scrollable_footer.set_vexpand(true);
		scrollable_footer.add(&footer);
		
		container.add(&footer);
	}
	
	pub fn show(&self) -> () {
		self.window.show_all();
	}
}
