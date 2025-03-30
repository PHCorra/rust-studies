// Study based: Mods, Struct, Imports, Complex Returns, Cargo Dependencies
mod utils;

use utils::http_adapter::handle_http_request;
use tokio;


#[tokio::main]
async fn main() {
    
    let url = "https://jsonplaceholder.typicode.com/posts/1";

    match handle_http_request(url).await {
        Ok(adapter) => {
            println!("URL: {}", adapter.url);
            println!("Response Content: {}", adapter.content);
        }
        Err(e) => println!("Error> {}", e), 
    }
}
