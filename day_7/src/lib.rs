pub mod day_7 {
    use std::collections::HashMap;

    #[derive(Debug, PartialEq, Eq)]
    pub struct Node<'a> {
        name: &'a str,
        weight: u32,
        children: Vec<&'a str>,
    }

    struct TreeNode<'a> {
        children: Vec<usize>,
        name: &'a str,
        weight: u32,
    }

    struct Tree<'a> {
        nodes: Vec<TreeNode<'a>>,
        root: usize,
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

    pub fn input() -> Vec<Node<'static>> {
        let input = include_str!("../input.txt");
        input.lines().map(|l| parse_line(l)).collect::<Vec<Node>>()
    }

    fn tree_it<'a>(nodes: &[Node<'a>]) -> Tree<'a> {
        let nodes_by_name: HashMap<&'a str, (&Node<'a>, usize)> = nodes
            .iter()
            .enumerate()
            .map(|(count, i)| (i.name, (i, count)))
            .collect();
        let mut built: Vec<TreeNode<'a>> = nodes
            .iter()
            .map(|n| TreeNode {
                name: n.name,
                weight: n.weight,
                children: vec![],
            })
            .collect();
        let mut is_set: Vec<bool> = vec![false; nodes.len()];

        let mut stack: Vec<usize> = vec![];
        for parent_node_index in 0..nodes.len() {
            if !is_set[parent_node_index] {
                stack.push(parent_node_index);
                while let Some(node_index) = stack.pop() {
                    if is_set[node_index] {
                        continue;
                    }
                    let node = &nodes[node_index];
                    let children = (*node).children.iter().map(|&i| {
                        let (_, child_index) = nodes_by_name.get(i).unwrap();
                        *child_index
                    });
                    built[node_index].children.extend(children);
                    stack.extend(built[node_index].children.iter());
                    is_set[node_index] = true;
                }
            }
        }

        // We'll reuse the is_set array for an "is possibly the root" array.
        // They're all true now.
        for node in built.iter() {
            for child in node.children.iter() {
                is_set[*child] = false;
            }
        }
        let (top_node, _) = is_set.iter().enumerate().find(|(_, v)| **v).unwrap();
        Tree {
            nodes: built,
            root: top_node,
        }
    }

    pub fn part_1<'a>(nodes: &[Node<'a>]) -> &'a str {
        let tree = tree_it(nodes);
        tree.nodes[tree.root].name
    }

    #[derive(Debug)]
    struct BalancedWeight {
        child_weights: u32,
        self_weight: u32,
        child_count: usize,
    }

    fn total_weight(b: &BalancedWeight) -> u32 {
        b.child_weights * (b.child_count as u32) + b.self_weight
    }

    fn weight(tree: &Tree, node: usize) -> Result<BalancedWeight, i32> {
        let node = &tree.nodes[node];
        let children: Vec<_> = node
            .children
            .iter()
            .map(|node| weight(&tree, *node))
            .collect();

        let count = children.len();
        let mut c = children.iter();
        let mut different_child = None;
        if let Some(child) = c.next() {
            match child {
                Err(e) => Err(*e),
                Ok(child) => {
                    for other in c {
                        match other {
                            Err(e) => return Err(*e),
                            Ok(other) => {
                                if total_weight(other) != total_weight(child) {
                                    match different_child {
                                        None => {
                                            different_child = Some(other);
                                        }
                                        Some(different_child) => {
                                            if total_weight(other) == total_weight(different_child)
                                            {
                                                // The bad one is `child`.
                                                return Err(total_weight(other) as i32
                                                    - (child.child_weights
                                                        * child.child_count as u32)
                                                        as i32);
                                            } else {
                                                // The bad one is `other`.
                                                return Err(total_weight(child) as i32
                                                    - (other.child_weights
                                                        * other.child_count as u32)
                                                        as i32);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    match different_child {
                        None => Ok(BalancedWeight {
                            self_weight: node.weight,
                            child_weights: total_weight(child),
                            child_count: count,
                        }),
                        Some(different_child) => {
                            // The bad one is `different_child`.
                            Err(total_weight(child) as i32
                                - (different_child.child_weights
                                    * different_child.child_count as u32)
                                    as i32)
                        }
                    }
                }
            }
        } else {
            Ok(BalancedWeight {
                self_weight: node.weight,
                child_weights: 0,
                child_count: 0,
            })
        }
    }

    pub fn part_2(nodes: &[Node]) -> i32 {
        let tree = tree_it(nodes);
        let result = weight(&tree, tree.root);
        match result {
            Err(e) => e,
            Ok(w) => panic!("Expected unbalanced tree, got {:?}", w),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::day_7::*;

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
