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
                        let diff = chars_diff(target, s);
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

fn chars_diff(target: &str, other: &str) -> usize {
    let mut result = 0;
    for it in target.chars().zip(other.chars()) {
        let (target_c, other_c) = it;
        if target_c != other_c {
            result += 1;
        }
    }
    result += target.len().abs_diff(other.len());
    result
}

fn load_file(path: &Path) -> Result<String, std::io::Error> {
    let mut file = std::fs::File::open(path)?;
    let mut result = String::new();
    file.read_to_string(&mut result)?;
    Ok(result)
}

#[cfg(test)]
mod tests{
    use std::path::Path;
    use super::*;

    #[test]
    fn it_works() {
        let results = search_dir(Path::new("./tests"), "Hello").unwrap();
        assert_eq!(format!("{}", results[0]), "./tests/hello.txt:1\tHello");
    }

    #[test]
    fn fuzzier() {
        let results = search_dir(Path::new("./tests"), "Hell").unwrap();
        assert_eq!(format!("{}", results[0]), "./tests/hello.txt:1\tHello");

        let results = search_dir(Path::new("./tests"), "World").unwrap();
        assert_eq!(format!("{}", results[0]), "./tests/hello.txt:1\tWorld!");
    }

    #[test]
    fn cases() {
        let results = search_dir(Path::new("./tests"), "hello").unwrap();
        assert_eq!(format!("{}", results[0]), "./tests/hello.txt:1\tHello");
    }
}
