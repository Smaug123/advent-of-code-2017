# advent-of-code-2017
Solutions to the 2017 Advent of Code (https://adventofcode.com/2017/), in Rust.
Trying to strike a balance between performance and ease-of-writing, erring on the side of performance where necessary.

Simple to use: just `cargo test --release` or `cargo test -p day_1`, for example.
Some crates have Criterion benchmarks: `cargo bench`, or `cargo bench -p day_1`, for example.

I'm certainly no expert in Rust; don't assume I've done anything in a sane way.

# Day-specific notes

## Day 3

This one I put quite a lot of time and effort into choosing a fast solution.
That means, for example, a constant-time answer to part 1, and a cache-friendly implementation for part 2.
This came at the cost of a lot of code complexity!

## Day 7

This was the first recursive data structure I've attempted in Rust.
After fighting the borrow checker repeatedly (and grudgingly coming to admit that while I knew what I wanted to do was safe, there was no possible way even in principle that I could demonstrate it to Rust without some kind of dependent types), I ended up going for a slotmap.
It felt a bit odd, given that I always think of creating the slotmap as the easy part of constructing a tree from a list of edges; but that's probably just my inner garbage-collector speaking.

## Day 9

I attempted this one chronologically after Day 7, so I was already primed to consider using a slotmap.
However, this time, I arranged things from the very beginning on the assumption that I would be using a catamorphism for all queries against my tree structure.
That meant I was psychologically happy to make the internals weird, because the user is only ever going to use the cata.

## Day 12

This one I didn't really bother reifying into a "proper" data structure, because I already knew what tasks were involved in Part 1 and Part 2.

## Day 13

I know there's a proper answer to this question, one which uses the Chinese remainder theorem to get a solution in time linear in the number of constraints.
I honestly just couldn't be bothered, so I put in a slight optimisation (namely putting in the more effective "can I rule out this number" checks first) but just left it at that.

## Day 18

Actually really enjoyed this one!
There's probably lots and lots of scope for a nice design which actually reifies the channel between the machines, but I didn't start thinking about that.
