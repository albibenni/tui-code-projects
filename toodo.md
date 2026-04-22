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
- [ ] tell when cleanup is done
- [ ] last step if esc blank screen, with a second press go back (should go back with the first)
- [ ] cleanup if I quit
- [ ] lint require 2 enter to accept
