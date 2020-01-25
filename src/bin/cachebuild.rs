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
use syntect::parsing::SyntaxSetBuilder;

use technicolor::paths::{BUILTIN_SYNTAXES_DIR, BUILTIN_THEMES_DIR};
const SYNTAX_CACHE_PATH_NEWLINES: &str = "assets/syntaxes-nl.pack";
const SYNTAX_CACHE_PATH_NO_NEWLINES: &str = "assets/syntaxes-nonl.pack";
const THEME_CACHE_PATH: &str = "assets/themes.pack";

fn build_syntaxes_cache_with_newlines() {
    let mut ssb = SyntaxSetBuilder::new();
    // add Plain Text syntax to all syntax sets by default
    ssb.add_plain_text_syntax();
    ssb.add_from_folder(BUILTIN_SYNTAXES_DIR, true)
        .unwrap_or_else(|error| {
            eprintln!("Error during syntaxes with newline cache build: {}", error);
            std::process::exit(1);
        });
    let ss = ssb.build();
    match dump_to_file(&ss, SYNTAX_CACHE_PATH_NEWLINES) {
        Ok(_) => println!(
            "Succesfully dumped syntaxes with newlines cache to {}",
            SYNTAX_CACHE_PATH_NEWLINES
        ),
        Err(e) => panic!("Error during newline syntax cache dump: {}", e),
    };
}

fn build_syntaxes_cache_without_newlines() {
    let mut ssb = SyntaxSetBuilder::new();
    // add Plain Text syntax to all syntax sets by default
    ssb.add_plain_text_syntax();
    ssb.add_from_folder(BUILTIN_SYNTAXES_DIR, false)
        .unwrap_or_else(|error| {
            eprintln!(
                "Error during syntaxes without newline cache build: {}",
                error
            );
            std::process::exit(1);
        });
    let ss = ssb.build();
    match dump_to_file(&ss, SYNTAX_CACHE_PATH_NO_NEWLINES) {
        Ok(_) => println!(
            "Succesfully dumped syntaxes without newlines cache to {}",
            SYNTAX_CACHE_PATH_NO_NEWLINES
        ),
        Err(e) => panic!("Error during no newline syntax cache dump: {}", e),
    };
}

fn build_themes_cache() {
    let mut ts = ThemeSet::new();
    ts.add_from_folder(BUILTIN_THEMES_DIR)
        .unwrap_or_else(|error| {
            eprintln!("Error during themes cache build: {}", error);
            std::process::exit(1);
        });
    match dump_to_file(&ts, THEME_CACHE_PATH) {
        Ok(_) => println!("Successfully dumped themes cache to {}", THEME_CACHE_PATH),
        Err(e) => panic!("Error during themes cache dump: {}", e),
    };
}

fn main() {
    build_syntaxes_cache_with_newlines();
    build_syntaxes_cache_without_newlines();
    build_themes_cache();
}
