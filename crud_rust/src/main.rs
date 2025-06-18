// Study based: Traits, CRUD, Db Connection, Struct
mod config;
mod service;

use config::db_config::db_connection;
use service::crud::{Person, Crud};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Use environment variable or configuration file for database URL in production
    let db_url = "postgres://username:password@localhost:5432/database_name";
    
    // Connect to the database
    let pool = db_connection(db_url).await?;
    println!("Database connected successfully");

    // Create a new person
    let person = Person {
        id: 1,
        name: "John Doe".to_string(),
        email: "john.doe@example.com".to_string(),
        age: 30,
    };

    println!("Creating person: {}", person.name);

    // Create the person in the database
    match Person::create(&pool, person).await {
        Ok(created_person) => {
            println!("Person created successfully, name: {}", created_person.name);
            
            // Read the person from the database
            match Person::read(&pool, created_person.id).await {
                Ok(Some(read_person)) => {
                    println!("Person read successfully: {:?}", read_person);
                },
                Ok(None) => {
                    println!("Person not found");
                },
                Err(e) => {
                    eprintln!("Error reading person: {}", e);
                }
            }
        },
        Err(e) => {
            eprintln!("Error creating person: {}", e);
        }
    }

    Ok(())
}
