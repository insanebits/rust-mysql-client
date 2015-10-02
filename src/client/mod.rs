pub struct Database
{
    name: String,
    tables: Vec<String>
}

pub struct DbServer {
    name: String,
    databases: Vec<Database>,
    pool: MyPool
}

impl DbServer {
    fn load_metadata(&mut self) -> () {
        self.databases = 
        self.pool.prep_exec(r"SHOW databases", ())
            .map(|result| { // In this closure we sill map `QueryResult` to `Vec<Payment>`
                result.map(|x| x.unwrap()).map(|row| {
                    // read row
                    let db_name: String = from_row(row);
                    
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
                    let str: String = from_row(row.unwrap());
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

