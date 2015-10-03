///
/// Database struct responsible for database abstraction, holds objects like 
/// list of tables, views and etc.
///



pub struct Database
{
    pub name: String,
    pub tables: Vec<String>
}

impl Clone for Database {
	fn clone(&self) -> Database {
		return Database {
			name: self.name.clone(),
			tables: self.tables.clone()
		}
	}
}



