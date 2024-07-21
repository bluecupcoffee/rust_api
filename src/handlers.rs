use warp::Filter;
use super::models::*;

pub async fn get_post(id: u64) -> Result<impl warp::Reply, warp::Rejection> {
    let book = Book {
        id,
        title: String::from("Hi there"),
        body: String::from("This is a post from warp")
    };

    Ok(warp::reply::json(&book))
}