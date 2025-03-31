use sqlx::PgPool;
use async_trait::async_trait;

trait Crud {
    async fn create(&self, pool: &PgPool, person: Person) -> Result<Person, String>;
    async fn read($self, pool: &PgPool) -> Result<Person, String>;
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
struct Person {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub age: i32,
}


impl Crud for Person {
    pub async fn create(pool: &PgPool, person: Person) -> Result<Person, String{
        sqlx::query!("INSERT INTO users (name, email, age) VALUES ($1, $2)", person.name, person.email, person.age)
        .fetch_one(pool)
        .await?;
       

        match result {
            Ok(row) => {
                (Person {
                    id: row.id,
                    name: person.name,
                    email: person.email,
                    age: person.age
                })
            }
        }
    }
    
    pub async fn read(&self, pool: &PgPool, person: Person) -> Result<Person, String>{
        let Person = sqlx::query!("SELECT * from users")
                    .execute(pool)
                    .await?;
        Ok(Person);
    
    }


}

