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
                    for s in line.split_whitespace() {
                        let diff = levenshtein_distance(&target.to_string(), &s.to_string());
                        if diff < diff_limit {
                            result.push(hit::Hit::new(
                                String::from(s),
                                file.path().display().to_string(),
                                i + 1,
                            ))
                        }
                    }
                }
            }
        }
    }
    Ok(result)
}

fn levenshtein_distance(target: &String, other: &String) -> usize {
    let mut matrix = vec![vec![0_usize; target.len() + 1]; other.len() + 1];
    let target_len = target.chars().count();
    let other_len = other.chars().count();
    // Empty string sub-problems can just be solved by inserting i characters
    for i in 0..=other_len {
        matrix[i][0] = i;
    }

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
        assert_eq!(
            levenshtein_distance(&"Hello".to_string(), &"Hello".to_string()),
            0
        );
        assert_eq!(
            levenshtein_distance(&"Helo".to_string(), &"Hello".to_string()),
            1
        );
        assert_eq!(
            levenshtein_distance(&"ello".to_string(), &"Hello".to_string()),
            1
        );
        assert_eq!(
            levenshtein_distance(&"Heo".to_string(), &"Hello".to_string()),
            2
        );
    }

    #[test]
    fn complex_distance() {
        assert_eq!(
            levenshtein_distance(&String::from("kitten"), &String::from("sitting")),
            3
        );
        assert_eq!(
            levenshtein_distance(&String::from("sunday"), &String::from("saturday")),
            3
        );
        assert_eq!(
            levenshtein_distance(&String::from("manual"), &String::from("=")),
            6
        );
    }
    #[test]
    fn edgecases() {
        assert_eq!(
            levenshtein_distance(&String::from(""), &String::from("")),
            0
        );
        assert_eq!(
            levenshtein_distance(&String::from("Apple"), &String::from("")),
            5
        );
        assert_eq!(
            levenshtein_distance(&String::from(""), &String::from("Apple")),
            5
        );
        assert_eq!(
            levenshtein_distance(&String::from("a"), &String::from("a")),
            0
        );
        assert_eq!(
            levenshtein_distance(&String::from(""), &String::from("a")),
            1
        );
        assert_eq!(
            levenshtein_distance(&String::from("a"), &String::from("")),
            1
        );
        assert_eq!(
            levenshtein_distance(&String::from("A"), &String::from("Apple")),
            4
        );
    }
    #[test]
    fn test_unicode(){
        assert_eq!(
            levenshtein_distance(&String::from("😀"), &String::from("😀")),
            0
        );
        assert_eq!(
            levenshtein_distance(&String::from("😀"), &String::from("😀😁")),
            1
        );
        assert_eq!(
            levenshtein_distance(&String::from("😀🙂"), &String::from("😀😁🙂")),
            1
        );
    }
}
