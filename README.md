# aoc-2021

An attempt to get Advent of Code running in cloudflare workers. The code is written in rust and built/deployed with wrangler.

# Where are these running?

These (at the time of writing at least), are running on my cloudflare dev subdomain. Each day is a single worker with 3 routes (using the day 1 as an example):
- https://aoc2021-day1.jonesetc.workers.dev/ (index that shows the days input)
- https://aoc2021-day1.jonesetc.workers.dev/1/ (returns the solution for part one)
- https://aoc2021-day1.jonesetc.workers.dev/2/ (returns the solution for part two)

# Neat, can I run it with my own input?

In theory, even though I haven't really tried it yet, you should be able to add `?aoc_session=mysessioncookie` to any of those requests and it will evaluate using your session (not captured in any logs, if you trust me). I'm not that great at this, so I wouldn't just go and steal my answers.

# If I were you, and I was likely to forget how to manage this, how would I do that?

First install rust/cargo (likely just use rustup) and then install wrangler with `cargo install wrangler`. make sure that you have filled out `.envrc` with real values and actually use direnv. Or ignore that and set the env vars manually.

If you want to make a new worker for a new day run `./scripts/new-day.sh <day#>`. This will copy the template aoc2021-day0 worker, replace a few number references, add it to the cargo workspace, build it, add the session secret, and publish. This initial version will just return the input for all of the above routes. It's up to you to actually do the programming.

move into the directory of the worker you're interested in and the useful commands are:
```
cargo fmt # format the code
cargo clippy # static analysis for likely bugs
wrangler dev # start local dev server to test code locally
wrangler build # build the actual wasm bundle
wrangler publish # deploy the wasm bundle to cloudflare
```

# You think you're gonna finish this year?

No chance.
