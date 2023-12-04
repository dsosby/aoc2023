# Day 01

For part 1 I did a map/find solution, taking chars and mapping to digits.  This
bit me in part 2 as I continued with that thought, simply mapping "word" tokens
to numbers. The downside I finally realized after fighting with an accepted
solution was that this algorithm consumes tokens. For example:

```
"eightwo" -> '8wo' -> 88
```

instead of, correctly

```
"eightwo" -> ('8', '2') -> 82
```

The trickiness here was the wording of the problem. It only stated that I was to
"find the first and last" - I took it upon myself to transform the line and that
process introduces ambiguity of which character belongs to which token. By
reading the problem _exactly_ as it is, the ambiguity is irrelevant.

After that, I kind of coded it hackily.