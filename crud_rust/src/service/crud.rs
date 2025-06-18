use sqlx::PgPool;
use async_trait::async_trait;
use serde::{Serialize, Deserialize};

#[async_trait]
pub trait Crud {
    async fn create(pool: &PgPool, person: Person) -> Result<Person, String>;
    async fn read(pool: &PgPool, id: i32) -> Result<Option<Person>, String>;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Person {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub age: i32,
}

#[async_trait]
impl Crud for Person {
    async fn create(pool: &PgPool, person: Person) -> Result<Person, String> {
        let result = sqlx::query!(
            "INSERT INTO users (id, name, email, age) VALUES ($1, $2, $3, $4) RETURNING id",
            person.id, person.name, person.email, person.age
        )
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string());

        match result {
            Ok(row) => Ok(Person {
                id: row.id,
                name: person.name,
                email: person.email,
                age: person.age
            }),
            Err(e) => Err(e)
        }
    }
    
    async fn read(pool: &PgPool, id: i32) -> Result<Option<Person>, String> {
        let person = sqlx::query_as!(
            Person,
            "SELECT id, name, email, age FROM users WHERE id = $1",
            id
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| e.to_string());
        
        Ok(person)
    }
}

