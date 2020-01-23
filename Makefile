

update-syntax-theme:
	cd scripts && python3 stsync.py

lint-syntaxes:
	yamllint -d "{extends: relaxed, rules: {line-length: {max: 10000}}}" --no-warnings assets/syntaxes/*.sublime-syntax


.PHONY: lint-syntaxes update-syntax-theme