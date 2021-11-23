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
    Response::ok(utils::get_file_for_day(
        &utils::get_aoc_session(&req, &ctx),
        "0",
    ).await)
}

async fn part1(req: worker::Request, ctx: worker::RouteContext<()>) -> Result<Response> {
    index(req, ctx).await
}

async fn part2(req: worker::Request, ctx: worker::RouteContext<()>) -> Result<Response> {
    index(req, ctx).await
}
