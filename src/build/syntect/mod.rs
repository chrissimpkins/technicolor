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

//! The `technicolor::build::syntect` submodules support instantiation of [syntect crate](https://docs.rs/syntect)
//! `syntect::parsing::SyntaxSet` and `syntect::highlighting::ThemeSet` structs with builtin
//! syntaxes and themes.

pub mod syntax;
pub mod theme;
