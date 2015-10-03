///
/// MySQL connector module holds everything related to MySQL communication
///

// Explicitly define what structs does module export
pub use self::server::Server;
pub use self::database::Database;

// Internal modules, some of them might be private
mod server;
mod database;

