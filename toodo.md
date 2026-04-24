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

- [ ] Add a section on how to add new languages
- [ ] makefile for common tasks like linting, testing, building, etc. for those who use makefiles
- [ ] tests in typescript
- [ ] java - add gradle
- [ ] cleanup if I quit - it goes in background
- [ ] create gitignore
