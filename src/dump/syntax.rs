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
use syntect::parsing::SyntaxSet;

use crate::build::syntax::build_syntaxset_by_names;
use crate::errors::TCResult;

pub fn dump_syntaxset_to_binary(ss: &SyntaxSet, filepath: &str) -> TCResult<()> {
    match dump_to_file(ss, filepath) {
        Ok(_n) => Ok(()),
        Err(err) => Err(err),
    }
}

pub fn dump_syntaxset_to_binary_by_names<'a, T>(
    names: T,
    filepath: &str,
    lines_include_newline: bool,
) -> TCResult<()>
where
    T: IntoIterator<Item = &'a &'a str>,
{
    match build_syntaxset_by_names(names, lines_include_newline) {
        Ok(n) => {
            dump_syntaxset_to_binary(&n, filepath)?;
            Ok(())
        }
        Err(err) => Err(err),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fs;

    use tempfile::tempdir;

    use syntect::dumps::from_dump_file;
    use syntect::parsing::SyntaxSet;

    use super::*;
    use crate::build;

    #[test]
    fn test_dump_syntaxset_to_binary() {
        let tmpdir = tempdir().unwrap();
        let file_path = tmpdir.path().join("syntaxes.bin");
        let ss = build::syntax::build_all_as_syntect_syntaxset_with_newlines();
        dump_syntaxset_to_binary(&ss, &file_path.to_str().unwrap()).unwrap();
        let ss_in: SyntaxSet = from_dump_file(&file_path).unwrap();
        assert!(file_path.is_file());
        assert_eq!(&ss_in.find_syntax_by_extension("ini").unwrap().name, "INI");

        // clean up file
        fs::remove_file(&file_path).unwrap();

        // run again newline parameter = false
        let ss = build::syntax::build_all_as_syntect_syntaxset_with_newlines();
        dump_syntaxset_to_binary(&ss, &file_path.to_str().unwrap()).unwrap();
        let ss_in: SyntaxSet = from_dump_file(&file_path).unwrap();
        assert!(file_path.is_file());
        assert_eq!(&ss_in.find_syntax_by_extension("ini").unwrap().name, "INI");
        // cleanup
        tmpdir.close().unwrap();
    }

    #[test]
    fn test_dump_syntaxset_to_binary_with_names_vector() {
        let tmpdir = tempdir().unwrap();
        let file_path = tmpdir.path().join("syntaxes.bin");
        let names = vec![&"INI", &"Kotlin"];
        dump_syntaxset_to_binary_by_names(names, &file_path.to_str().unwrap(), true).unwrap();
        let ss_in: SyntaxSet = from_dump_file(&file_path).unwrap();
        assert!(file_path.is_file());
        assert_eq!(&ss_in.find_syntax_by_extension("ini").unwrap().name, "INI");
        assert_eq!(
            &ss_in.find_syntax_by_extension("kt").unwrap().name,
            "Kotlin"
        );
        assert!(&ss_in.find_syntax_by_extension("swift").is_none());

        // clean up file
        fs::remove_file(&file_path).unwrap();

        // run again with newline parameter = false
        let names = vec![&"INI", &"Kotlin"];
        dump_syntaxset_to_binary_by_names(names, &file_path.to_str().unwrap(), false).unwrap();
        let ss_in: SyntaxSet = from_dump_file(&file_path).unwrap();
        assert!(file_path.is_file());
        assert_eq!(&ss_in.find_syntax_by_extension("ini").unwrap().name, "INI");
        assert_eq!(
            &ss_in.find_syntax_by_extension("kt").unwrap().name,
            "Kotlin"
        );
        assert!(&ss_in.find_syntax_by_extension("swift").is_none());

        // cleanup
        tmpdir.close().unwrap();
    }

    #[test]
    fn test_dump_syntaxset_to_binary_with_names_array() {
        let tmpdir = tempdir().unwrap();
        let file_path = tmpdir.path().join("syntaxes.bin");
        let names = ["INI", "Kotlin"];
        dump_syntaxset_to_binary_by_names(&names, &file_path.to_str().unwrap(), true).unwrap();
        let ss_in: SyntaxSet = from_dump_file(&file_path).unwrap();
        assert!(file_path.is_file());
        assert_eq!(&ss_in.find_syntax_by_extension("ini").unwrap().name, "INI");
        assert_eq!(
            &ss_in.find_syntax_by_extension("kt").unwrap().name,
            "Kotlin"
        );
        assert!(&ss_in.find_syntax_by_extension("swift").is_none());
        assert!(&ss_in.find_syntax_by_extension("ts").is_none());

        // clean up file
        fs::remove_file(&file_path).unwrap();

        // run again with newline parameter = false
        dump_syntaxset_to_binary_by_names(&names, &file_path.to_str().unwrap(), false).unwrap();
        let ss_in: SyntaxSet = from_dump_file(&file_path).unwrap();
        assert!(file_path.is_file());
        assert_eq!(&ss_in.find_syntax_by_extension("ini").unwrap().name, "INI");
        assert_eq!(
            &ss_in.find_syntax_by_extension("kt").unwrap().name,
            "Kotlin"
        );
        assert!(&ss_in.find_syntax_by_extension("swift").is_none());
        assert!(&ss_in.find_syntax_by_extension("ts").is_none());

        // cleanup
        tmpdir.close().unwrap();
    }

    #[test]
    fn test_dump_syntaxset_to_binary_with_names_hashmap() {
        let mut names = HashMap::new();
        names.insert("INI", 1);
        names.insert("Kotlin", 2);

        let tmpdir = tempdir().unwrap();
        let file_path = tmpdir.path().join("syntaxes.bin");
        dump_syntaxset_to_binary_by_names(names.keys(), file_path.to_str().unwrap(), true).unwrap();
        let ss_in: SyntaxSet = from_dump_file(&file_path).unwrap();
        assert!(file_path.is_file());
        assert_eq!(&ss_in.find_syntax_by_extension("ini").unwrap().name, "INI");
        assert_eq!(
            &ss_in.find_syntax_by_extension("kt").unwrap().name,
            "Kotlin"
        );
        assert!(&ss_in.find_syntax_by_extension("swift").is_none());
        assert!(&ss_in.find_syntax_by_extension("ts").is_none());

        // clean up file
        fs::remove_file(&file_path).unwrap();

        // run again with newline parameter = false
        dump_syntaxset_to_binary_by_names(names.keys(), file_path.to_str().unwrap(), false)
            .unwrap();
        let ss_in: SyntaxSet = from_dump_file(&file_path).unwrap();
        assert!(file_path.is_file());
        assert_eq!(&ss_in.find_syntax_by_extension("ini").unwrap().name, "INI");
        assert_eq!(
            &ss_in.find_syntax_by_extension("kt").unwrap().name,
            "Kotlin"
        );
        assert!(&ss_in.find_syntax_by_extension("swift").is_none());
        assert!(&ss_in.find_syntax_by_extension("ts").is_none());

        // cleanup
        tmpdir.close().unwrap();
    }

    #[test]
    fn test_dump_syntaxset_to_binary_with_names_array_fail_bad_name() {
        let tmpdir = tempdir().unwrap();
        let file_path = tmpdir.path().join("syntaxes.bin");
        let names = ["Bogus", "Kotlin"];
        let res = dump_syntaxset_to_binary_by_names(&names, &file_path.to_str().unwrap(), true);
        assert!(res.is_err());
        assert_eq!(file_path.is_file(), false);

        // repeat with newline parameter = false
        let res = dump_syntaxset_to_binary_by_names(&names, &file_path.to_str().unwrap(), false);
        assert!(res.is_err());
        assert_eq!(file_path.is_file(), false);

        // cleanup
        tmpdir.close().unwrap();
    }
}
