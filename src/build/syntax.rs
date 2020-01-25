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
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use syntect::dumps::from_binary;
use syntect::parsing::{SyntaxDefinition, SyntaxSet, SyntaxSetBuilder};
use syntect::LoadingError;

use crate::errors::{TCError, TCResult};
use crate::paths::BUILTIN_SYNTAXES_DIR;

// ======================================
//
// Syntax sets from text definition files
//
// ======================================

pub fn build_as_syntect_syntaxset_from_directory(
    dir: &str,
    lines_include_newline: bool,
) -> TCResult<SyntaxSet> {
    let mut ssb = SyntaxSetBuilder::new();
    // add Plain Text syntax to all syntax sets by default
    ssb.add_plain_text_syntax();
    match ssb.add_from_folder(dir, lines_include_newline) {
        Ok(_) => Ok(ssb.build()),
        Err(err) => Err(TCError::from(err)),
    }
}

pub fn build_all_as_syntect_syntaxset_with_newlines() -> SyntaxSet {
    from_binary(include_bytes!("../../assets/syntaxes-nl.pack"))
}

pub fn build_all_as_syntect_syntaxset_without_newlines() -> SyntaxSet {
    from_binary(include_bytes!("../../assets/syntaxes-nonl.pack"))
}

pub fn build_syntaxset_by_names<'a, T>(names: T, lines_include_newline: bool) -> TCResult<SyntaxSet>
where
    T: IntoIterator<Item = &'a &'a str>,
{
    // builds from builtin syntaxes defined in the technicolor project
    build_syntaxset_by_names_from_directory(names, BUILTIN_SYNTAXES_DIR, lines_include_newline)
}

pub fn build_syntaxset_by_names_from_directory<'a, T>(
    names: T,
    dir: &str,
    lines_include_newline: bool,
) -> TCResult<SyntaxSet>
where
    T: IntoIterator<Item = &'a &'a str>,
{
    let mut ssb = SyntaxSetBuilder::new();
    // add Plain Text syntax to all syntax sets by default
    ssb.add_plain_text_syntax();
    for name in names.into_iter() {
        let mut filepath = PathBuf::new();
        filepath.push(dir);
        filepath.push(name);
        filepath.set_extension("sublime-syntax");
        match load_syntax_file(filepath.as_path(), lines_include_newline) {
            Ok(syntax) => {
                ssb.add(syntax);
            }
            Err(err) => return Err(err),
        }
    }
    Ok(ssb.build())
}

// prviate load_syntax_file function from syntect library
// transition to public here
pub fn load_syntax_file(p: &Path, lines_include_newline: bool) -> TCResult<SyntaxDefinition> {
    let mut f = File::open(p)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;

    Ok(SyntaxDefinition::load_from_str(
        &s,
        lines_include_newline,
        p.file_stem().and_then(|x| x.to_str()),
    )
    .map_err(|e| LoadingError::ParseSyntax(e, Some(format!("{}", p.display()))))?)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::path::Path;

    use super::*;
    use crate::paths::BUILTIN_SYNTAXES_DIR;

    #[test]
    fn test_load_syntax_file() {
        let filepath = Path::new("assets/syntaxes/INI.sublime-syntax");
        let sd = load_syntax_file(filepath, true).unwrap();
        assert_eq!(sd.name, "INI");
        //repeat for newline = false
        let sd = load_syntax_file(filepath, false).unwrap();
        assert_eq!(sd.name, "INI");
    }

    #[test]
    fn test_load_syntax_file_fail_badpath() {
        let filepath = Path::new("assets/syntaxes/Bogus.sublime-syntax");
        let sd = load_syntax_file(filepath, true);
        assert!(sd.is_err());
        //repeat for newline = false
        let sd = load_syntax_file(filepath, false);
        assert!(sd.is_err());
    }

    #[test]
    fn test_build_syntaxset_from_directory() {
        let ss = build_as_syntect_syntaxset_from_directory(BUILTIN_SYNTAXES_DIR, true).unwrap();
        assert_eq!(
            &ss.find_syntax_by_extension("as").unwrap().name,
            "ActionScript"
        );
        assert_eq!(&ss.find_syntax_by_extension("yml").unwrap().name, "YAML");
        assert_eq!(
            &ss.find_syntax_by_extension("txt").unwrap().name,
            "Plain Text"
        );
        assert!(&ss.find_syntax_by_extension("bogus").is_none());
        // repeat for newline = false
        let ss = build_as_syntect_syntaxset_from_directory(BUILTIN_SYNTAXES_DIR, false).unwrap();
        assert_eq!(
            &ss.find_syntax_by_extension("as").unwrap().name,
            "ActionScript"
        );
        assert_eq!(&ss.find_syntax_by_extension("yml").unwrap().name, "YAML");
        assert_eq!(
            &ss.find_syntax_by_extension("txt").unwrap().name,
            "Plain Text"
        );
        assert!(&ss.find_syntax_by_extension("bogus").is_none());
    }

    #[test]
    fn test_build_syntaxset_from_directory_fail_bad_dirpath() {
        let ss = build_as_syntect_syntaxset_from_directory("bogusdir", true);
        assert!(ss.is_err());
        // repeat for newline = false
        let ss = build_as_syntect_syntaxset_from_directory("bogusdir", false);
        assert!(ss.is_err());
    }

    #[test]
    fn test_build_technicolor_syntaxset() {
        let ss = build_all_as_syntect_syntaxset_with_newlines();
        assert_eq!(
            &ss.find_syntax_by_extension("as").unwrap().name,
            "ActionScript"
        );
        assert_eq!(&ss.find_syntax_by_extension("yml").unwrap().name, "YAML");
        assert_eq!(
            &ss.find_syntax_by_extension("txt").unwrap().name,
            "Plain Text"
        );
        assert!(&ss.find_syntax_by_extension("bogus").is_none());
        // repeat for newline = false
        let ss = build_all_as_syntect_syntaxset_with_newlines();
        assert_eq!(
            &ss.find_syntax_by_extension("as").unwrap().name,
            "ActionScript"
        );
        assert_eq!(&ss.find_syntax_by_extension("yml").unwrap().name, "YAML");
        assert_eq!(
            &ss.find_syntax_by_extension("txt").unwrap().name,
            "Plain Text"
        );
        assert!(&ss.find_syntax_by_extension("bogus").is_none());
    }

    #[test]
    fn test_build_syntaxset_by_names_with_vector() {
        let test_syntax_names = vec![&"INI", &"Swift"];
        let ss = build_syntaxset_by_names(test_syntax_names, true).unwrap();
        assert_eq!(&ss.find_syntax_by_extension("ini").unwrap().name, "INI");
        assert_eq!(&ss.find_syntax_by_extension("swift").unwrap().name, "Swift");
        // Plain Text always included by default
        assert_eq!(
            &ss.find_syntax_by_extension("txt").unwrap().name,
            "Plain Text"
        );
        assert!(&ss.find_syntax_by_extension("kt").is_none());
        assert!(&ss.find_syntax_by_extension("bogus").is_none());
        // repeat for newline = false
        let test_syntax_names = vec![&"INI", &"Swift"];
        let ss = build_syntaxset_by_names(test_syntax_names, false).unwrap();
        assert_eq!(&ss.find_syntax_by_extension("ini").unwrap().name, "INI");
        assert_eq!(&ss.find_syntax_by_extension("swift").unwrap().name, "Swift");
        // Plain Text always included by default
        assert_eq!(
            &ss.find_syntax_by_extension("txt").unwrap().name,
            "Plain Text"
        );
        assert!(&ss.find_syntax_by_extension("kt").is_none());
        assert!(&ss.find_syntax_by_extension("bogus").is_none());
    }

    #[test]
    fn test_build_syntaxset_by_names_vector_fail_bad_syntax_name() {
        let test_syntax_names = vec![&"Bogus"];
        let ss = build_syntaxset_by_names(test_syntax_names, true);
        assert!(ss.is_err());
        // repeat for newline = false
        let test_syntax_names = vec![&"Bogus"];
        let ss = build_syntaxset_by_names(test_syntax_names, false);
        assert!(ss.is_err());
    }

    #[test]
    fn test_build_syntaxset_by_names_with_array() {
        let test_syntax_names = ["INI", "Swift"];
        let ss = build_syntaxset_by_names(&test_syntax_names, true).unwrap();
        assert_eq!(&ss.find_syntax_by_extension("ini").unwrap().name, "INI");
        assert_eq!(&ss.find_syntax_by_extension("swift").unwrap().name, "Swift");
        // Plain Text always included by default
        assert_eq!(
            &ss.find_syntax_by_extension("txt").unwrap().name,
            "Plain Text"
        );
        assert!(&ss.find_syntax_by_extension("kt").is_none());
        assert!(&ss.find_syntax_by_extension("bogus").is_none());
        // repeat for newline = false
        let ss = build_syntaxset_by_names(&test_syntax_names, false).unwrap();
        assert_eq!(&ss.find_syntax_by_extension("ini").unwrap().name, "INI");
        assert_eq!(&ss.find_syntax_by_extension("swift").unwrap().name, "Swift");
        // Plain Text always included by default
        assert_eq!(
            &ss.find_syntax_by_extension("txt").unwrap().name,
            "Plain Text"
        );
        assert!(&ss.find_syntax_by_extension("kt").is_none());
        assert!(&ss.find_syntax_by_extension("bogus").is_none());
    }

    #[test]
    fn test_build_syntaxset_by_names_array_fail_bad_syntax_name() {
        let test_syntax_names = ["Bogus"];
        let ss = build_syntaxset_by_names(&test_syntax_names, true);
        assert!(ss.is_err());
        // repeat for newline = false
        let ss = build_syntaxset_by_names(&test_syntax_names, false);
        assert!(ss.is_err());
    }

    #[test]
    fn test_build_syntaxset_by_names_with_hashmap() {
        let mut test_syntax_names = HashMap::new();
        test_syntax_names.insert("INI", 1);
        test_syntax_names.insert("Swift", 2);
        let ss = build_syntaxset_by_names(test_syntax_names.keys(), true).unwrap();
        assert_eq!(&ss.find_syntax_by_extension("ini").unwrap().name, "INI");
        assert_eq!(&ss.find_syntax_by_extension("swift").unwrap().name, "Swift");
        // Plain Text always included by default
        assert_eq!(
            &ss.find_syntax_by_extension("txt").unwrap().name,
            "Plain Text"
        );
        assert!(&ss.find_syntax_by_extension("kt").is_none());
        assert!(&ss.find_syntax_by_extension("bogus").is_none());
        // repeat for newline = false
        let ss = build_syntaxset_by_names(test_syntax_names.keys(), false).unwrap();
        assert_eq!(&ss.find_syntax_by_extension("ini").unwrap().name, "INI");
        assert_eq!(&ss.find_syntax_by_extension("swift").unwrap().name, "Swift");
        // Plain Text always included by default
        assert_eq!(
            &ss.find_syntax_by_extension("txt").unwrap().name,
            "Plain Text"
        );
        assert!(&ss.find_syntax_by_extension("kt").is_none());
        assert!(&ss.find_syntax_by_extension("bogus").is_none());
    }

    #[test]
    fn test_build_syntaxset_by_names_hashmap_fail_bad_syntax_name() {
        let mut test_syntax_names = HashMap::new();
        test_syntax_names.insert("Bogus", 1);
        let ss = build_syntaxset_by_names(test_syntax_names.keys(), true);
        assert!(ss.is_err());
        // repeat for newline = false
        let ss = build_syntaxset_by_names(test_syntax_names.keys(), false);
        assert!(ss.is_err());
    }
}
