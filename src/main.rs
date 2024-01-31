use std::fs::{DirBuilder, File};
use crate::rudabaDB::{Data, Database, User};
use delete::{delete_file_async};
use compare::{Compare,Extract};

fn main() {

}

/*
    struct for storing database values
        - database_name: the name of the database to be created and accessed
        - username: the name of the user
 */
pub mod rudabaDB {
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
fn create_database(database: Database, user: User){
    //save user details to a config file
    let database_path = database.database_directory + &*database.database_name;
    DirBuilder::new()
        .create(database_path).unwrap(); //we do not use .recursive to avoid overwriting the database if it exists
}

/*fn create_table(table: Table){
    //ALL DATABASE FILES ARE IN json format
    let mut new_table_name = table.table_name + ".json";
    let mut file = File::create(new_table_name);
}
*/
//add data to a Database object
//example
fn add_data(data: Data){
    let mut new_data_title = data.data_title + ".json";
    let mut file = File::create(new_data_title);
}

fn edit_data(data: Data){
    add_data(Data{data_id: "0123".parse().unwrap(), data_title: "Tools".parse().unwrap(), data: "scraper".parse().unwrap() })
}
//dataID is the
async fn delete_data(data: Data){
    let mut to_be_deleted_data_title = data.data_title + ".json";
    //delete file asynchronously
    delete_file_async(&*to_be_deleted_data_title).await.unwrap();
}
//read data from database using its title
async fn read_data(data_title: String){
    let mut to_be_read:String = Data::from(data_title);    

}
fn update_data(data: Data){
    let mut to_be_edited_data_title: String = data.data_title + ".json";

}
//compare a data tuple element with another
fn compare_data(dataToCompare: Data, referenceData: Data) {
    let comparison_found:bool=false;
        match dataToCompare{
//            referenceData => {comparison_found == true};
            referenceData=> {
                comparison_found = true;
            }
        }
    }
//User x=new User;
fn login_to_database(username: str, password: str){
//    let currentUsername = User{'{}', read_data("currentUser")};// get from db login
//    println!("Welcome '{}'", currentUsername.user_name);
//    compare to database file
    let attemptingUser: User = {user_name: username, password: password} ;
    compare_data(attemptingUser, read_data("username"))
    println!("Welcome '{}'", read_data("username"));
}
trait readable{
    fn read_data($self) -> Data;
    fn login_to_database(&mut self, username: &str, password: &str);

}
trait editable{
    pub fn edit_data($mut self, edited_data_title: str, edited_data : str){
        self.data_title = edited_data_title;
        self.data = edited_data;
    }
}