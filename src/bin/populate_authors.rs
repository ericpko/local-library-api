use bson::{doc, Document};
use chrono::{TimeZone, Utc};
use mongodb::{options::ClientOptions, Client};

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
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

    // get a handle to a collection in the database
    let collection_authors = db.collection::<Document>("authors");
    // let authors = vec![
    //     Author::new(
    //         "George".to_string(),
    //         "Orwell".to_string(),
    //         Some(Utc.ymd(1903, 5, 25).and_hms(0, 0, 0)),
    //         Some(Utc.ymd(1950, 1, 21).and_hms(0, 0, 0)),
    //     ),
    //     Author::new(
    //         "Santa".to_string(),
    //         "Claus".to_string(),
    //         Some(Utc.ymd(1981, 12, 25).and_hms(0, 0, 0)),
    //         None,
    //     ),
    // ];
    let authors = vec![
        doc! {
                    "first_name": "Patrick",
                    "family_name": "Rothfuss",
                    "date_of_birth": Utc.ymd(1973, 6, 6).and_hms(0, 0, 0),
                    "date_of_death": null
        },
        doc! {
                    "first_name": "Ben",
                    "family_name": "Bova",
                    "date_of_birth": Utc.ymd(1932, 11, 8).and_hms(0, 0, 0),
                    "date_of_death": null
        },
        doc! {
                    "first_name": "Isaac",
                    "family_name": "Asimov",
                    "date_of_birth": Utc.ymd(1920, 1, 2).and_hms(0, 0, 0),
                    "date_of_death": Utc.ymd(1992, 4, 6).and_hms(0, 0, 0),
        },
        doc! {
                    "first_name": "Bob",
                    "family_name": "Billings",
                    "date_of_birth": null,
                    "date_of_death": null
        },
        doc! {
                    "first_name": "Jim",
                    "family_name": "Jones",
                    "date_of_birth": Utc.ymd(1971, 12, 16).and_hms(0, 0, 0),
                    "date_of_death": null
        },
        doc! {
                    "first_name": "George",
                    "family_name": "Orwell",
                    "date_of_birth": Utc.ymd(1903, 5, 25).and_hms(0, 0, 0),
                    "date_of_death": Utc.ymd(1950, 1, 21).and_hms(0, 0, 0),
        },
        doc! {
                    "first_name": "Santa",
                    "last_name": "Claus",
                    "date_of_birth": Utc.ymd(1959, 12, 25).and_hms(0, 0, 0),
                    "date_of_death": null
        },
    ];

    collection_authors.insert_many(authors, None).await?;

    Ok(())
}
