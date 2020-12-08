use aoc_runner_derive::{aoc, aoc_generator};
use lazy_static::lazy_static;
use regex::Regex;
use simple_error::bail;
use std::collections::hash_map::HashMap;
use std::collections::hash_set::HashSet;
use std::error::Error;

#[aoc_generator(day7)]
fn parse_input(input: &str) -> Tree {
    let mut tree = Tree::new();

    let rules = input.lines().map(|l| parse_rule(l));

    rules.for_each(|r| tree.insert_rule(r));

    return tree;
}

#[aoc(day7, part1)]
fn count_containers(tree: &Tree) -> usize {
    tree.search_containers("shiny gold").unwrap().len()
}

#[aoc(day7, part2)]
fn count_bags_needed(tree: &Tree) -> usize {
    tree.count_needed_bags("shiny gold")
}

type Rule<'a> = (&'a str, Option<Vec<(usize, String)>>);

struct Bag {
    idx: usize,
    contained_by: Option<Vec<usize>>,
    contains: Option<Vec<(usize, usize)>>,
    description: String,
}

impl Bag {
    pub fn new(idx: usize, description: String) -> Self {
        Bag {
            idx,
            contained_by: None,
            contains: None,
            description,
        }
    }

    pub fn add_child(&mut self, (qty, idx): (usize, usize)) {
        match &mut self.contains {
            Some(v) => v.push((qty, idx)),
            None => self.contains = Some(vec![(qty, idx)]),
        }
    }

    pub fn add_parent(&mut self, idx: usize) {
        match &mut self.contained_by {
            Some(v) => v.push(idx),
            None => self.contained_by = Some(vec![idx]),
        }
    }

    pub fn search_container_bags_recursive(&self, tree: &Tree, set: &mut HashSet<usize>) {
        if let Some(parents) = &self.contained_by {
            parents.iter().for_each(|p| {
                set.insert(*p);
                let parent_node = &tree.nodes[*p];
                parent_node.search_container_bags_recursive(tree, set);
            });
        }
    }

    pub fn count_needed_bags_recursive(&self, tree: &Tree) -> usize {
        if let Some(contained) = &self.contains {
            let mut count = 0;
            for (qty, idx) in contained {
                let bag = &tree.nodes[*idx];
                count += qty + qty * bag.count_needed_bags_recursive(tree);
            }

            return count;
        } else {
            //Is end node, return 0
            return 0;
        }
    }
}

struct Tree {
    nodes: Vec<Bag>,
    len: usize,
    node_index_hash: HashMap<String, usize>,
}

impl Tree {
    pub fn new() -> Self {
        Tree {
            nodes: vec![],
            len: 0,
            node_index_hash: HashMap::new(),
        }
    }

    fn get_or_insert(&mut self, description: &str) -> usize {
        let in_tree = self.node_index_hash.get(&description.to_owned());
        match in_tree {
            Some(v) => *v,
            None => {
                let new_idx = self.len;
                let new_bag = Bag::new(new_idx, description.to_string());
                self.nodes.push(new_bag);
                self.len += 1;
                self.node_index_hash.insert(description.to_owned(), new_idx);
                new_idx
            }
        }
    }

    pub fn get(&self, description: &str) -> Option<&Bag> {
        let idx = self.node_index_hash.get(description)?;
        Some(&self.nodes[*idx])
    }

    pub fn insert_rule(&mut self, rule: Rule) {
        let (container_desc, contained_rules) = rule;

        let container_node_idx = self.get_or_insert(container_desc);

        match contained_rules {
            Some(vec) => {
                for (qty, desc) in vec {
                    // append current_child's parent with container_node_idx
                    let current_child_idx = self.get_or_insert(&desc);
                    let current_child = &mut self.nodes[current_child_idx];
                    current_child.add_parent(container_node_idx);

                    // append child_idx and qty to container_node
                    let container_node = &mut self.nodes[container_node_idx];
                    container_node.add_child((qty, current_child_idx));
                }
            }
            None => (),
        }
    }

    /// Returns a vector of containers that can eventually hold a container
    /// with description @desc
    pub fn search_containers(&self, desc: &str) -> Result<Vec<&Bag>, Box<dyn Error>> {
        let container = self.get(desc);

        if let Some(c) = container {
            let mut container_set = HashSet::new();
            c.search_container_bags_recursive(self, &mut container_set);
            let container_idxs = container_set.iter();
            let containers = container_idxs.map(|idx| &self.nodes[*idx]).collect();
            Ok(containers)
        } else {
            bail!("Bag not found!");
        }
    }

    /// Returns the count of needed bags inside
    pub fn count_needed_bags(&self, desc: &str) -> usize {
        let container = self.get(desc);
        if let Some(c) = container {
            c.count_needed_bags_recursive(self)
        } else {
            unreachable!();
        }
    }
}

fn parse_rule(input: &str) -> Rule {
    lazy_static! {
        static ref RULE_PARSER: Regex = Regex::new(
            r"(?P<description_container>\w+ \w+) bags contain (?P<description_contained>.*).",
        )
        .unwrap();
    }
    let captures = RULE_PARSER.captures(input).unwrap();

    let description_container = captures.name("description_container").unwrap();
    let description_contained = captures.name("description_contained").unwrap();
    if description_contained == "no other bags" {
        return (description_container, None);
    } else {
        let unparsed: Vec<&str> = description_contained.split(",").map(|r| r.trim()).collect();
        let rules: Vec<(usize, String)> = unparsed
            .iter()
            .map(|u| {
                let tokens: Vec<&str> = u.split(" ").collect();
                (tokens[0].parse().unwrap(), tokens[1..3].join(" "))
            })
            .collect();
        return (description_container, Some(rules));
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_INPUT: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
    #[test]
    fn test_search_containers() {
        let tree = parse_input(EXAMPLE_INPUT);
        let count = tree.search_containers("shiny gold").unwrap();
        assert_eq!(count.len(), 4);
    }

    #[test]
    fn test_tree_construction() {
        let tree = parse_input(EXAMPLE_INPUT);
        let node1 = tree.get("light red").unwrap();
        let node2 = tree.get("bright white").unwrap();
        let node3 = tree.get("muted yellow").unwrap();
        let node4 = tree.get("faded blue").unwrap();
        let node5 = tree.get("shiny gold").unwrap();
        assert_eq!(node1.contains, Some(vec![(1, node2.idx), (2, node3.idx)]));
        assert_eq!(node4.contains, None);
        assert_eq!(node5.contained_by, Some(vec![node2.idx, node3.idx]))
    }

    #[test]
    fn test_parse_rule() {
        let s1 = "faded blue bags contain no other bags.";
        let s2 = "bright white bags contain 1 shiny gold bag.";
        let s3 = "light red bags contain 1 bright white bag, 2 muted yellow bags.";

        let rule = parse_rule(&s1);
        assert_eq!(rule, ("faded blue", None));

        let rule2 = parse_rule(&s2);
        assert_eq!(
            rule2,
            ("bright white", Some(vec![(1, "shiny gold".to_string())]))
        );

        let rule3 = parse_rule(&s3);
        assert_eq!(
            rule3,
            (
                "light red",
                Some(vec![
                    (1, "bright white".to_string()),
                    (2, "muted yellow".to_string())
                ])
            )
        );
    }

    #[test]
    fn test_regex() {
        let instruction_parser: Regex = Regex::new(
            r"(?P<description_container>\w+ \w+) bags contain (?P<description_contained>.*).",
        )
        .unwrap();

        let s1 = "bright white bags contain 1 shiny gold bag.";
        let s2 = "light red bags contain 1 bright white bag, 2 muted yellow bags.";
        let s3 = "faded blue bags contain no other bags.";

        let cap1 = instruction_parser.captures(&s1).unwrap();

        assert_eq!(cap1.name("description_container").unwrap(), "bright white");
        assert_eq!(
            cap1.name("description_contained").unwrap(),
            "1 shiny gold bag"
        );

        let cap2 = instruction_parser.captures(&s2).unwrap();

        assert_eq!(cap2.name("description_container").unwrap(), "light red");
        assert_eq!(
            cap2.name("description_contained").unwrap(),
            "1 bright white bag, 2 muted yellow bags"
        );

        let cap3 = instruction_parser.captures(&s3).unwrap();

        assert_eq!(cap3.name("description_container").unwrap(), "faded blue");
        assert_eq!(cap3.name("description_contained").unwrap(), "no other bags");
    }
}
