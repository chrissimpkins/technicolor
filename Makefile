

update-syntax-theme:
	cd scripts && python3 stsync.py

lint: lint-syntaxes lint-themes

lint-syntaxes:
	@echo "Linting syntax directory contents..."
	yamllint -d "{extends: relaxed, rules: {line-length: {max: 10000}}}" --no-warnings assets/syntaxes/*.sublime-syntax

lint-themes:
	@echo "Linting themes directory contents..."
	xmllint --valid --noout assets/themes/*.tmTheme


.PHONY: lint lint-syntaxes lint-themes update-syntax-theme