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
    Response::ok(utils::get_input(&req, &ctx, "2").await)
}

async fn part1(req: worker::Request, ctx: worker::RouteContext<()>) -> Result<Response> {
    let final_position = utils::get_input(&req, &ctx, "2")
        .await
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        .map(|parts| {
            (
                parts[0],
                parts[1].parse::<u64>().expect("Failed to parse integer"),
            )
        })
        .fold((0, 0), |pos, instruction| match instruction.0 {
            "forward" => (pos.0 + instruction.1, pos.1),
            "down" => (pos.0, pos.1 + instruction.1),
            "up" => (pos.0, pos.1 - instruction.1),
            _ => panic!("Unrecognized direction encountered"),
        });

    Response::ok((final_position.0 * final_position.1).to_string())
}

async fn part2(req: worker::Request, ctx: worker::RouteContext<()>) -> Result<Response> {
    let final_position = utils::get_input(&req, &ctx, "2")
        .await
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        .map(|parts| {
            (
                parts[0],
                parts[1].parse::<u64>().expect("Failed to parse integer"),
            )
        })
        .fold((0, 0, 0), |pos, instruction| match instruction.0 {
            "forward" => (pos.0 + instruction.1, pos.1 + pos.2 * instruction.1, pos.2),
            "down" => (pos.0, pos.1, pos.2 + instruction.1),
            "up" => (pos.0, pos.1, pos.2 - instruction.1),
            _ => panic!("Unrecognized direction encountered"),
        });

    Response::ok((final_position.0 * final_position.1).to_string())
}
