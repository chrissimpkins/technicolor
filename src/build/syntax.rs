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

use syntect::parsing::{SyntaxDefinition, SyntaxSet, SyntaxSetBuilder};
use syntect::LoadingError;

use crate::paths::BUILTIN_SYNTAXES_DIR;

pub fn build_syntaxset_from_directory(dir: &str) -> Result<SyntaxSet, LoadingError> {
    let mut ssb = SyntaxSetBuilder::new();
    match ssb.add_from_folder(dir, true) {
        Ok(_) => Ok(ssb.build()),
        Err(err) => Err(err),
    }
}

pub fn build_technicolor_syntaxset() -> Result<SyntaxSet, LoadingError> {
    build_syntaxset_from_directory(BUILTIN_SYNTAXES_DIR)
}

pub fn build_syntaxset_by_technicolor_names(names: Vec<&str>) -> Result<SyntaxSet, LoadingError> {
    let mut ssb = SyntaxSetBuilder::new();
    for name in names {
        let mut filepath = PathBuf::new();
        filepath.push(BUILTIN_SYNTAXES_DIR);
        filepath.push(name);
        filepath.set_extension("sublime-syntax");
        match load_syntax_file(filepath.as_path(), true) {
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
pub fn load_syntax_file(
    p: &Path,
    lines_include_newline: bool,
) -> Result<SyntaxDefinition, LoadingError> {
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
    use crate::build::syntax;
    use std::path::Path;

    #[test]
    fn test_load_syntax_file() {
        let filepath = Path::new("assets/syntaxes/INI.sublime-syntax");
        let sd = syntax::load_syntax_file(filepath, true).unwrap();
        assert_eq!(sd.name, "INI");
    }

    #[test]
    fn test_load_syntax_file_fail_badpath() {
        let filepath = Path::new("assets/syntaxes/Bogus.sublime-syntax");
        let sd = syntax::load_syntax_file(filepath, true);
        assert!(sd.is_err());
    }

    #[test]
    fn test_build_technicolor_syntaxset() {
        let ss = syntax::build_technicolor_syntaxset().unwrap();
        assert_eq!(&ss.find_syntax_by_extension("ini").unwrap().name, "INI");
        assert_eq!(&ss.find_syntax_by_extension("kt").unwrap().name, "Kotlin");
        assert!(&ss.find_syntax_by_extension("bogus").is_none());
    }

    #[test]
    fn test_build_syntaxset_by_technicolor_names() {
        let test_syntax_names = vec!["INI", "Swift"];
        let ss = syntax::build_syntaxset_by_technicolor_names(test_syntax_names).unwrap();
        assert_eq!(&ss.find_syntax_by_extension("ini").unwrap().name, "INI");
        assert_eq!(&ss.find_syntax_by_extension("swift").unwrap().name, "Swift");
        assert!(&ss.find_syntax_by_extension("kt").is_none());
        assert!(&ss.find_syntax_by_extension("bogus").is_none());
    }

    #[test]
    fn test_build_syntaxset_by_technicolor_names_fail_bad_syntax_name() {
        let test_syntax_names = vec!["Bogus"];
        let ss = syntax::build_syntaxset_by_technicolor_names(test_syntax_names);
        assert!(ss.is_err());
    }
}