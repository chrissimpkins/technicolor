THEMES_DIR=assets/themes
SYNTAXES_DIR=assets/syntaxes

# ===============================
# Syntax and theme update targets
# ===============================
sync: update-syntaxes-themes
	cargo run --bin cachebuild

update-syntaxes:
	cd scripts && python3 stsync.py --syntax

update-themes:
	cd scripts && python3 stsync.py --theme

update-syntaxes-themes: update-syntaxes update-themes

# Binary cache dump targets
# (eliminates pull step before that occurs in `sync` target)
dump-syntax-theme-binary:
	cargo run --bin cachebuild

# ===============================
# Source linting targets
# ===============================

lint: lint-python lint-themes lint-syntaxes

lint-python:
	flake8 --ignore=E501,W50 scripts/*.py

lint-syntaxes:
	@echo "Linting $(SYNTAXES_DIR) directory contents..."
	yamllint -d "{extends: relaxed, rules: {line-length: {max: 10000}}}" --no-warnings $(SYNTAXES_DIR)/*.sublime-syntax

lint-themes:
	@echo "Linting $(THEMES_DIR) directory contents..."
	plutil $(THEMES_DIR)/*.tmTheme
	xmllint --valid --noout $(THEMES_DIR)/*.tmTheme

# ===============================
# Source formatting targets
# ===============================
fmt: fmt-python fmt-rs

fmt-python:
	black scripts/*.py

fmt-rs:
	cargo fmt

# ===============================
# Test targets
# ===============================
test: fmt-rs
	cargo test

.PHONY: \
dump-syntax-theme-binary \
fmt fmt-python fmt-rs \
lint lint-python lint-syntaxes lint-themes \
update-syntaxes update-themes update-syntaxes-themes