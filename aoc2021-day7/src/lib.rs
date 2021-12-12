use worker::*;

use utils;

#[event(fetch)]
pub async fn main(req: Request, env: Env) -> Result<Response> {
    console_error_panic_hook::set_once();

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
    Response::ok(utils::get_input(&req, &ctx, "7").await)
}

async fn part1(req: worker::Request, ctx: worker::RouteContext<()>) -> Result<Response> {
    let positions = utils::get_input(&req, &ctx, "7")
        .await
        .trim()
        .split(",")
        .map(|raw| raw.parse().expect("Couldn't parse integer"))
        .collect::<Vec<u64>>();

    let cost = (0..=positions.iter().max().unwrap().clone())
        .map(|goal| {
            positions
                .iter()
                .map(|&value| u64::max(goal, value) - u64::min(goal, value))
                .sum::<u64>()
        })
        .min()
        .expect("Couldn't find minimum cost");

    Response::ok(cost.to_string())
}

async fn part2(req: worker::Request, ctx: worker::RouteContext<()>) -> Result<Response> {
    let positions = utils::get_input(&req, &ctx, "7")
        .await
        .trim()
        .split(",")
        .map(|raw| raw.parse().expect("Couldn't parse integer"))
        .collect::<Vec<u64>>();

    let cost = (0..=positions.iter().max().unwrap().clone())
        .map(|goal| {
            positions
                .iter()
                .map(|&value| (1..=(u64::max(goal, value) - u64::min(goal, value))).sum::<u64>())
                .sum::<u64>()
        })
        .min()
        .expect("Couldn't find minimum cost");

    Response::ok(cost.to_string())
}
