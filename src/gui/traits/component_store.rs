///
///
///
use std;
use gtk;

pub trait ComponentStoreTrait {
	fn add_component(&mut self, name: String, component: Box<gtk::WidgetTrait>) -> ();
	fn remove_component(&mut self, name: &String) -> ();
	fn get_component(&self, name: &String) -> Option<&Box<gtk::WidgetTrait>>;
}
