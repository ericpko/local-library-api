use axum_local_library::models::{
    author::Author,
    book::Book,
    book_instance::{BookInstance, Status},
    genre::Genre,
};
use bson::oid::ObjectId;
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

    // populate the database

    // Authors
    let mut author_ids = Vec::new();
    for _ in 0..6 {
        author_ids.push(ObjectId::new());
    }
    let authors = vec![
        Author::new(
            Some(author_ids[0]),
            "Patrick".to_string(),
            "Rothfuss".to_string(),
            Some(Utc.ymd(1973, 6, 6).and_hms(0, 0, 0)),
            None,
        ),
        Author::new(
            Some(author_ids[1]),
            "Ben".to_string(),
            "Bova".to_string(),
            Some(Utc.ymd(1932, 11, 8).and_hms(0, 0, 0)),
            None,
        ),
        Author::new(
            Some(author_ids[2]),
            "Isaac".to_string(),
            "Asimov".to_string(),
            Some(Utc.ymd(1920, 1, 2).and_hms(0, 0, 0)),
            Some(Utc.ymd(1992, 4, 6).and_hms(0, 0, 0)),
        ),
        Author::new(
            Some(author_ids[3]),
            "Bob".to_string(),
            "Billings".to_string(),
            None,
            None,
        ),
        Author::new(
            Some(author_ids[4]),
            "Jim".to_string(),
            "Jones".to_string(),
            Some(Utc.ymd(1971, 12, 16).and_hms(0, 0, 0)),
            None,
        ),
        Author::new(
            Some(author_ids[5]),
            "George".to_string(),
            "Orwell".to_string(),
            Some(Utc.ymd(1903, 5, 25).and_hms(0, 0, 0)),
            Some(Utc.ymd(1950, 1, 21).and_hms(0, 0, 0)),
        ),
    ];
    let author_collection = db.collection::<Author>("authors");
    author_collection.insert_many(authors, None).await?;

    // Genres
    let mut genre_ids = Vec::new();
    for _ in 0..3 {
        genre_ids.push(ObjectId::new());
    }
    let genres = vec![
        Genre::new(Some(genre_ids[0]), "Fantasy".to_owned()),
        Genre::new(Some(genre_ids[1]), "Science Fiction".to_string()),
        Genre::new(Some(genre_ids[2]), "French Poetry".to_string()),
    ];
    let genre_collection = db.collection::<Genre>("genres");
    genre_collection.insert_many(genres, None).await?;

    // Books
    let mut book_ids = Vec::new();
    for _ in 0..6 {
        book_ids.push(ObjectId::new());
    }
    let books = vec![
        Book::new(
            Some(book_ids[0]),
            "Apes and Angels".to_owned(),
            author_ids[0],
            "Humankind headed out to the stars not for conquest, nor exploration".to_string(),
            "9780765379528".to_string(),
            Some(vec![genre_ids[0]]),
        ),
        Book::new(
            Some(book_ids[1]),
            "The Name of the Wind".to_string(),
            author_ids[1],
            "I have stolen princesses".to_string(),
            "9781473211896".to_string(),
            Some(vec![genre_ids[0], genre_ids[2]]),
        ),
        Book::new(
            Some(book_ids[2]),
            "The wise man".to_string(),
            author_ids[2],
            "Picking up the tale of".to_string(),
            "2458252472087".to_string(),
            None,
        ),
        Book::new(
            Some(book_ids[3]),
            "The Slow Regard".to_string(),
            author_ids[3],
            "Deep below the university".to_string(),
            "2482547825048".to_string(),
            Some(vec![genre_ids[1], genre_ids[2]]),
        ),
        Book::new(
            Some(book_ids[4]),
            "Test book 2".to_string(),
            author_ids[4],
            "Summary of test book 2".to_string(),
            "2480274727240".to_string(),
            Some(vec![genre_ids[1]]),
        ),
        Book::new(
            Some(book_ids[5]),
            "Death wave".to_string(),
            author_ids[5],
            "In blah blah".to_string(),
            "284257241700".to_string(),
            Some(vec![genre_ids[2]]),
        ),
    ];
    let book_collection = db.collection::<Book>("books");
    book_collection.insert_many(books, None).await?;

    // Book Instances
    let book_instances = vec![
        BookInstance::new(
            None,
            book_ids[0],
            "Gonzallas, 2011".to_string(),
            Status::Available,
        ),
        BookInstance::new(None, book_ids[1], "My imprint".to_string(), Status::Loaned),
        BookInstance::new(
            None,
            book_ids[3],
            "imprint imprint imprint".to_string(),
            Status::Reserved,
        ),
        BookInstance::new(None, book_ids[1], "My imprint".to_string(), Status::Loaned),
        BookInstance::new(None, book_ids[1], "My imprint".to_string(), Status::Loaned),
        BookInstance::new(None, book_ids[1], "My imprint".to_string(), Status::Loaned),
        BookInstance::new(
            None,
            book_ids[3],
            "imprint imprint imprint".to_string(),
            Status::Reserved,
        ),
        BookInstance::new(
            None,
            book_ids[5],
            "imprint #5".to_string(),
            Status::Maintenance,
        ),
    ];
    let book_instance_collection = db.collection::<BookInstance>("bookinstances");
    book_instance_collection
        .insert_many(book_instances, None)
        .await?;

    Ok(())
}
