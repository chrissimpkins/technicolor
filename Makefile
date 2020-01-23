

lint-syntaxes:
	yamllint -d "{extends: relaxed, rules: {line-length: {max: 10000}}}" assets/syntaxes/*.sublime-syntax


.PHONY: lint-syntaxes