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

fn tree_it<'a>(nodes: &[Node<'a>]) -> Tree<'a> {
    let nodes: HashMap<&'a str, &Node<'a>> = nodes.iter().map(|i| (i.name, i)).collect();
    let mut built: HashMap<&'a str, Box<Tree<'a>>> = HashMap::new();
    let mut stack: Vec<&'a str> = vec![];
    for name in nodes.keys() {
        if !built.contains_key(name) {
            stack.push(name);
            while stack.len() > 0 {
                let node = *nodes.get(stack[stack.len() - 1]).unwrap();
                let required = node.children.iter().filter(|&&i| !built.contains_key(i));
                let old_len = stack.len();
                stack.extend(required);
                if stack.len() == old_len {
                    let tree = Box::new(Tree {
                        name: node.name,
                        weight: node.weight,
                        children: node
                            .children
                            .iter()
                            .map(|&i| built.get(i).unwrap().as_ref())
                            .collect(),
                    });

                    built.insert(node.name, tree);
                    stack.pop();
                }
            }
        }
    }
    panic!("")
}

fn part_1<'a>(nodes: &[Node<'a>]) -> &'a str {
    let tree = tree_it(nodes);
    ""
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
