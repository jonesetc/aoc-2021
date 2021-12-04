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
    Response::ok(utils::get_input(&req, &ctx, "3").await)
}

async fn part1(req: worker::Request, ctx: worker::RouteContext<()>) -> Result<Response> {
    let input = utils::get_input(&req, &ctx, "3").await;
    let width = find_width(&input);
    let lines = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let gamma = binary_to_int(&common_bits(lines.iter(), width));
    let epsilon = gamma ^ ((1u64 << width) - 1);

    Response::ok((gamma * epsilon).to_string())
}

async fn part2(req: worker::Request, ctx: worker::RouteContext<()>) -> Result<Response> {
    let input = utils::get_input(&req, &ctx, "3").await;
    let width = find_width(&input);
    let lines = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    // Hate all of the cloning, but started working with vecs in part one and didn't want to spend too much rime optimizing
    let (most_rem, least_rem) = (0..width).fold(
        (lines.clone(), lines.clone()),
        |(most_rem, least_rem), i| {
            let new_most_rem = if most_rem.len() == 1 {
                most_rem
            } else {
                let most_bits = common_bits(most_rem.iter(), width);
                most_rem
                    .iter()
                    .filter(|rem| rem[i] == most_bits[i])
                    .cloned()
                    .collect::<Vec<Vec<char>>>()
            };

            let new_least_rem = if least_rem.len() == 1 {
                least_rem
            } else {
                let least_bits = common_bits(least_rem.iter(), width);
                least_rem
                    .iter()
                    .filter(|rem| rem[i] != least_bits[i])
                    .cloned()
                    .collect::<Vec<Vec<char>>>()
            };

            (new_most_rem, new_least_rem)
        },
    );

    Response::ok((binary_to_int(&most_rem[0]) * binary_to_int(&least_rem[0])).to_string())
}

fn common_bits<'a, I>(lines: I, width: usize) -> Vec<char>
where
    I: Iterator<Item = &'a Vec<char>>,
{
    let (zero_counts, one_counts) = lines.fold(
        (vec![0u64; width], vec![0u64; width]),
        |(mut zero_counts, mut one_counts), bits| {
            for (i, &bit) in bits.iter().enumerate() {
                match bit {
                    '0' => zero_counts[i] += 1,
                    '1' => one_counts[i] += 1,
                    _ => panic!("Unrecognized bit encountered {}", bit),
                }
            }
            (zero_counts, one_counts)
        },
    );

    zero_counts
        .iter()
        .zip(one_counts.iter())
        .map(|(zeroes, ones)| if zeroes > ones { '0' } else { '1' })
        .collect::<Vec<char>>()
}

fn binary_to_int(bits: &Vec<char>) -> u64 {
    u64::from_str_radix(&bits.iter().collect::<String>(), 2).expect("Couldn't parse binary string")
}

fn find_width(input: &str) -> usize {
    input.find("\n").expect("Couldn't determine width.")
}
