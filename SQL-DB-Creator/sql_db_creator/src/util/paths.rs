use std::fs;
use std::io::Error;

pub fn sub_paths(dir_path: &str) -> Result<Vec<String>, Error> {

    let paths = match fs::read_dir(dir_path) {
        Ok(paths) => paths,
        Err(e) => {
            return Err(e);
        }
    };

    let mut sub_paths: Vec<String> = Vec::new();

    for path_result in paths {
        let path = match path_result {
            Ok(path) => path,
            Err(e) => {
                return Err(e);
            }
        };
        sub_paths.push(path.path().display().to_string());
    }

    Ok(sub_paths)
}
