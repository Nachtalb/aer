use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

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

pub fn fast_hash_str(input: &str) -> String {
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    hasher.finish().to_string()
}

pub fn fast_hash_str_with_salt(input: &str, salt: &str) -> String {
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    salt.hash(&mut hasher);
    hasher.finish().to_string()
}

pub fn generate_random_string(len: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fast_hash_str() {
        fast_hash_str("hello world");
    }

    #[test]
    fn test_generate_random_string() {
        let random = generate_random_string(10);
        assert_eq!(random.len(), 10);
    }

    #[test]
    fn test_fast_hash_str_with_salt() {
        fast_hash_str_with_salt("hello world", "salt");
    }

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
}
