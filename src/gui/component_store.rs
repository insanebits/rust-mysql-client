///
/// Holds GUI widgets
///

use std::collections::HashMap;
use gtk;
use gui;

pub struct ComponentStore<'a> {
	components: HashMap<String, Box<gtk::WidgetTrait + 'a>>,
}

impl <'a>ComponentStore<'a> {
	pub fn new() -> ComponentStore<'a> {
		ComponentStore {
			components: HashMap::new()
		}
	}
}

impl gui::ComponentStoreTrait for ComponentStore {
	fn add_component(&mut self, name: String, component: Box<gtk::WidgetTrait>) -> () {
		self.components.insert(name, component);
	}
	
	fn remove_component(&mut self, name: &String) -> () {
		self.components.remove(name);
	}
	
	fn get_component(&self, name: &String) -> Option<&Box<gtk::WidgetTrait>> {
		self.components.get(name)
	}
}
