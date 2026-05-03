# TODO

- [ ] husky like pre-commit hooks for linting and testing
  - [ ] `test_cmd` and `lint_cmd` in hooks config, check if they exist and create them in makefile/package.json
  - [ ] add this command example to makefile files to add native hooks:

```bash
# Example setup command in your Makefile
setup:
 cp scripts/pre-push-hook.sh .git/hooks/pre-push
 chmod +x .git/hooks/pre-push`
```

- [x] Add a section on how to add new languages
- [x] makefile for common tasks like linting, testing, building, etc. for those who use makefiles
- [x] tests in typescript
- [ ] java - add gradle
- [x] cleanup if I quit - it goes in background
- [x] create gitignore
