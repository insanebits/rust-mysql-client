///
/// MySQL server struct which handles server itself related things like database 
/// name, encodings and etc.
///

use mysql::conn::MyOpts;
use mysql::conn::QueryResult;
use mysql::conn::pool::MyPool;
use mysql::error::MyResult;
use mysql::value;

use connector::Database;
use config::types::Config;
use config;

pub struct Server {
    pub name: String,
    pub databases: Vec<Database>,
    pub pool: MyPool
}

impl Server {
	// TODO split into config reading, connecting, data loading
	pub fn new(config: Config) -> Server {
	  let opts = MyOpts {
		user: Some(config.lookup_str("username").unwrap().to_string()),
		pass: Some(config.lookup_str("password").unwrap().to_string()),
		..Default::default()
	  };
	   
	  let pool = MyPool::new(opts).unwrap();    
	  let mut server =  Server {
		name: "localhost".to_string(),
		databases: Vec::new(),
		pool: pool
	  };

	  server.load_metadata();
	  
	  server
	}
    pub fn load_metadata(&mut self) -> () {
        self.databases = 
        self.pool.prep_exec(r"SHOW databases", ())
            .map(|result| { // In this closure we sill map `QueryResult` to `Vec<Payment>`
                result.map(|x| x.unwrap()).map(|row| {
                    // read row
                    let db_name: String = value::from_row(row);
                    
                    // return Database instance
                    Database {
                        name: db_name.clone(), // we need a copy here not a value to be moved to struct
                        tables: self.get_db_tables(db_name)
                    }
                }).collect() // collect mapped values into Vec<Database>
            }).unwrap(); // unwrap because collect may fail
    }
    
    fn get_db_tables(&self, database: String) -> Vec<String> {

        let mut conn = self.pool.get_conn().unwrap();
        
        let result: MyResult<QueryResult> = 
        conn.prep_exec("SELECT table_name from information_schema.tables where table_schema=?", (database,));

        let mut tables = Vec::new();

        match result {
            Ok(mysql_result) => {
                for row in mysql_result {
                    let str: String = value::from_row(row.unwrap());
                    tables.push(str);
                }
            }

            Err(e) => {
                println!("Failed {}", e);
            }
        }

        tables
    }
}



