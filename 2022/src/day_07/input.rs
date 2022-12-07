use std::fs;

use super::Input;

pub fn read() -> Input {
    let file = fs::read_to_string("./src/day_07/input.txt").unwrap();
    let mut paths = [("".to_string(), 0)].into_iter().collect::<Input>();
    let mut cwd = "".to_string();
    for line in file.lines() {
        if let Some(dir) = line.strip_prefix("$ cd ") {
            cwd = match dir {
                "/" => "".to_string(),
                ".." => cwd.rsplit_once('/').map_or("", |(cwd, _)| cwd).to_string(),
                _ => {
                    if cwd.is_empty() {
                        dir.to_string()
                    } else {
                        vec![cwd, dir.to_string()].join("/")
                    }
                }
            };
        } else if line == "$ ls" {
            continue;
        } else {
            let (size, _) = line.split_once(' ').unwrap();
            if size == "dir" {
                continue;
            }
            let size = size.parse::<u32>().unwrap();
            let mut path = cwd.to_string();
            loop {
                paths
                    .entry(path.to_string())
                    .and_modify(|s| *s += size)
                    .or_insert(size);
                if path.is_empty() {
                    break;
                }
                path.drain(path.rfind('/').unwrap_or(0)..);
            }
        }
    }
    paths
}

// #[cfg(test)]
// mod tests {
//     use std::collections::HashMap;

//     use super::*;

//     #[test]
//     fn test_parser() {
//         let input = read();
//         let mut paths = HashMap::new();
//         paths.insert("".to_string(), 48381165);
//         paths.insert("a".to_string(), 94853);
//         paths.insert("d".to_string(), 24933642);
//         paths.insert("a/e".to_string(), 584);

//         assert_eq!(input, paths)
//     }
// }
