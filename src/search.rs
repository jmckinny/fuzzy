use crate::hit;
use std::{io::Read, path::Path};

pub fn search_dir(
    path: &Path,
    target: &str,
    diff_limit: usize,
) -> Result<Vec<hit::Hit>, std::io::Error> {
    let mut result = Vec::new();
    let paths = std::fs::read_dir(path)?;
    for file in paths {
        let file = file?;
        if file.file_type()?.is_file() {
            if let Ok(data) = load_file(&file.path()) {
                for (i, line) in data.lines().enumerate() {
                    let mut valid = line
                        .split_whitespace()
                        .filter(|x| levenshtein_distance(target, x) < diff_limit)
                        .map(|x| {
                            hit::Hit::new(String::from(x), file.path().display().to_string(), i + 1)
                        })
                        .collect();
                    result.append(&mut valid);
                }
            }
        }
    }
    Ok(result)
}

fn levenshtein_distance(target: &str, other: &str) -> usize {
    let mut matrix = vec![vec![0_usize; target.len() + 1]; other.len() + 1];
    let target_len = target.chars().count();
    let other_len = other.chars().count();
    // Empty string sub-problems can just be solved by inserting i characters
    (0..=other_len).for_each(|i| {
        matrix[i][0] = i;
    });

    for j in 0..=target_len {
        matrix[0][j] = j
    }

    for j in 1..=target_len {
        for i in 1..=other_len {
            let substitue_cost =
                if target.chars().nth(j - 1).unwrap() == other.chars().nth(i - 1).unwrap() {
                    0
                } else {
                    1
                };

            matrix[i][j] = min3(
                matrix[i - 1][j] + 1,                  //deletion
                matrix[i][j - 1] + 1,                  //insertion
                matrix[i - 1][j - 1] + substitue_cost, //substitution
            );
        }
    }
    matrix[other_len][target_len]
}

fn min3(x: usize, y: usize, z: usize) -> usize {
    x.min(y.min(z))
}

fn load_file(path: &Path) -> Result<String, std::io::Error> {
    let mut file = std::fs::File::open(path)?;
    let mut result = String::new();
    file.read_to_string(&mut result)?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    const DEFAULT_LIMIT: usize = 2;
    use super::*;
    use std::path::Path;

    #[test]
    fn it_works() {
        let results = search_dir(Path::new("./tests"), "Hello", DEFAULT_LIMIT).unwrap();
        assert_eq!(format!("{}", results[0]), "./tests/hello.txt:1\tHello");
    }

    #[test]
    fn fuzzier() {
        let results = search_dir(Path::new("./tests"), "Hell", DEFAULT_LIMIT).unwrap();
        assert_eq!(format!("{}", results[0]), "./tests/hello.txt:1\tHello");

        let results = search_dir(Path::new("./tests"), "World!", DEFAULT_LIMIT).unwrap();
        assert_eq!(format!("{}", results[0]), "./tests/hello.txt:1\tWorld!");
    }

    #[test]
    fn cases() {
        let results = search_dir(Path::new("./tests"), "hello", DEFAULT_LIMIT).unwrap();
        assert_eq!(format!("{}", results[0]), "./tests/hello.txt:1\tHello");
    }

    #[test]
    fn basic_distance() {
        assert_eq!(levenshtein_distance("Hello", "Hello"), 0);
        assert_eq!(levenshtein_distance("Helo", "Hello"), 1);
        assert_eq!(levenshtein_distance("ello", "Hello"), 1);
        assert_eq!(levenshtein_distance("Heo", "Hello"), 2);
    }

    #[test]
    fn complex_distance() {
        assert_eq!(levenshtein_distance("kitten", "sitting"), 3);
        assert_eq!(levenshtein_distance("sunday", "saturday"), 3);
        assert_eq!(levenshtein_distance("manual", "="), 6);
    }
    #[test]
    fn edgecases() {
        assert_eq!(levenshtein_distance("", ""), 0);
        assert_eq!(levenshtein_distance("Apple", ""), 5);
        assert_eq!(levenshtein_distance("", "Apple"), 5);
        assert_eq!(levenshtein_distance("a", "a"), 0);
        assert_eq!(levenshtein_distance("", "a"), 1);
        assert_eq!(levenshtein_distance("a", ""), 1);
        assert_eq!(levenshtein_distance("A", "Apple"), 4);
    }
    #[test]
    fn test_unicode() {
        assert_eq!(levenshtein_distance("😀", "😀"), 0);
        assert_eq!(levenshtein_distance("😀", "😀😁"), 1);
        assert_eq!(levenshtein_distance("😀🙂", "😀😁🙂"), 1);
    }
}
