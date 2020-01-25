THEMES_DIR=assets/themes
SYNTAXES_DIR=assets/syntaxes

update-syntaxes:
	cd scripts && python3 stsync.py --syntax

update-themes:
	cd scripts && python3 stsync.py --theme

update-syntaxes-themes: update-syntaxes update-themes

dump-syntax-theme-binary:
	cargo run --bin cachebuild

lint: lint-themes lint-syntaxes

lint-syntaxes:
	@echo "Linting $(SYNTAXES_DIR) directory contents..."
	yamllint -d "{extends: relaxed, rules: {line-length: {max: 10000}}}" --no-warnings $(SYNTAXES_DIR)/*.sublime-syntax

lint-themes:
	@echo "Linting $(THEMES_DIR) directory contents..."
	plutil $(THEMES_DIR)/*.tmTheme
	xmllint --valid --noout $(THEMES_DIR)/*.tmTheme


.PHONY: dump-syntax-theme-binary lint lint-syntaxes lint-themes update-syntaxes update-themes update-syntaxes-themes