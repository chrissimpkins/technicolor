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
use std::boxed::Box;
use std::error::Error;

use syntect::dumps::dump_to_file;
use syntect::highlighting::ThemeSet;

use crate::build::theme::build_themeset_by_names;

pub fn dump_themeset_to_binary(ts: &ThemeSet, filepath: &str) -> Result<(), Box<dyn Error>> {
    match dump_to_file(ts, filepath) {
        Ok(_n) => Ok(()),
        Err(err) => Err(err),
    }
}

pub fn dump_themeset_to_binary_by_names<'a, I>(
    names: I,
    filepath: &str,
) -> Result<(), Box<dyn Error>>
    where
        I: IntoIterator<Item = &'a &'a str>,
{
    match build_themeset_by_names(names) {
        Ok(n) => {
            dump_themeset_to_binary(&n, filepath)?;
            Ok(())
        }
        Err(err) => Err(Box::new(err)),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use tempfile::tempdir;

    use syntect::dumps::from_dump_file;
    use syntect::highlighting::ThemeSet;

    use crate::build;
    use crate::dump;

    #[test]
    fn test_dump_themeset_to_binary() {
        let tmpdir = tempdir().unwrap();
        let file_path = tmpdir.path().join("themes.bin");
        let ts = build::theme::build_technicolor_themeset().unwrap();
        dump::theme::dump_themeset_to_binary(&ts, file_path.to_str().unwrap()).unwrap();
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
        dump::theme::dump_themeset_to_binary_by_names(names, file_path.to_str().unwrap()).unwrap();
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
        dump::theme::dump_themeset_to_binary_by_names(&names, file_path.to_str().unwrap()).unwrap();
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
        dump::theme::dump_themeset_to_binary_by_names(names.keys(), file_path.to_str().unwrap()).unwrap();
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
        let res = dump::theme::dump_themeset_to_binary_by_names(names, file_path.to_str().unwrap());
        assert!(res.is_err());
        // cleanup
        tmpdir.close().unwrap();
    }
}
