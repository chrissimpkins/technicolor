THEMES_DIR=assets/themes
SYNTAXES_DIR=assets/syntaxes

update-syntax-theme:
	cd scripts && python3 stsync.py

lint: lint-themes lint-syntaxes

lint-syntaxes:
	@echo "Linting $(SYNTAXES_DIR) directory contents..."
	yamllint -d "{extends: relaxed, rules: {line-length: {max: 10000}}}" --no-warnings $(SYNTAXES_DIR)/*.sublime-syntax

lint-themes:
	@echo "Linting $(THEMES_DIR) directory contents..."
	plutil $(THEMES_DIR)/*.tmTheme
	xmllint --valid --noout $(THEMES_DIR)/*.tmTheme


.PHONY: lint lint-syntaxes lint-themes update-syntax-theme