# Day 03

## Part 1

> Off by one errors... off by one errors for days.

I guess that we'd be attaching those symbols to each part number so I coded the structures to keep those symbols around. I thought this would be sufficient for part 2. I wanted to scan numbers instead of symbols so that I didn't have to deal with de-duplicating part numbers.

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

This probably could have been done like water islands instead, building out a set of graphs.