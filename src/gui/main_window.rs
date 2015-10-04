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
use gui::ComponentStoreTrait;

pub struct MainWindow<'a> {
	window: gtk::Window,
	components: gui::ComponentStore,
	header: gtk::Box,
	body: gtk::Box,
	footer: gtk::Box
}

impl MainWindow {
	pub fn new() -> MainWindow {
		let window = gtk::Window::new(gtk::WindowType::Toplevel).unwrap();

		window.set_title("MySQL Browser");
		window.set_border_width(0);
		window.set_window_position(gtk::WindowPosition::Center);
		window.set_default_size(640, 480);

		window.connect_delete_event(|_, _| {
		    gtk::main_quit();
		    gtk::signal::Inhibit(true)
		});
		
		MainWindow {
			window: window,
			components: gui::ComponentStore::new(),
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
	
	fn setup_header(&mut self, container: &gtk::Box) -> () {	
		let name = "header".to_string();
		self.components.add_component(name, &Box::new(gui::HeaderBar::new().widget));
		// dereference box and borrow value
		container.add(&*self.components.get_component(&name).unwrap());
	}
	
	fn setup_body(&self, container: &gtk::Box) -> () {
    	let config_result = config::reader::from_file(Path::new("database.conf"));
    	
    	match config_result {
    		Err(e) => panic!("Error reading config: {}", e.desc),
    		Ok(_) => {} 
    	}
    	
    	let mysql_server = connector::Server::new(config_result.unwrap());
    	let panels = gtk::Paned::new(gtk::Orientation::Horizontal).unwrap();
    	
		//TODO maybe create container wrapper to get rid of .widget calls every time
		
		panels.pack1(&gui::DatabaseBrowser::new(&mysql_server).widget, true, false);
		panels.pack2(&gui::Editor::new().widget, true, false);
		
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
