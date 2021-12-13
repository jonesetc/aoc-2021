use std::collections::{HashMap, HashSet};

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
    Response::ok(utils::get_input(&req, &ctx, "8").await)
}

async fn part1(req: worker::Request, ctx: worker::RouteContext<()>) -> Result<Response> {
    let uniques = utils::get_input(&req, &ctx, "8")
        .await
        .lines()
        .flat_map(|line| {
            line.split(" | ").collect::<Vec<&str>>()[1]
                .split_whitespace()
                .map(|output| output.len())
        })
        .filter(|&len| len == 2 || len == 3 || len == 4 || len == 7)
        .count();

    Response::ok(uniques.to_string())
}

async fn part2(req: worker::Request, ctx: worker::RouteContext<()>) -> Result<Response> {
    let total = utils::get_input(&req, &ctx, "8")
        .await
        .lines()
        .map(|line| {
            let parts = line
                .split(" | ")
                .map(|put| put.split_whitespace().collect())
                .collect::<Vec<Vec<&str>>>();

            let input_sets = &parts[0]
                .iter()
                .map(|input| input.chars().collect::<HashSet<char>>())
                .map(|segments| (segments.len() as u8, segments))
                .filter(|(len, _)| {
                    match len {
                        // The only needed info comes from 1 (2 segments) and 4 (4 segments)
                        2 | 4 => true,
                        _ => false,
                    }
                })
                .collect::<HashMap<u8, HashSet<char>>>();

            let num = &parts[1]
                .iter()
                .map(|output| output.chars().collect::<HashSet<char>>())
                .map(|segments| match segments.len() {
                    2 => '1',
                    3 => '7',
                    4 => '4',
                    5 => {
                        if segments.intersection(&input_sets[&2]).count() == 2 {
                            '3'
                        } else if segments.intersection(&input_sets[&4]).count() == 2 {
                            '2'
                        } else {
                            '5'
                        }
                    }
                    6 => {
                        if segments.intersection(&input_sets[&2]).count() == 1 {
                            '6'
                        } else if segments.intersection(&input_sets[&4]).count() == 4 {
                            '9'
                        } else {
                            '0'
                        }
                    }
                    7 => '8',
                    _ => {
                        panic!("unexpected number of segments")
                    }
                })
                .collect::<String>();

            num.parse::<u64>().expect("Couldn't parse output")
        })
        .sum::<u64>();

    Response::ok(total.to_string())
}
