fn main() {
    println!("Welcome {username}");
}

/*
    struct for storing database values
        - databaseName: the name of the database to be created and accessed
        - username: the name of the user
 */struct Database{
    databaseName: String,
    hostName: String,
    userName: String,

}

//create a new databse
fn createDatabase(database: Database, user)