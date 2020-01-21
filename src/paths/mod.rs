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

use std::path::Path;

pub const BUILTIN_SYNTAXES_DIR: &str = "assets/syntaxes";
pub const BUILTIN_THEMES_DIR: &str = "assets/themes";

pub fn get_builtin_syntaxes_dirpath() -> &'static Path {
    Path::new(BUILTIN_SYNTAXES_DIR)
}

pub fn get_builtin_themes_dirpath() -> &'static Path {
    Path::new(BUILTIN_THEMES_DIR)
}

#[cfg(test)]
mod tests {
    use crate::paths;

    #[test]
    fn test_get_builtin_syntaxes_dirpath() {
        let testpath = paths::get_builtin_syntaxes_dirpath();
        assert_eq!(testpath.is_absolute(), false);
        assert_eq!(testpath.is_relative(), true);
        assert_eq!(testpath.is_file(), false);
        assert_eq!(testpath.is_dir(), true);
        assert!(testpath.exists(), true);
    }

    #[test]
    fn test_get_builtin_themes_dirpath() {
        let testpath = paths::get_builtin_themes_dirpath();
        assert_eq!(testpath.is_absolute(), false);
        assert_eq!(testpath.is_relative(), true);
        assert_eq!(testpath.is_file(), false);
        assert_eq!(testpath.is_dir(), true);
        assert!(testpath.exists());
    }
}
