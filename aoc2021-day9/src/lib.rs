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
    Response::ok(utils::get_input(&req, &ctx, "9").await)
}

async fn part1(req: worker::Request, ctx: worker::RouteContext<()>) -> Result<Response> {
    let readings = get_readings(utils::get_input(&req, &ctx, "9").await);
    let height = readings.len();
    let total = readings
        .iter()
        .enumerate()
        .map(|(i, row)| {
            let width = row.len();
            row.iter()
                .enumerate()
                .map(|(j, &reading)| {
                    if get_neighbors(&(i, j), &(height, width))
                        .iter()
                        .all(|(x, y)| readings[*x][*y] > reading)
                    {
                        reading + 1
                    } else {
                        0
                    }
                })
                .sum::<u64>()
        })
        .sum::<u64>();

    Response::ok(total.to_string())
}

async fn part2(req: worker::Request, ctx: worker::RouteContext<()>) -> Result<Response> {
    Response::ok(utils::get_input(&req, &ctx, "9").await)
}

fn get_readings(raw: String) -> Vec<Vec<u64>> {
    raw.lines()
        .map(|line| {
            line.chars()
                .map(|raw| raw.to_digit(10).expect("Couldn't parse integer") as u64)
                .collect()
        })
        .collect::<Vec<Vec<u64>>>()
}

fn get_neighbors(start: &(usize, usize), max: &(usize, usize)) -> Vec<(usize, usize)> {
    let mut coords = vec![];
    if start.0 != 0 {
        coords.push((start.0 - 1, start.1));
    }
    if start.0 != max.0 - 1 {
        coords.push((start.0 + 1, start.1));
    }
    if start.1 != 0 {
        coords.push((start.0, start.1 - 1));
    }
    if start.1 != max.1 - 1 {
        coords.push((start.0, start.1 + 1));
    }

    coords
}
