pub mod day_9 {

    #[derive(Debug, Eq, PartialEq)]
    pub(crate) struct GroupIndex {
        pub(crate) i: usize,
    }

    #[derive(Debug)]
    pub(crate) struct Group {
        pub(crate) entries: Vec<GroupIndex>,
    }

    #[derive(Debug)]
    pub(crate) enum GroupEntry<'a> {
        Garbage(&'a str),
        Group(Group),
    }

    pub struct Stream<'a> {
        pub(crate) groups: Vec<GroupEntry<'a>>,
        pub(crate) head_group: GroupIndex,
    }

    impl Stream<'_> {
        fn cata_inner<'a, 'b, T>(
            entries: &'b [GroupEntry<'a>],
            head: &GroupIndex,
            depth: usize,
            at_garbage: fn(&'a str) -> T,
            combine: fn(&mut dyn Iterator<Item = T>, usize) -> T,
        ) -> T
        where
            T: 'static,
        {
            let e = &entries[head.i];
            match e {
                GroupEntry::Garbage(s) => at_garbage(s),
                GroupEntry::Group(Group { entries: e }) => {
                    let mut v = e
                        .iter()
                        .map(|i| Stream::cata_inner(entries, i, depth + 1, at_garbage, combine));
                    combine(&mut v, depth)
                }
            }
        }

        pub fn cata<'a, 'b, T>(
            s: &'b Stream,
            at_garbage: fn(&'a str) -> T,
            combine: fn(&mut dyn Iterator<Item = T>, usize) -> T,
        ) -> T
        where
            'b: 'a,
            T: 'static,
        {
            Stream::cata_inner(&s.groups, &s.head_group, 0, at_garbage, combine)
        }
    }

    pub(crate) fn parse<'a>(s: &'a str) -> Stream<'a> {
        let mut iter = s.chars().enumerate();
        match iter.next().unwrap() {
            (_, '{') => {}
            (_, c) => {
                panic!("Group didn't start with open-brace, but '{}'!", c);
            }
        }
        let mut groups: Vec<GroupEntry<'a>> = vec![];
        let mut stack: Vec<Vec<GroupIndex>> = vec![vec![]];

        let mut garbage = None;
        let mut skip = false;

        for (pos, c) in iter {
            if let Some(start) = garbage {
                if skip {
                    skip = false;
                } else {
                    match c {
                        '!' => {
                            skip = true;
                        }
                        '>' => {
                            groups.push(GroupEntry::Garbage(&s[start + 1..pos]));
                            let constructing_group = stack.len() - 1;
                            stack[constructing_group].push(GroupIndex {
                                i: groups.len() - 1,
                            });
                            garbage = None;
                        }
                        _ => {}
                    }
                }
            } else {
                match c {
                    '<' => {
                        // New garbage. Consume up until the next non-cancelled '>'.
                        garbage = Some(pos);
                    }
                    '{' => {
                        // New group. Consume up to the next non-cancelled '}'.
                        stack.push(vec![]);
                    }
                    '}' => {
                        let entries = stack.pop().unwrap();
                        groups.push(GroupEntry::Group(Group { entries }));
                        match stack.last_mut() {
                            None => {}
                            Some(l) => {
                                l.push(GroupIndex {
                                    i: groups.len() - 1,
                                });
                            }
                        }
                    }
                    ',' => {}
                    c => {
                        panic!("Expected a known character, got: {}", c);
                    }
                }
            }
        }
        let max = groups.len() - 1;
        Stream {
            groups,
            head_group: GroupIndex { i: max },
        }
    }

    pub fn input() -> Stream<'static> {
        let input = include_str!("../input.txt");
        parse(input.trim())
    }

    pub fn part_1(numbers: &Stream) -> usize {
        Stream::cata(
            numbers,
            |_| 0,
            |v, depth| {
                let mut sum = depth + 1;
                for i in v {
                    sum += i;
                }
                sum
            },
        )
    }

    pub fn part_2(numbers: &Stream) -> u32 {
        Stream::cata(
            numbers,
            |g| {
                let mut skip = false;
                let mut ans = 0;
                for c in g.chars() {
                    if skip {
                        skip = false;
                    } else {
                        match c {
                            '!' => {
                                skip = true;
                            }
                            _ => {
                                ans += 1;
                            }
                        }
                    }
                }
                ans
            },
            |v, _| {
                let mut sum = 0;
                for i in v {
                    sum += i;
                }
                sum
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::day_9::*;

    #[test]
    fn test_empty_parse() {
        let result = parse("{}");
        assert_eq!(result.groups.len(), 1);
        match &result.groups[0] {
            GroupEntry::Group(Group { entries: e }) => {
                assert_eq!(e.len(), 0);
            }
            e => {
                panic!("Wrong match: {:?}", e);
            }
        }
        assert_eq!(result.head_group, GroupIndex { i: 0 });
    }

    #[test]
    fn test_triple_empty_parse() {
        let result = parse("{{{}}}");
        assert_eq!(result.groups.len(), 3);
        let (zero_count, one_count) = Stream::cata(
            &result,
            |_| panic!("No garbage"),
            |i, _| {
                let (mut zero_count, mut one_count) = (0, 0);
                let mut my_count = 0u32;
                for (z, o) in i {
                    zero_count += z;
                    one_count += o;
                    my_count += 1;
                }
                match my_count {
                    0 => (zero_count + 1, one_count),
                    1 => (zero_count, one_count + 1),
                    l => {
                        panic!("Unexpected count: {}", l);
                    }
                }
            },
        );
        assert_eq!(zero_count, 1);
        assert_eq!(one_count, 2);
    }

    #[test]
    fn part_1_known() {
        assert_eq!(part_1(&parse("{}")), 1);
        assert_eq!(part_1(&parse("{{{}}}")), 6);
        assert_eq!(part_1(&parse("{{},{}}")), 5);
        assert_eq!(part_1(&parse("{{{},{},{{}}}}")), 16);
        assert_eq!(part_1(&parse("{<a>,<a>,<a>,<a>}")), 1);
        assert_eq!(part_1(&parse("{{<ab>},{<ab>},{<ab>},{<ab>}}")), 9);
        assert_eq!(part_1(&parse("{{<!!>},{<!!>},{<!!>},{<!!>}}")), 9);
        assert_eq!(part_1(&parse("{{<a!>},{<a!>},{<a!>},{<ab>}}")), 3);
    }

    #[test]
    fn test_day_9() {
        let input = input();
        assert_eq!(part_1(&input), 16869);
        assert_eq!(part_2(&input), 7284);
    }
}
