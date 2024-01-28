use md5;
use std::path::{Path, PathBuf};
use std::{fs::File, io, io::Read};

pub fn relative_to(path: &Path, base: &Path) -> PathBuf {
    path.strip_prefix(base)
        .unwrap_or_else(|_| panic!("Base is not a prefix of path"))
        .to_path_buf()
}

pub fn all_relative_to(paths: &[PathBuf], base: &Path) -> Vec<PathBuf> {
    paths
        .iter()
        .map(|path| relative_to(path, base))
        .collect::<Vec<PathBuf>>()
}

pub fn compute_md5(path: &Path) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(format!("{:x}", md5::compute(buffer)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relative_to() {
        let path = PathBuf::from("/a/b/c");
        let base = PathBuf::from("/a");
        let relative = relative_to(&path, &base);
        assert_eq!(relative, PathBuf::from("b/c"));
    }

    #[test]
    fn test_all_relative_to() {
        let paths = vec![
            PathBuf::from("/a/b/c"),
            PathBuf::from("/a/b/d"),
            PathBuf::from("/a/b/e"),
        ];
        let base = PathBuf::from("/a");
        let relative = all_relative_to(&paths, &base);
        assert_eq!(
            relative,
            vec![
                PathBuf::from("b/c"),
                PathBuf::from("b/d"),
                PathBuf::from("b/e")
            ]
        );
    }

    #[test]
    fn test_compute_md5() {
        let path = PathBuf::from("test_data/file.txt");
        let md5 = compute_md5(&path).unwrap();
        assert_eq!(md5, "d41d8cd98f00b204e9800998ecf8427e"); // Funny how this md5 was auto
                                                             // inserted by Copilot. Really makes
                                                             // you think about leaking data
                                                             // through LLM training sets. 🤔
    }
}
