use std::collections::HashSet;

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
    Response::ok(utils::get_input(&req, &ctx, "4").await)
}

async fn part1(req: worker::Request, ctx: worker::RouteContext<()>) -> Result<Response> {
    let input = utils::get_input(&req, &ctx, "4").await;
    let numbers = input
        .lines()
        .nth(0)
        .expect("Couldn't read input")
        .split(",")
        .map(|raw| raw.parse().expect("Couldn't parse integer"))
        .collect::<Vec<u64>>();

    let card_lines = input
        .lines()
        .filter(|line| !line.is_empty())
        .skip(1)
        .collect::<Vec<&str>>();

    let mut cards = card_lines
        .chunks(5)
        .map(|batch| BingoCard::from_lines(batch))
        .collect::<Vec<BingoCard>>();

    for number in numbers.iter() {
        for card in cards.iter_mut() {
            if let Some(score) = card.mark_number(number) {
                return Response::ok((score * number).to_string());
            }
        }
    }

    Response::error("Didn't find an answer", 500)
}

async fn part2(req: worker::Request, ctx: worker::RouteContext<()>) -> Result<Response> {
    let input = utils::get_input(&req, &ctx, "4").await;
    let numbers = input
        .lines()
        .nth(0)
        .expect("Couldn't read input")
        .split(",")
        .map(|raw| raw.parse().expect("Couldn't parse integer"))
        .collect::<Vec<u64>>();

    let card_lines = input
        .lines()
        .filter(|line| !line.is_empty())
        .skip(1)
        .collect::<Vec<&str>>();

    let mut cards = card_lines
        .chunks(5)
        .map(|batch| BingoCard::from_lines(batch))
        .collect::<Vec<BingoCard>>();

    let final_score = cards
        .iter_mut()
        .map(|card| {
            numbers
                .iter()
                .enumerate()
                .map(|(i, number)| card.mark_number(number).map(|score| (i, score * number)))
                .skip_while(|option| option.is_none())
                .nth(0)
                .expect("Bingo card never completed")
                .expect("Something went wrong")
        })
        .max_by(|x, y| x.0.cmp(&y.0))
        .expect("No final winner, I guess")
        .1;

    Response::ok(final_score.to_string())
}

struct BingoCard {
    rows: Vec<HashSet<u64>>,
    columns: Vec<HashSet<u64>>,
}

impl BingoCard {
    fn from_lines(lines: &[&str]) -> Self {
        let grid = lines
            .iter()
            .map(|line| {
                line.split_whitespace()
                    .map(|raw| raw.parse().expect("Couldn't parse integer"))
                    .collect()
            })
            .collect::<Vec<Vec<u64>>>();

        let mut rows = vec![HashSet::new(); 5];
        let mut columns = vec![HashSet::new(); 5];

        for (i, row) in grid.iter().enumerate() {
            for (j, value) in row.iter().enumerate() {
                rows[i].insert(value.clone());
                columns[j].insert(value.clone());
            }
        }

        Self { rows, columns }
    }

    fn mark_number(&mut self, number: &u64) -> Option<u64> {
        let mut has_won = false;
        self.rows.iter_mut().for_each(|row| {
            row.remove(number);
            if row.is_empty() {
                has_won = true;
            }
        });

        self.columns.iter_mut().for_each(|column| {
            column.remove(number);
            if column.is_empty() {
                has_won = true;
            }
        });

        if has_won {
            Some(
                self.rows
                    .iter()
                    .chain(self.columns.iter())
                    .fold(HashSet::new(), |mut remaining: HashSet<u64>, set| {
                        remaining.extend(set);
                        remaining
                    })
                    .iter()
                    .sum(),
            )
        } else {
            None
        }
    }
}
