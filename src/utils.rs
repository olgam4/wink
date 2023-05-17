use std::env;

use axum::response::Response;
use rand::{distributions::Alphanumeric, thread_rng, Rng};

pub fn parse_url(url: &str) -> String {
    if url.starts_with("http://") || url.starts_with("https://") {
        url.to_string()
    } else {
        format!("https://{}", url)
    }
}

pub fn redirect(url: &str) -> Response<String> {
    Response::builder()
        .status(301)
        .header("HX-Location", url.clone())
        .header("Location", url)
        .body("".to_string())
        .unwrap()
}

pub fn get_salt_lenght() -> usize {
    let lenght = env::var("SALT_LENGTH").expect("SALT_LENG must be set");
    let lenght = lenght.parse::<usize>().expect("SALT_LENG must be a number");
    lenght
}

pub fn get_nano_length() -> usize {
    let lenght = env::var("NANO_LENGTH").expect("NANO_LENGTH must be set");
    let lenght = lenght.parse::<usize>().expect("NANO_LENGTH must be a number");
    lenght
}

pub fn get_random_string(lenght: usize) -> String {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(lenght)
        .map(char::from)
        .collect();

    rand_string
}
