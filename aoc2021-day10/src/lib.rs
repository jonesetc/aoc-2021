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
    Response::ok(utils::get_input(&req, &ctx, "10").await)
}

async fn part1(req: worker::Request, ctx: worker::RouteContext<()>) -> Result<Response> {
    let matching_table: HashMap<char, char> = vec![('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]
        .into_iter()
        .collect();

    let scoring_table: HashMap<char, u64> =
        vec![(')', 3u64), (']', 57u64), ('}', 1197u64), ('>', 25137u64)]
            .into_iter()
            .collect();

    let score = utils::get_input(&req, &ctx, "10")
        .await
        .lines()
        .map(|line| {
            let mut stack = vec![];
            for char in line.chars() {
                if matching_table.contains_key(&char) {
                    stack.push(char);
                } else {
                    let opener = stack.pop().expect("Couldn't pop from stack");
                    if char != matching_table[&opener] {
                        return scoring_table[&char];
                    }
                }
            }
            0u64
        })
        .sum::<u64>();

    Response::ok(score.to_string())
}

async fn part2(req: worker::Request, ctx: worker::RouteContext<()>) -> Result<Response> {
    let matching_table: HashMap<char, char> = vec![('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]
        .into_iter()
        .collect();

    let scoring_table: HashMap<char, u64> =
        vec![(')', 1u64), (']', 2u64), ('}', 3u64), ('>', 4u64)]
            .into_iter()
            .collect();

    let mut scores = utils::get_input(&req, &ctx, "10")
        .await
        .lines()
        .map(|line| {
            let mut stack = vec![];
            for char in line.chars() {
                if matching_table.contains_key(&char) {
                    stack.push(char);
                } else {
                    let opener = stack.pop().expect("Couldn't pop from stack");
                    if char != matching_table[&opener] {
                        return 0;
                    }
                }
            }

            stack
                .into_iter()
                .rev()
                .map(|opener| matching_table[&opener])
                .map(|closer| scoring_table[&closer])
                .fold(0, |score, curr| (score * 5) + curr)
        })
        .filter(|&score| score > 0)
        .collect::<Vec<u64>>();
    scores.sort();

    Response::ok((scores[scores.len() / 2]).to_string())
}
