// Copyright 2020 Christopher Simpkins
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
use syntect::dumps::dump_to_file;
use syntect::highlighting::ThemeSet;

use crate::build::syntect::theme;
use crate::errors::TCResult;

pub fn dump_themeset_to_binary(ts: &ThemeSet, filepath: &str) -> TCResult<()> {
    match dump_to_file(ts, filepath) {
        Ok(_n) => Ok(()),
        Err(err) => Err(err),
    }
}

pub fn dump_themeset_to_binary_by_names<'a, T>(names: T, filepath: &str) -> TCResult<()>
where
    T: IntoIterator<Item = &'a &'a str>,
{
    match theme::build_themeset_with_names(names) {
        Ok(n) => {
            dump_themeset_to_binary(&n, filepath)?;
            Ok(())
        }
        Err(err) => Err(err),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use tempfile::tempdir;

    use syntect::dumps::from_dump_file;
    use syntect::highlighting::ThemeSet;

    use super::*;
    use crate::build::syntect::theme;

    #[test]
    fn test_dump_themeset_to_binary() {
        let tmpdir = tempdir().unwrap();
        let file_path = tmpdir.path().join("themes.bin");
        let ts = theme::build_full_themeset();
        dump_themeset_to_binary(&ts, file_path.to_str().unwrap()).unwrap();
        let ts_in: ThemeSet = from_dump_file(&file_path).unwrap();
        assert!(file_path.is_file());
        let all_themes: Vec<&str> = ts_in.themes.keys().map(|x| &**x).collect();
        assert!(all_themes.contains(&"Dracula"));
        assert!(all_themes.contains(&"Material"));
        assert!(!all_themes.contains(&"Bogus"));
        // cleanup
        tmpdir.close().unwrap();
    }

    #[test]
    fn test_dump_themeset_to_binary_by_names_vector() {
        let tmpdir = tempdir().unwrap();
        let file_path = tmpdir.path().join("themes.bin");
        let names = vec![&"Ayu-Light", &"Dracula"];
        dump_themeset_to_binary_by_names(names, file_path.to_str().unwrap()).unwrap();
        let ts_in: ThemeSet = from_dump_file(&file_path).unwrap();
        assert!(file_path.is_file());
        let all_themes: Vec<&str> = ts_in.themes.keys().map(|x| &**x).collect();
        assert!(all_themes.contains(&"Dracula"));
        assert!(all_themes.contains(&"Ayu-Light"));
        assert!(!all_themes.contains(&"Material"));
        assert!(!all_themes.contains(&"Bogus"));
        // cleanup
        tmpdir.close().unwrap();
    }

    #[test]
    fn test_dump_themeset_to_binary_by_names_array() {
        let tmpdir = tempdir().unwrap();
        let file_path = tmpdir.path().join("themes.bin");
        let names = ["Ayu-Light", "Dracula"];
        dump_themeset_to_binary_by_names(&names, file_path.to_str().unwrap()).unwrap();
        let ts_in: ThemeSet = from_dump_file(&file_path).unwrap();
        assert!(file_path.is_file());
        let all_themes: Vec<&str> = ts_in.themes.keys().map(|x| &**x).collect();
        assert!(all_themes.contains(&"Dracula"));
        assert!(all_themes.contains(&"Ayu-Light"));
        assert!(!all_themes.contains(&"Material"));
        assert!(!all_themes.contains(&"Bogus"));
        // cleanup
        tmpdir.close().unwrap();
    }

    #[test]
    fn test_dump_themeset_to_binary_by_names_hashmap() {
        let tmpdir = tempdir().unwrap();
        let file_path = tmpdir.path().join("themes.bin");
        let mut names = HashMap::new();
        names.insert("Dracula", 1);
        names.insert("Ayu-Light", 2);
        dump_themeset_to_binary_by_names(names.keys(), file_path.to_str().unwrap()).unwrap();
        let ts_in: ThemeSet = from_dump_file(&file_path).unwrap();
        assert!(file_path.is_file());
        let all_themes: Vec<&str> = ts_in.themes.keys().map(|x| &**x).collect();
        assert!(all_themes.contains(&"Dracula"));
        assert!(all_themes.contains(&"Ayu-Light"));
        assert!(!all_themes.contains(&"Material"));
        assert!(!all_themes.contains(&"Bogus"));
        // cleanup
        tmpdir.close().unwrap();
    }

    #[test]
    fn test_dump_themeset_to_binary_by_names_vector_fail_bad_name() {
        let tmpdir = tempdir().unwrap();
        let file_path = tmpdir.path().join("themes.bin");
        let names = vec![&"Bogus", &"Dracula"];
        let res = dump_themeset_to_binary_by_names(names, file_path.to_str().unwrap());
        assert!(res.is_err());
        // cleanup
        tmpdir.close().unwrap();
    }
}
