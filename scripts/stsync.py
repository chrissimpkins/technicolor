#!/usr/bin/env python3

# Copyright 2020 Christopher Simpkins
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
# http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

import pathlib
import sys

import requests


class Definition(object):
    def __init__(self, name, author, license, license_url, url):
        self.name = name
        self.author = author
        self.license = license
        self.license_url = license_url
        self.url = url
        self.comment_opening_delimiter = ""
        self.comment_closing_delimiter = ""

    def __repr__(self):
        return f"Name: {self.name}\nAuthor: {self.author}\nLicense: {self.license}\nLicense URL:{self.license_url}\nURL: {self.url}"


class Syntax(Definition):
    def __init__(self, name, author, license, license_url, url):
        super().__init__(name, author, license, license_url, url)
        self.comment_opening_delimiter = "#"
        self.comment_closing_delimiter = ""

    def get_metadata_text(self):
        return f"{self.comment_opening_delimiter} Copyright {self.author}. {self.license}\n{self.comment_opening_delimiter} License: {self.license_url}\n{self.comment_opening_delimiter} Source: {self.url}\n"


class Theme(Definition):
    def __init__(self, name, author, license, license_url, url):
        super().__init__(name, author, license, license_url, url)
        self.comment_opening_delimiter = "<!--"
        self.comment_closing_delimiter = "-->"

    def get_metadata_text(self):
        return f"{self.comment_opening_delimiter}\nCopyright {self.author}. {self.license}\nLicense: {self.license_url}\nSource: {self.url}\n{self.comment_closing_delimiter}\n"


class DefinitionCollection(object):
    def __init__(self):
        self.syntax_list = []
        self.theme_list = []
        self.syntax_definition_path = "sdefs.txt"
        self.theme_definition_path = "tdefs.txt"

    def parse_definition_files(self):
        # parse syntax definition list
        with open(self.syntax_definition_path, "r") as f:
            for line in f.readlines():
                if (
                    not line.startswith("#") and len(line.strip()) > 0
                ):  # = comments in definition files
                    parsed_list = line.strip().split("::")
                    if len(parsed_list) != 5:
                        sys.stderr.write(f"[PARSE FAIL]: --> {parsed_list}\n")
                        sys.exit(1)
                    syntax = Syntax(
                        parsed_list[0],
                        parsed_list[1],
                        parsed_list[2],
                        parsed_list[3],
                        parsed_list[4],
                    )
                    self.syntax_list.append(syntax)

        # parse theme definition list
        with open(self.theme_definition_path, "r") as f:
            for line in f.readlines():
                if (
                    not line.startswith("#") and len(line.strip()) > 0
                ):  # = comments in definition files
                    parsed_list = line.strip().split("::")
                    assert len(parsed_list) == 5
                    theme = Theme(
                        parsed_list[0],
                        parsed_list[1],
                        parsed_list[2],
                        parsed_list[3],
                        parsed_list[4],
                    )
                    self.theme_list.append(theme)

    def write_syntax_files(self):
        fails = 0
        fails_list = []
        successes = 0
        for c in self.syntax_list:
            r = requests.get(c.url, stream=True)
            if not (r.status_code == requests.codes.ok):
                sys.stderr.write(
                    f"[**FAIL**]: {c.name} ******************** [Status: {r.status_code}]"
                )
                sys.stderr.write(f"Status code: {r.status_code}\n")
                fails += 1
                fails_list.append(c.name)
                break
            if r.encoding is None:
                r.encoding = "utf-8"

            outfile_text = ""
            for line in r.iter_lines(decode_unicode=True):
                outfile_text += line + "\n"
                if line.strip() == "%YAML 1.2":
                    outfile_text += c.get_metadata_text()

            filename = c.name + ".sublime-syntax"
            root_dir = pathlib.Path(__file__).resolve().parent.parent
            outpath = pathlib.PurePath(root_dir).joinpath(
                "assets", "syntaxes", filename
            )

            with open(outpath, "w") as f:
                f.write(outfile_text)
                print(f"[OK] {c.name}")
                successes += 1
        print(f"Successfully pulled {successes} syntax files...")
        if fails > 0:
            print(f"{fails} files were not pulled: {fails_list}")


def main():
    collection = DefinitionCollection()
    collection.parse_definition_files()
    collection.write_syntax_files()


if __name__ == "__main__":
    main()
