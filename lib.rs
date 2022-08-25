#![cfg(test)]

use ucan::ucan::Ucan;
use wasm_bindgen_test::wasm_bindgen_test;

#[wasm_bindgen_test]
fn fail() {
    let token = "eyJhbGciOiJFZERTQSIsInR5cCI6IkpXVCIsInVjdiI6IjAuOC4xIn0.eyJpc3MiOiJkaWQ6a2V5Ono2TWtrODliQzNKclZxS2llNzFZRWNjNU0xU01WeHVDZ054NnpMWjhTWUpzeEFMaSIsImF1ZCI6ImRpZDprZXk6ejZNa2ZmRFpDa0NUV3JlZzg4NjhmRzFGR0ZvZ2NKajVYNlBZOTNwUGNXRG45Ym9iIiwiZXhwIjoxNjYxNDUwODk5LCJuYmYiOm51bGwsIm5uYyI6bnVsbCwiYXR0IjpbXSwiZmN0IjpbXSwicHJmIjpbXX0.liZl14IKOyc1eQluPJ39U4AoZLN_Ma9WKp9Vr1ak2VLFcglbZeIJmnEuuyWDRdo-81mv79cZ1tygaPwJrIKfCQ";
    let ucan = Ucan::try_from_token_string(token).unwrap();
    ucan.is_too_early(); // This panics because it call `now()`.
}
