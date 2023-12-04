# Day 02

I decided to use a cannon in the water gun fight and try out
[`chumsky`](https://crates.io/crates/chumsky) for a parsing solution. It's
pretty straight-forward - not as easy as `fparsec` but useful.

## Part 1

Really straight-forward with the parsed games. I almost missed the "sum the ids"
and implemented a count, but Maggie stopped by and I was showing her the puzzle
and picked that bit out on that second read.

## Part 2

Really straight-forward to implement each function. I love Rust's support for
lazy iterations. I did get one incorrect solution - I did min-by instead of
max-by, but figured it out really quickly.