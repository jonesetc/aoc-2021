use console_error_panic_hook;
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
    Response::ok(utils::get_input(&req, &ctx, "1").await)
}

async fn part1(req: worker::Request, ctx: worker::RouteContext<()>) -> Result<Response> {
    let answer = utils::get_input(&req, &ctx, "1")
        .await
        .lines()
        .map(|line| line.parse::<u64>().expect("Non-integer value encountered"))
        .collect::<Vec<u64>>()
        .windows(2)
        .filter(|pair| pair[1] > pair[0])
        .count();

    Response::ok(format!("{}", answer))
}

async fn part2(req: worker::Request, ctx: worker::RouteContext<()>) -> Result<Response> {
    let answer = utils::get_input(&req, &ctx, "1")
        .await
        .lines()
        .map(|line| line.parse::<u64>().expect("Non-integer value encountered"))
        .collect::<Vec<u64>>()
        .windows(4)
        .filter(|quad| quad[1] + quad[2] + quad[3] > quad[0] + quad[1] + quad[2])
        .count();

    Response::ok(format!("{}", answer))
}
