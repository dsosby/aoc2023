# Day 04

## Part 1

Seems pretty straight-forward. I could parse each line pretty quickly with
`split`, but why not practice with Chumsky on the easy stuff so that the harder
stuff is easier?

The parser was pretty easy this time, only trip-up I had was expecting a 3-tuple
like `(id, numbers, winning_numbers)` but getting a tree `((id, numbers),
winning_numbers)`. I think I could have used `chain` instead.

## Part 2

First, this is more of a test of reading comprehension. I needed more coffee to
understand that one.

The example spells out the algorithm pretty succintly.

More Rust fun in simply incrementing values on a Vector. I had to look up the
docs on it again to remember how to use `get_mut`. I've done this with `HashMap`
and `BTreeMap` which has a useful `Entry` struct, but that was overkill for this
indexed structur.

My first attempt crashed, but realized quickly it was consuming (lots) more wins
than needed. Whoops, I was using a card's `score` instead of number of winners.
Then it ran fine and produced the right answer.