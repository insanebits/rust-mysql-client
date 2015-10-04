///
/// Export public structs 
///



pub use self::editor::Editor;
pub use self::database_browser::DatabaseBrowser;
pub use self::header_bar::HeaderBar;
pub use self::result_list::ResultList;
pub use self::main_window::MainWindow;
pub use self::component_store::ComponentStore;

pub use self::traits::ComponentStoreTrait;


// local modules
mod editor;
mod database_browser;
mod header_bar;
mod result_list;
mod main_window;
mod component_store;
// make traits public
pub mod traits;


