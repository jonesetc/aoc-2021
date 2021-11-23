use std::collections::{HashMap, HashSet};

use worker::*;

use utils;

#[event(fetch)]
pub async fn main(req: Request, env: Env) -> Result<Response> {
    let router = Router::new();
    router
        .get_async("/", index)
        .get_async("/1", part1)
        .get_async("/1/", part1)
        .get_async("/2", part2)
        .get_async("/2/", part2)
        .run(req, env)
        .await
}

async fn index(req: worker::Request, ctx: worker::RouteContext<()>) -> Result<Response> {
    Response::ok(utils::get_file_for_day(&utils::get_aoc_session(&req, &ctx), "1").await)
}

async fn part1(req: worker::Request, ctx: worker::RouteContext<()>) -> Result<Response> {
    let input = utils::get_file_for_day(&utils::get_aoc_session(&req, &ctx), "1").await;
    let answer = input
        .split("\n")
        .map(|line| line.parse::<u64>().expect("Non-integer value encountered"))
        .scan(HashSet::new(), |seen, curr| {
            let compliment: u64 = 2020 - curr;
            if seen.contains(&compliment) {
                Some(curr * compliment)
            } else {
                seen.insert(curr);
                Some(0)
            }
        })
        .filter(|&answer| answer != 0)
        .nth(0)
        .expect("No answer was found");

    Response::ok(format!("{}", answer))
}

async fn part2(req: worker::Request, ctx: worker::RouteContext<()>) -> Result<Response> {
    let input = utils::get_file_for_day(&utils::get_aoc_session(&req, &ctx), "1").await;
    let answer = input
        .split("\n")
        .map(|line| line.parse::<u64>().expect("Non-integer value encountered"))
        .scan(
            (HashSet::new(), HashMap::new()),
            |(seen, small_pairs), curr| {
                let compliment: u64 = 2020 - curr;
                if small_pairs.contains_key(&compliment) {
                    Some(curr * small_pairs[&compliment])
                } else {
                    seen.iter().for_each(|prev| {
                        small_pairs.insert(prev + curr, prev * curr);
                    });
                    seen.insert(curr);
                    Some(0)
                }
            },
        )
        .filter(|&answer| answer != 0)
        .nth(0)
        .expect("No answer was found");

    Response::ok(format!("{}", answer))
}
