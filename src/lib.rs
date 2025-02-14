use std::{collections::HashMap, fmt::Display, hash::Hash};

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

pub fn uncrunch() {}

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
}
