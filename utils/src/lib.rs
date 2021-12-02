use std::{borrow::Cow, collections::HashMap};

use worker::*;

pub async fn get_input(req: &Request, ctx: &RouteContext<()>, day: &str) -> String {
    get_file_for_day(&get_aoc_session(req, ctx), day).await
}

fn get_aoc_session(req: &Request, ctx: &RouteContext<()>) -> String {
    req.url()
        .expect("No URL found")
        .query_pairs()
        .collect::<HashMap<Cow<str>, Cow<str>>>()
        .get("aoc_session")
        .map(|cow| cow.to_string())
        .or_else(|| ctx.secret("AOC_SESSION").ok().map(|s| s.to_string()))
        .expect("No AOC session available")
}

async fn get_file_for_day(session: &str, day: &str) -> String {
    let mut request_headers = Headers::new();
    request_headers
        .append("Cookie", &format!("session={}", session))
        .expect("Couldn't set cookie");

    Fetch::Request(
        Request::new_with_init(
            &format!("https://adventofcode.com/2021/day/{}/input", day),
            RequestInit::new().with_headers(request_headers),
        )
        .expect("Couldn't build request for day's input"),
    )
    .send()
    .await
    .expect("Couldn't get day's input")
    .text()
    .await
    .expect("Couldn't unwrap day's input")
}
