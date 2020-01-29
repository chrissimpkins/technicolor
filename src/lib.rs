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

//! # Technicolor
//!
//! ## Source Repository
//!
//! GitHub: [chrissimpkins/technicolor](https://github.com/chrissimpkins/technicolor)
//!
//! ## Color Theme Definitions
//!
//! Source repository path: [assets/themes](https://github.com/chrissimpkins/technicolor/tree/master/assets/themes)
//!
//! ## Syntax Definitions
//!
//! Source repository path: [assets/syntaxes](https://github.com/chrissimpkins/technicolor/tree/master/assets/syntaxes)
//!
//! ## About
//! The technicolor library distributes over 130 builtin language syntaxes and over 110 builtin color themes to
//! support syntax highlighting. All [syntaxes](https://github.com/chrissimpkins/technicolor/tree/master/assets/syntaxes)
//! and [themes](https://github.com/chrissimpkins/technicolor/tree/master/assets/themes) included in this project
//! are distrbuted under free licenses. See the headers of the definition files in the source repository
//! for author and license details.
//!
//! Technicolor is tightly integrated with the [syntect syntax highlighting crate](https://docs.rs/syntect).
//! Technicolor library functions permit you to instantiate `syntect::parsing::SyntaxSet` and
//! `syntect::highlighting::ThemeSet` structs with all, or select subsets of, builtin syntaxes and themes.
//! Support can be found in the `build::syntect::syntax` and `build::syntect::theme` modules.
//!
//! This library also supports binary caching of all, or select subsets of, the builtin syntaxes
//! and themes.  Support can be found in the `dump::syntect::syntax` and `dump::syntect::theme` modules.
//!
//! ## License
//!
//! This project is licensed under the [Apache License, v2.0](https://github.com/chrissimpkins/technicolor/blob/master/LICENSE).
//!
//! Syntax and theme definitions are distributed under a number of compatible free licenses.  Please refer to the
//! header of the definition files in the [syntaxes](https://github.com/chrissimpkins/technicolor/tree/master/assets/syntaxes)
//! and [themes](https://github.com/chrissimpkins/technicolor/tree/master/assets/themes) directories of the source
//! repository for details.

pub mod build;
pub mod dump;
pub mod errors;
pub mod paths;

/// The technicolor library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use crate::VERSION;
    use regex::Regex;

    #[test]
    fn test_library_version() {
        let re = Regex::new(r"^\d{1,3}\.\d{1,3}\.\d{1,3}$").unwrap();
        assert!(re.is_match(VERSION));
    }
}
