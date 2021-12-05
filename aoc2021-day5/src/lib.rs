use std::collections::HashMap;

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
    Response::ok(utils::get_input(&req, &ctx, "5").await)
}

async fn part1(req: worker::Request, ctx: worker::RouteContext<()>) -> Result<Response> {
    let overlaps = get_segments(&utils::get_input(&req, &ctx, "5").await)
        .iter()
        .fold(HashMap::new(), |mut counts, ((x1, y1), (x2, y2))| {
            if x1 == x2 {
                for y in u64::min(*y1, *y2)..=u64::max(*y1, *y2) {
                    *counts.entry((*x1, y)).or_insert(0u64) += 1
                }
            } else if y1 == y2 {
                for x in u64::min(*x1, *x2)..=u64::max(*x1, *x2) {
                    *counts.entry((x, *y1)).or_insert(0u64) += 1
                }
            }
            counts
        })
        .values()
        .filter(|&&count| count > 1u64)
        .count();

    Response::ok(overlaps.to_string())
}

async fn part2(req: worker::Request, ctx: worker::RouteContext<()>) -> Result<Response> {
    let overlaps = get_segments(&utils::get_input(&req, &ctx, "5").await)
        .iter()
        .fold(HashMap::new(), |mut counts, ((x1, y1), (x2, y2))| {
            if x1 == x2 {
                for y in u64::min(*y1, *y2)..=u64::max(*y1, *y2) {
                    *counts.entry((*x1, y)).or_insert(0u64) += 1;
                }
            } else if y1 == y2 {
                for x in u64::min(*x1, *x2)..=u64::max(*x1, *x2) {
                    *counts.entry((x, *y1)).or_insert(0u64) += 1;
                }
            } else {
                // lot of repetition, but it's simple
                match (x1 < x2, y1 < y2) {
                    (true, true) => {
                        for (x, y) in (*x1..=*x2).zip(*y1..=*y2) {
                            *counts.entry((x, y)).or_insert(0u64) += 1;
                        }
                    }
                    (true, false) => {
                        for (x, y) in (*x1..=*x2).zip((*y2..=*y1).rev()) {
                            *counts.entry((x, y)).or_insert(0u64) += 1;
                        }
                    }
                    (false, true) => {
                        for (x, y) in ((*x2..=*x1).rev()).zip(*y1..=*y2) {
                            *counts.entry((x, y)).or_insert(0u64) += 1;
                        }
                    }
                    (false, false) => {
                        for (x, y) in ((*x2..=*x1).rev()).zip((*y2..=*y1).rev()) {
                            *counts.entry((x, y)).or_insert(0u64) += 1;
                        }
                    }
                }
            }
            counts
        })
        .values()
        .map(|value| value)
        .filter(|&&count| count > 1u64)
        .count();

    Response::ok(overlaps.to_string())
}

fn get_segments(input: &str) -> Vec<((u64, u64), (u64, u64))> {
    input
        .lines()
        .map(|line| {
            let segment = line
                .split(" -> ")
                .map(|raw_coords| {
                    let coords = raw_coords
                        .split(",")
                        .map(|raw_coord| raw_coord.parse().expect("Couldn't parse integer"))
                        .collect::<Vec<u64>>();
                    (coords[0], coords[1])
                })
                .collect::<Vec<(u64, u64)>>();
            (segment[0], segment[1])
        })
        .collect()
}
