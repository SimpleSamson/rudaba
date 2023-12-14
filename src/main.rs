use std::fs::{DirBuilder, File};

fn main() {
    println!("Welcome {username}");
}

/*
    struct for storing database values
        - database_name: the name of the database to be created and accessed
        - username: the name of the user
 */
struct Database{
    database_name: String,
    host_name: String,
    user_name: String,

}
struct User{
    user_id: String, //can be combination of letters and integers but preferably integer
    user_name: String,
    password: String   //should be able to hold encrypted data
}

//tables
struct Table{
    table_name: String,
}
//create a new databse
fn create_database(database: Database, user: User){
    //save user details to a config file
    let databasePath = database.database_name;
    DirBuilder::new()
        .create(databasePath).unwrap(); //we do not use .recursive to avoid overwriting the database if it exists

}
fn create_table(table: Table){
    //ALL DATABASE FILES ARE IN json format
    let mut new_table_name = table.table_name + ".json";
    let mut file = File::create(new_table_name);

}