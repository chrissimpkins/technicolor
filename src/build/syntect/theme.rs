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
use std::path::PathBuf;

use syntect::dumps::from_binary;
use syntect::highlighting::ThemeSet;

use crate::errors::TCResult;
use crate::paths::BUILTIN_THEMES_DIR;
// ======================================
//
// Theme sets from text definition files
//
// ======================================
pub fn build_themeset_from_directory(dir: &str) -> TCResult<ThemeSet> {
    let mut ts = ThemeSet::new();
    ts.add_from_folder(dir)?;
    Ok(ts)
}

pub fn build_technicolor_themeset() -> ThemeSet {
    from_binary(include_bytes!("../../../assets/themes.pack"))
}

pub fn build_themeset_by_names<'a, I>(names: I) -> TCResult<ThemeSet>
where
    I: IntoIterator<Item = &'a &'a str>,
{
    // builds from builtin themes defined in the technicolor project
    build_themeset_by_names_from_directory(names, BUILTIN_THEMES_DIR)
}

pub fn build_themeset_by_names_from_directory<'a, I>(names: I, dir: &str) -> TCResult<ThemeSet>
where
    I: IntoIterator<Item = &'a &'a str>,
{
    let mut ts = ThemeSet::new();
    for name in names.into_iter() {
        let mut filepath = PathBuf::new();
        filepath.push(dir);
        filepath.push(name);
        filepath.set_extension("tmTheme");
        let theme = ThemeSet::get_theme(filepath)?;
        ts.themes.insert((*name).to_string(), theme);
    }
    Ok(ts)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::paths::BUILTIN_THEMES_DIR;
    use std::collections::HashMap;

    #[test]
    fn test_build_themeset_from_directory() {
        let ts = build_themeset_from_directory(BUILTIN_THEMES_DIR).unwrap();
        let all_themes: Vec<&str> = ts.themes.keys().map(|x| &**x).collect();
        assert!(all_themes.contains(&"Dracula"));
    }

    #[test]
    fn test_build_themeset_from_directory_fail_bad_dirpath() {
        let ts = build_themeset_from_directory("bogusdir");
        assert!(ts.is_err());
    }

    #[test]
    fn test_build_technicolor_themeset() {
        let ts = build_technicolor_themeset();
        let all_themes: Vec<&str> = ts.themes.keys().map(|x| &**x).collect();
        assert!(all_themes.contains(&"Dracula"));
    }

    #[test]
    fn test_build_themeset_from_directory_by_names() {
        let test_theme_names = vec![&"Dracula", &"Ayu-Light"];
        let ts =
            build_themeset_by_names_from_directory(test_theme_names, BUILTIN_THEMES_DIR).unwrap();
        let all_themes: Vec<&str> = ts.themes.keys().map(|x| &**x).collect();
        assert!(all_themes.contains(&"Dracula")); // expected
        assert!(all_themes.contains(&"Ayu-Light")); // expected
        assert!(!all_themes.contains(&"Material")); // should not be present
    }

    #[test]
    fn test_build_themeset_by_names_with_vector() {
        let test_theme_names = vec![&"Dracula", &"Ayu-Light"];
        let ts = build_themeset_by_names(test_theme_names).unwrap();
        let all_themes: Vec<&str> = ts.themes.keys().map(|x| &**x).collect();
        assert!(all_themes.contains(&"Dracula")); // expected
        assert!(all_themes.contains(&"Ayu-Light")); // expected
        assert!(!all_themes.contains(&"Material")); // should not be present
    }

    #[test]
    fn test_build_themeset_by_names_with_array() {
        let test_theme_names = ["Dracula", "Ayu-Light"];
        let ts = build_themeset_by_names(&test_theme_names).unwrap();
        let all_themes: Vec<&str> = ts.themes.keys().map(|x| &**x).collect();
        assert!(all_themes.contains(&"Dracula")); // expected
        assert!(all_themes.contains(&"Ayu-Light")); // expected
        assert!(!all_themes.contains(&"Material")); // should not be present
    }

    #[test]
    fn test_build_themeset_by_names_with_hashmap() {
        let mut test_theme_names = HashMap::new();
        test_theme_names.insert("Dracula", 1);
        test_theme_names.insert("Ayu-Light", 2);
        let ts = build_themeset_by_names(test_theme_names.keys()).unwrap();
        let all_themes: Vec<&str> = ts.themes.keys().map(|x| &**x).collect();
        assert!(all_themes.contains(&"Dracula")); // expected
        assert!(all_themes.contains(&"Ayu-Light")); // expected
        assert!(!all_themes.contains(&"Material")); // should not be present
    }

    #[test]
    fn test_build_themeset_by_names_with_vector_fail_bad_name() {
        let test_theme_names = vec![&"Bogus"];
        let ts = build_themeset_by_names(test_theme_names);
        assert!(ts.is_err());
    }
}
