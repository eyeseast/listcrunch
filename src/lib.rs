#![doc = include_str!("../README.md")]

use std::{collections::HashMap, fmt::Display, hash::Hash};

/**
`crunch` compresses an interable into a compressed string

## Examples

```
use listcrunch::crunch;

let pages = vec!["595.0x842.0", "595.0x842.0", "595.0x842.0", "595.0x842.0", "595.0x842.0", "595.0x842.0", "595.0x842.0"];
let compressed_string = crunch(&pages);
assert_eq!(compressed_string, "595.0x842.0:0-6");
```
*/
pub fn crunch<T: Hash + Eq + Display>(list: &[T]) -> String {
    // bail early for empty lists
    if list.len() == 0 {
        return String::new();
    }

    let index = build_index(list);

    let mut parts: Vec<String> = Vec::new();

    for (value, nums) in index {
        let mut subparts: Vec<String> = Vec::new();
        let mut run: (i32, i32) = (-1, -1);

        for n in nums {
            if run == (-1, -1) {
                run = (n, n);
            } else {
                if n == run.1 + 1 {
                    run.1 = n;
                } else {
                    subparts.push(end_run(run));
                    // start a new run
                    run = (n, n)
                }
            }
        }

        subparts.push(end_run(run));

        let joined = subparts.join(",");
        parts.push(format!("{}:{}", value, joined));
    }

    parts.join(";")
}

/**
`uncrunch` turns a crunched string into a vector of string slices

## Examples

```
use listcrunch::uncrunch;

let decompressed = uncrunch("595.0x842.0:0-6").unwrap();
assert_eq!(
    decompressed,
    vec!["595.0x842.0", "595.0x842.0", "595.0x842.0", "595.0x842.0", "595.0x842.0", "595.0x842.0", "595.0x842.0"]
);
```
*/
pub fn uncrunch(s: &str) -> Result<Vec<&str>, &'static str> {
    // for an empty string, return an empty vector
    if s.trim().len() == 0 {
        return Ok(Vec::new());
    }

    let mut results: Vec<(u32, &str)> = Vec::new();
    let parts = s.split(";");
    for part in parts {
        let mut subparts = part.split(":");

        // value and specs
        let value = match subparts.next() {
            Some(s) => s,
            None => return Err("Each ';'-delimited region must have exactly one ':'"),
        };
        let specs = match subparts.next() {
            Some(s) => s,
            None => return Err("Each ';'-delimited region must have exactly one ':'"),
        };

        // anything left is an error
        if subparts.count() > 0 {
            return Err("Each ';'-delimited region must have exactly one ':'");
        }

        for spec in specs.split(",") {
            if spec.contains("-") {
                // it's a range
                let mut range = spec.split("-");
                let start = match range.next() {
                    Some(s) => s,
                    None => return Err("Each page range (e.g. 3-5) must have exactly one '-'"),
                };

                let end = match range.next() {
                    Some(s) => s,
                    None => return Err("Each page range (e.g. 3-5) must have exactly one '-'"),
                };

                // shadow and convert to numbers
                let start: i32 = match start.parse() {
                    Ok(v) => v,
                    Err(_) => return Err("Couldn't parse range"),
                };
                let end: i32 = match end.parse() {
                    Ok(v) => v,
                    Err(_) => return Err("Couldn't parse range"),
                };

                for i in start..end + 1 {
                    results.push((i as u32, value));
                }
            } else {
                // just one page
                let spec: u32 = match spec.parse() {
                    Ok(i) => i,
                    Err(_) => return Err("Couldn't parse range"),
                };

                results.push((spec, value));
            }
        }
    }

    // sort and return
    results.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(results.into_iter().map(|(_, value)| value).collect())
}

fn build_index<T: Hash + Eq>(list: &[T]) -> Vec<(&T, Vec<i32>)> {
    let mut index: HashMap<&T, Vec<i32>> = HashMap::new();

    // index all appearances of an item
    for (i, item) in list.iter().enumerate() {
        index.entry(item).or_insert(Vec::new()).push(i as i32);
    }

    // shadow this because we want the new version to sort
    let mut index: Vec<(&T, Vec<i32>)> = index.into_iter().collect();
    index.sort_by(|a, b| a.1[0].cmp(&b.1[0]));
    index
}

fn end_run(run: (i32, i32)) -> String {
    if run == (-1, -1) {
        return String::new();
    }

    if run.0 == run.1 {
        return format!("{}", run.0);
    }

    return format!("{}-{}", run.0, run.1);
}

#[cfg(test)]
mod tests {
    use super::*;

    // crunch
    #[test]
    fn basic_crunch() {
        assert_eq!(crunch(&vec![1, 2, 3]), "1:0;2:1;3:2")
    }

    #[test]
    fn basic_run() {
        assert_eq!(crunch(&vec![1, 1, 1]), "1:0-2")
    }

    #[test]
    fn basic_broken_run() {
        assert_eq!(crunch(&vec![1, 2, 1]), "1:0,2;2:1")
    }

    #[test]
    fn order() {
        assert_eq!(crunch(&vec![2, 1]), "2:0;1:1")
    }

    #[test]
    fn advanced() {
        assert_eq!(
            crunch(&vec![50, 50, 3, 50, 50, 3, 60, 70, 70]),
            "50:0-1,3-4;3:2,5;60:6;70:7-8"
        )
    }

    #[test]
    fn empty_crunch() {
        let empty: Vec<i32> = Vec::new();
        assert_eq!(crunch(&empty), "")
    }

    #[test]
    fn single_item_crunch() {
        assert_eq!(crunch(&vec![77]), "77:0")
    }

    // uncrunch
    #[test]
    fn basic_uncrunch() {
        assert_eq!(uncrunch("1:0;2:1;3:2"), Ok(vec!["1", "2", "3"]))
    }

    #[test]
    fn basic_run_uncrunch() {
        assert_eq!(uncrunch("1:0;2:1;3:2"), Ok(vec!["1", "2", "3"]))
    }

    #[test]
    fn basic_broken_run_uncrunch() {
        assert_eq!(uncrunch("1:0,2;2:1"), Ok(vec!["1", "2", "1"]))
    }

    #[test]
    fn order_uncrunch() {
        assert_eq!(uncrunch("2:0;1:1"), Ok(vec!["2", "1"]))
    }

    #[test]
    fn advanced_uncrunch() {
        assert_eq!(
            uncrunch("50:0-1,3-4;3:2,5;60:6;70:7-8"),
            Ok(vec!["50", "50", "3", "50", "50", "3", "60", "70", "70"])
        )
    }

    #[test]
    fn empty_uncrunch() {
        assert_eq!(uncrunch(""), Ok(vec![]))
    }

    #[test]
    fn single_item_uncrunch() {
        assert_eq!(uncrunch("77:0"), Ok(vec!["77"]))
    }
}
