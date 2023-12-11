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
//create a new databse
fn createDatabase(database: Database, user: User){
    
}