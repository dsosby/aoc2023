# Day 03

## Part 1

> Off by one errors... off by one errors for days.

I guess that we'd be attaching those symbols to each part number so I coded the
structures to keep those symbols around. I thought this would be sufficient for
part 2. I wanted to scan numbers instead of symbols so that I didn't have to
deal with de-duplicating part numbers.

## Part 2

Wasn't sufficient. Lol. The locations of those '*' need to be kept around as well.

I'll have to come back to this later today, but one general idea will be to:

 - Store the coordinates of each symbol alongside it using a PartSymbol struct
 - Filter all products that have a gear symbol
 - Group by PartSymbol
 - Filter groups to those with 2 parts
 - Map to calculated gear ratio
 - Sum

## After Thought

This probably could have been done like water islands instead, building out a
set of graphs.

## Part 2 - DONE

HOLY CRAP that was annoying. It would have been faster to re-write this. I
figured something like "group_by" would be good enough and easy enough to just
trumble along.

First - I wanted to quickly verify that there weren't weird gear pairs with
shared gears. That was satisfactory so continued with the approach I thought of
this morning.

Borrow checker hell - I'm fairly used to it at this point, but it's still
annoying sometimes. F# would be trivial here but I picked Rust, so let's ride.

However, I got the wrong solution once I ran it. Hmm...

Lots of debugging later, I finally arrived that `group_by` was not working as I
expected. Some groups would work, but not _all_ of them. I stumbled on this
Github issue titled [group_by is
weird](https://github.com/rust-itertools/itertools/issues/374) and after looking
at the example it hit me like a ton of bricks -- that's my problem!

Re-reading the docs, `group_by` only groups on _consecutive_ groupings. So I can
end up with multiple groupings of the same key. This, of course, breaks
everything.

It seems that what I wanted was `into_group_map` which limits keys to a single
grouped instance. Once that was in place, it worked fine.