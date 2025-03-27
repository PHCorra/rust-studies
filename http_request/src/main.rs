// Study based: Mods, Struct, Imports, Complex Returns, Cargo Dependencies

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    
    let response = reqwest::get("http://httpbin.org/get").await?;

    let body = response.text().await?;

    println!("Answer: {}", body);

    Ok(())
}
