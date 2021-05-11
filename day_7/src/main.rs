use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
struct Node<'a> {
    name: &'a str,
    weight: u32,
    children: Vec<&'a str>,
}

struct Tree<'a> {
    children: Vec<&'a Tree<'a>>,
    name: &'a str,
    weight: u32,
}

enum ParserState {
    Name,
    ExpectBracket,
    Weight,
    AnyChildren(usize),
    ParseChild(usize),
    ExpectSpace,
}

fn parse_line(s: &str) -> Node {
    let mut state = ParserState::Name;
    let mut count: usize = 0;
    let mut name_end = 0;
    let mut weight: u32 = 0;
    let mut children: Vec<&str> = vec![];

    for chr in s.chars() {
        match state {
            ParserState::Name => {
                if chr == ' ' {
                    state = ParserState::ExpectBracket;
                    name_end = count;
                }
            }
            ParserState::ExpectBracket => {
                if chr != '(' {
                    panic!("Malformed string! {}", s)
                }
                state = ParserState::Weight;
            }
            ParserState::Weight => {
                if chr == ')' {
                    state = ParserState::AnyChildren(0);
                } else {
                    weight = weight * 10
                        + chr
                            .to_digit(10)
                            .unwrap_or_else(|| panic!("Expected a digit, got {}", chr));
                }
            }
            ParserState::AnyChildren(i) => {
                if i < 3 {
                    state = ParserState::AnyChildren(i + 1);
                } else {
                    state = ParserState::ParseChild(count + 1);
                }
            }
            ParserState::ParseChild(start) => {
                if chr == ',' {
                    children.push(&s[start..count]);
                    state = ParserState::ExpectSpace;
                }
            }
            ParserState::ExpectSpace => {
                if chr != ' ' {
                    panic!("Parse failure, expected space, got {}", chr);
                }
                state = ParserState::ParseChild(count + 1);
            }
        }
        count += 1;
    }

    let name = &s[0..name_end];
    match state {
        ParserState::AnyChildren(0) => {
            // No children
            Node {
                name,
                weight,
                children,
            }
        }
        ParserState::ParseChild(start) => {
            children.push(&s[start..count]);
            Node {
                name,
                weight,
                children,
            }
        }
        _ => {
            panic!("Parse failure");
        }
    }
}

fn input() -> Vec<Node<'static>> {
    let input = include_str!("../input.txt");
    input.lines().map(|l| parse_line(l)).collect::<Vec<Node>>()
}

fn tree_it<'a>(nodes: &[Node<'a>]) -> (Vec<Tree<'a>>, usize) {
    let nodes_by_name: HashMap<&'a str, (&Node<'a>, usize)> = nodes
        .iter()
        .enumerate()
        .map(|(count, i)| (i.name, (i, count)))
        .collect();
    let mut built: Vec<Tree<'a>> = Vec::with_capacity(nodes.len());
    let mut is_set: Vec<bool> = vec![false; nodes.len()];

    let mut stack: Vec<usize> = vec![];
    let mut top_node = None;
    for parent_node_index in 0..nodes.len() {
        if !is_set[parent_node_index] {
            stack.push(parent_node_index);
            while let Some(node_index) = stack.pop() {
                let node = &nodes[node_index];
                let (existing, required): (Vec<_>, Vec<_>) = (*node)
                    .children
                    .iter()
                    .map(|&i| {
                        let (_, child_index) = nodes_by_name.get(i).unwrap();
                        (i, *child_index)
                    })
                    .partition(|&(_, b)| is_set[b]);
                let required = required.iter().map(|&(name, _)| {
                    let (_, child_index) = nodes_by_name.get(name).unwrap();
                    *child_index
                });
                let old_len = stack.len();
                stack.extend(required);
                if stack.len() == old_len {
                    built[node_index] = Tree {
                        name: node.name,
                        weight: node.weight,
                        children: vec![],
                    };
                    built[node_index]
                        .children
                        .extend(existing.iter().map(|&(_, value)| &built[value]));
                    is_set[node_index] = true;
                } else {
                    top_node = Some(node_index);
                }
            }
        }
    }
    (built, top_node.unwrap())
}

fn part_1<'a>(nodes: &[Node<'a>]) -> &'a str {
    let (trees, top_node) = tree_it(nodes);
    trees[top_node].name
}

fn part_2(nodes: &[Node]) -> u32 {
    let tree = tree_it(nodes);
    0
}

fn main() {
    let input = input();
    println!("part 1 => {}", part_1(&input));
    println!("part 2 => {}", part_2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_inputs() -> Vec<Node<'static>> {
        vec![
            Node {
                name: "pbga",
                weight: 66,
                children: vec![],
            },
            Node {
                name: "xhth",
                weight: 57,
                children: vec![],
            },
            Node {
                name: "ebii",
                weight: 61,
                children: vec![],
            },
            Node {
                name: "havc",
                weight: 66,
                children: vec![],
            },
            Node {
                name: "ktlj",
                weight: 57,
                children: vec![],
            },
            Node {
                name: "fwft",
                weight: 72,
                children: vec!["ktlj", "cntj", "xhth"],
            },
            Node {
                name: "qoyq",
                weight: 66,
                children: vec![],
            },
            Node {
                name: "padx",
                weight: 45,
                children: vec!["pbga", "havc", "qoyq"],
            },
            Node {
                name: "tknk",
                weight: 41,
                children: vec!["ugml", "padx", "fwft"],
            },
            Node {
                name: "jptl",
                weight: 61,
                children: vec![],
            },
            Node {
                name: "ugml",
                weight: 68,
                children: vec!["gyxo", "ebii", "jptl"],
            },
            Node {
                name: "gyxo",
                weight: 61,
                children: vec![],
            },
            Node {
                name: "cntj",
                weight: 57,
                children: vec![],
            },
        ]
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            parse_line("vvwrg (51)"),
            Node {
                name: "vvwrg",
                weight: 51,
                children: vec![]
            }
        );
        assert_eq!(
            parse_line("uglvj (99) -> ymfjt, gkpgf"),
            Node {
                name: "uglvj",
                weight: 99,
                children: vec!["ymfjt", "gkpgf"]
            }
        );
    }

    #[test]
    fn part1_known() {
        assert_eq!(part_1(&test_inputs()), "tknk");
    }

    #[test]
    fn part2_known() {
        assert_eq!(part_2(&test_inputs()), 60);
    }

    #[test]
    fn test_day_7() {
        let input = input();
        assert_eq!(part_1(&input), "dgoocsw");
        assert_eq!(part_2(&input), 1275);
    }
}
