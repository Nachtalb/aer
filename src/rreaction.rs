use ignore::{
    types::{Types, TypesBuilder},
    Walk, WalkBuilder,
};
use std::path::PathBuf;

const MEDIA_TYPES: &[(&[&str], &[&str])] = &[
    (&["jpg", "jpeg"], &["*.jpg", "*.jpeg"]),
    (&["png"], &["*.png"]),
    (&["gif"], &["*.gif"]),
    (&["mp4"], &["*.mp4"]),
    (&["flac"], &["*.flac"]),
    (&["mp3"], &["*.mp3"]),
    (&["webp"], &["*.webp"]),
    (&["webm"], &["*.webm"]),
    (&["images"], &["includes", "jpg", "png", "gif", "webp"]),
    (&["videos"], &["includes", "mp4", "webm"]),
    (&["animations"], &["includes", "mp4", "webm", "gif"]),
    (&["audio"], &["includes", "mp3"]),
    (&["v8"], &["includes", "webp", "webm"]),
    (&["media"], &["includes", "images", "videos", "audio"]),
];

pub fn default_type_builder() -> TypesBuilder {
    let mut types_builder = TypesBuilder::new();
    for &(names, exts) in MEDIA_TYPES {
        for name in names {
            if exts[0] == "includes" {
                // type_builder.add_def("{name}:include:{ext1},{ext2},...")
                let extensions = &exts[1..].join(",");
                types_builder
                    .add_def(&format!("{}:include:{}", name, extensions))
                    .unwrap();
            } else {
                for ext in exts {
                    types_builder.add(name, ext).unwrap();
                }
            }
        }
    }

    types_builder
}

pub fn get_files(path: PathBuf) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for result in Walk::new(path) {
        if let Ok(entry) = result {
            if entry.path().is_file() {
                files.push(entry.path().to_path_buf());
            }
        }
    }
    files
}

pub fn get_files_by_extensions(path: PathBuf, extensions: &[&str]) -> Vec<PathBuf> {
    let mut types_builder = TypesBuilder::new();
    for ext in extensions {
        types_builder.add(*ext, &format!("*.{}", ext)).unwrap();
        types_builder.select(&ext);
    }
    let types = types_builder.build().unwrap();

    get_files_with_filter(path, types)
}

pub fn get_files_with_filter(path: PathBuf, extensions: Types) -> Vec<PathBuf> {
    let mut files = Vec::new();

    for result in WalkBuilder::new(path).types(extensions).build() {
        if let Ok(entry) = result {
            if entry.path().is_file() {
                files.push(entry.path().to_path_buf());
            }
        }
    }

    files
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_files() {
        let files = get_files(PathBuf::from("test_data"));
        assert_eq!(files.len(), 7);
    }

    #[test]
    fn test_get_files_by_extensions() {
        let files = get_files_by_extensions(PathBuf::from("test_data"), &["gif"]);
        assert_eq!(files.len(), 1);
    }

    #[test]
    fn test_get_files_with_filter() {
        let mut types_builder = TypesBuilder::new();
        types_builder.add("gif", "*.gif").unwrap();
        types_builder.select("gif");
        let types = types_builder.build().unwrap();

        let files = get_files_with_filter(PathBuf::from("test_data"), types);
        assert_eq!(files.len(), 1);
    }

    #[test]
    fn test_default_type_builder() {
        let types = default_type_builder().select("animations").build().unwrap();

        let files = get_files_with_filter(PathBuf::from("test_data"), types);
        assert_eq!(files.len(), 2);
    }
}
