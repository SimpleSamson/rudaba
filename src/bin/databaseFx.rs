use std::fs::{DirBuilder, File};
use std::string;
//use crate::rudaba_db::{Data, Database, DatabaseAdmins, User}; 
use delete::delete_file_async;
use std::path::Path;
use std::io::{self, BufRead, BufReader, Read};
use std::num::ParseIntError;
use std::io::ErrorKind;

fn main(){

}
#[derive(Debug)]
enum DataFileParseError{
    Io,
    Parse,
    NotFound,
}
/*
    struct for storing database values
        - database_name: the name of the database to be created and accessed
        - username: the name of the user
 */
pub mod rudaba_db {
    pub struct Database {
        pub(crate) database_name: String,
        host_name: String,
        user_name: String,
        pub(crate) database_directory: String //create a default database location when user does not add their own
    }
    //struct for storing user data
    //
    //user_id stores the unique user identifier in String form
    //username stores the user name as a String
    //password stores the password of a user as a string, NOTE: TODO: change this to an encrypted value
    //example:     rudabaDB::User{user_id: "12648465".parse().unwrap(), user_name: "example".parse().unwrap(), password: "54fchx654gh".parse().unwrap() }
    pub(crate) struct User {
        pub(crate) user_id: String,
        //can be combination of letters and integers but preferably integer
        pub(crate) user_name: String,
        pub(crate) password: String   //should be able to hold encrypted data
    }

    //the users who are allowed to access files create new users...
    pub(crate) struct DatabaseAdmins{
        pub user_id: String,
        pub user_password: String,
        pub username: String
    }

    /* //tables better to use data since this is a no sql
    struct Table{
        table_name: String,
        column_name: String,
        column_number: i128,
    } */
    //Data: used to store information in files
    pub struct Data{
        pub(crate) data_id: String, //this should be unique to enable the data to be located when a search isd performed
        pub(crate) data_title: String,
        pub(crate) data: String
    }
}
//create a new databse
impl rudaba_db::Database {
    fn create_database(database: rudaba_db::Database, user: rudaba_db::User){
        //save user details to a config file
        let database_path = database.database_directory + &*database.database_name;
        DirBuilder::new()
            .create(database_path).unwrap(); //we do not use .recursive to avoid overwriting the database if it exists
    }
}


/*fn create_table(table: Table){
    //ALL DATABASE FILES ARE IN json format
    let mut new_table_name = table.table_name + ".json";
    let mut file = File::create(new_table_name);
}
*/
//add data to a Database object
//example
fn add_data(data: rudaba_db::Data){
    let new_data_title = data.data_title + ".json";
    let file = File::create(new_data_title);
}


fn edit_data(data: rudaba_db::Data){
    add_data(rudaba_db::Data{data_id: "0123".parse().unwrap(), data_title: "Tools".parse().unwrap(), data: "scraper".parse().unwrap() })
}

//dataID is the
async fn delete_data(data: rudaba_db::Data){
    let to_be_deleted_data_title = data.data_title + ".json";
    //delete file asynchronously
    delete_file_async(&*to_be_deleted_data_title).await.unwrap();
}

//read data from database using its title
pub async fn read_data(_selected_data_title: String, data_input: rudaba_db::Data){
    match data_input{
        rudaba_db::Data {
            data_title: _selected_data_title,
            ..
        } => println!("{:?}", &data_input.data) //TODO check this to ensure right value returned,//, data 
    }
}
//read the values from a file
pub fn read_data_values(data_title: String) -> Result<String, DataFileParseError>{
    fn database_error(the_error: std::io::Error) -> String{format!("error found at database retrieval")}//;
    let the_error:Result<String, i32> = Err(2); //continue trying here
    let xy = "database/".to_owned() + &data_title;
    let f = File::open(xy).expect("error reading values");//.map_err(database_error);
    let mut reader = BufReader::new(f);//.expect("unable to open file");
    let mut contents = String::with_capacity(1000);

    //read entire file as a string
    reader.read_to_string(&mut contents).expect("Can't read file!");

    let SEARCH_FOR: &str = &data_title;
    for line in contents.lines(){
        if line.starts_with(SEARCH_FOR){
            return line[SEARCH_FOR.len()..].parse::<String>().map_err(|_e| DataFileParseError::Parse);
        }
    }
    return Err(DataFileParseError::NotFound);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>,{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
fn update_data(data: rudaba_db::Data){
    let to_be_edited_data_title: String = data.data_title + ".json";

}
//compare a data tuple element with another
fn compare_data(data_to_compare: rudaba_db::Data, reference_data: rudaba_db::Data) {
    let mut comparison_found:bool=false;
    match data_to_compare{
        reference_data=> {
            comparison_found = true;
        }
    }
}
fn compare_user_data(data_to_compare: rudaba_db::User, reference_data: rudaba_db::User) {
    let mut comparison_found:bool=false;
    match data_to_compare{
        reference_data=> {
            comparison_found = true;
        }
    }
}
//User x=new User;
fn login_to_database(username: String, password: String, admin_user_id: String){
    let un :String =  username.clone();

    let attempting_user: rudaba_db::User = rudaba_db::User { 
//        user_id: (_), 
        user_name: (username),
        password: (password),
        user_id : "%".to_owned()
    };
    let dab = rudaba_db::DatabaseAdmins{
        user_password: "%".to_string(),
        username: "%".to_string(),
        user_id: admin_user_id
    }; 
    let existing_user: rudaba_db::User = rudaba_db::User{
            user_id: dab.user_id, 
            user_name: dab.username, 
            password: dab.user_password
        };

    compare_user_data(attempting_user, existing_user);
                println!("Welcome {}", un);
}
trait Readable{
    fn read_data(&mut self) -> rudaba_db::Data;
    fn login_to_database(&mut self, username: &str, password: &str);

}
trait Editable{
    fn edit_data(&mut self, edited_data_title: &str, edited_data : &str){
        rudaba_db::Data{
            data_title: edited_data_title.to_owned(),
            data: edited_data.to_owned(),
            data_id: "%".to_owned()
        };
    }
    fn delete_data(&mut self, data_to_be_deleted: rudaba_db::Data, data_source:rudaba_db::Data){
       // self.data_title = data_to_be_deleted;
        data_source.data_id.replace(data_to_be_deleted.data_id.as_str(), "");
    }
}