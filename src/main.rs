use std::{io::Read, path::Path};
mod hit;

const LIMIT: usize = 2;
fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = std::env::args().collect();
    let target = args.get(1).expect("No search item given!");
    let results = search_dir(Path::new("./"), target)?;
    results.iter().for_each(|x| println!("{}", x));

    Ok(())
}

fn search_dir(path: &Path, target: &str) -> Result<Vec<hit::Hit>, std::io::Error> {
    let mut result = Vec::new();
    let paths = std::fs::read_dir(path)?;
    for file in paths {
        let file = file?;
        if file.file_type()?.is_file() {
            if let Ok(data) = load_file(&file.path()) {
                for (i, line) in data.lines().enumerate() {
                    for s in line.split_whitespace() {
                        let diff = levenshtein_distance(&target.to_string(), &s.to_string());
                        if diff < LIMIT {
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
    if target.is_empty() {
        other.len()
    } else if other.is_empty() {
        target.len()
    } else if target.chars().next() == other.chars().next() {
        levenshtein_distance(
            &target.chars().skip(1).collect(),
            &other.chars().skip(1).collect(),
        )
    } else {
        let target_tail = target.chars().skip(1).collect();
        let other_tail = other.chars().skip(1).collect();
        let insert = levenshtein_distance(target, &other_tail);
        let delete = levenshtein_distance(&target_tail, other);
        let replaced = levenshtein_distance(&target_tail, &other_tail);
        1 + insert.min(delete.min(replaced))
    }
}

fn load_file(path: &Path) -> Result<String, std::io::Error> {
    let mut file = std::fs::File::open(path)?;
    let mut result = String::new();
    file.read_to_string(&mut result)?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn it_works() {
        let results = search_dir(Path::new("./tests"), "Hello").unwrap();
        assert_eq!(format!("{}", results[0]), "./tests/hello.txt:1\tHello");
    }

    #[test]
    fn fuzzier() {
        let results = search_dir(Path::new("./tests"), "Hell").unwrap();
        assert_eq!(format!("{}", results[0]), "./tests/hello.txt:1\tHello");

        let results = search_dir(Path::new("./tests"), "World!").unwrap();
        assert_eq!(format!("{}", results[0]), "./tests/hello.txt:1\tWorld!");
    }

    #[test]
    fn cases() {
        let results = search_dir(Path::new("./tests"), "hello").unwrap();
        assert_eq!(format!("{}", results[0]), "./tests/hello.txt:1\tHello");
    }

    #[test]
    fn distance() {
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
}
