# Day 06

## Part 1

This seems like an algebra problem. I don't remember much algebra, but I know
enough to get the inequality set up and remember it something to do with the
quadratic equation. ChatGPT did help me out here.

One stumbling block was solving for the next value -- I found an unstable
feature for f64::next_up() and f64::next_down() that works, but only in the
nightly channel. Once I got the answer, I did try it with a "very small number"
and got the same answer at least.

## Part 2

Oooohhh....that's a big number.

Time to go look up some big float crates?

## DOH

Lol... my "kerning" was bad. That wasn't a big number. `f64` handles it fine. Phew.