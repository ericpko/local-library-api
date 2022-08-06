use mongodb::{options::ClientOptions, Client, Database};

pub async fn init_db() -> mongodb::error::Result<Database> {
    dotenv::dotenv().ok();
    let mongodb_uri = dotenv::var("MONGODB_URI").unwrap();

    let mut client_options = ClientOptions::parse(mongodb_uri).await?;
    client_options.app_name = Some("Local Library".to_string());

    // get a handle to the deployment
    let client = Client::with_options(client_options)?;

    // list the names of the databases in that deployment
    println!("Databases:");
    for db_name in client.list_database_names(None, None).await? {
        println!("    - {db_name}");
    }

    // get a handle to the database
    let db = client.database("axum");

    // list the names of the collections in that database
    println!("\nCollections:");
    for collection_name in db.list_collection_names(None).await? {
        println!("    - {collection_name}");
    }

    Ok(db)
}
