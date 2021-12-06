use std::collections::HashMap;

use worker::*;

use utils;

#[event(fetch)]
pub async fn main(req: Request, env: Env) -> Result<Response> {
    // console_error_panic_hook::set_once();

    Router::new()
        .get_async("/", index)
        .get_async("/1", part1)
        .get_async("/1/", part1)
        .get_async("/2", part2)
        .get_async("/2/", part2)
        .run(req, env)
        .await
}

async fn index(req: worker::Request, ctx: worker::RouteContext<()>) -> Result<Response> {
    Response::ok(utils::get_input(&req, &ctx, "6").await)
}

async fn part1(req: worker::Request, ctx: worker::RouteContext<()>) -> Result<Response> {
    Response::ok(simulate(&utils::get_input(&req, &ctx, "6").await, 80).to_string())
}

async fn part2(req: worker::Request, ctx: worker::RouteContext<()>) -> Result<Response> {
    Response::ok(simulate(&utils::get_input(&req, &ctx, "6").await, 256).to_string())
}

fn simulate(raw_input: &str, iterations: u64) -> u64 {
    let starting_fish = raw_input
        .trim()
        .split(",")
        .map(|raw| raw.parse::<u8>().expect("Couldn't parse integer"))
        .fold(HashMap::new(), |mut fish, day| {
            *fish.entry(day).or_insert(0u64) += 1;
            fish
        });

    let final_fish = (0..iterations).fold(starting_fish, |old_fish, _| {
        // easier to do it this way than try to create pairs as 7 and 0 will clash
        let mut new_fish = HashMap::new();
        for (day, &count) in old_fish.iter() {
            match day {
                0 => {
                    *new_fish.entry(6).or_insert(0u64) += count;
                    new_fish.insert(8, count);
                }
                _ => {
                    *new_fish.entry(day - 1).or_insert(0u64) += count;
                }
            }
        }

        new_fish
    });

    final_fish.values().sum::<u64>()
}
