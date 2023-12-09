# Day 05

## Part 1

Seems pretty easy here. Thoughts on part 2 might be that the parser is actually
different. I test out a parser quickly - surely there's a simpler way to get
first 3 numbers out of a Vec.

First answer failed - I had a simple bug adding source start/dest start (had
them reversed)

## Part 2

First glance - easy! Just change the parser for seeds into ranges, then flat_map
them, then:

`crash`

Looking at input, that's a lot of big numbers. I'll deal with this later

### Later

I've gone some time again. A quick debugger shows that I'm crashed because I
collected all of the seeds into a Vec to print it out for debugging. I don't
have the memory for that!

The iterator should be fine doing this in constant mem per seed, so I kick off a
release build and get that running for a brute force.

I think this can be simplified though, if I change my `MapTable::map` method to
support a `map_range`, e.g.

```rust fn map_range(&self, input: Range<u128>) -> Vec::<Range<u128>> ```

I'd break that apart into a new set of mapped ranges, and then `flat_map` those
into the `fold`

As I'm starting to code that, the process finishes in a few minutes. The
answer's correct, so I'm moving on.