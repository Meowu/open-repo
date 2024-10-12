# open-repo

Open current repo in browser from command line.


## Usage

Execute the command under a *github* or *gitlab* repository.

### Open main page

```bash
open_repo
```

### Open an issue

```bash
open_repo --issue ISSUE_NUMBER
```

### Open a pull request

```bash
open_repo --pull PR_NUMBER
```

### Caution

Don't use `--pull` and `--issue` arguments at the same time.

```bash
open_repo --pull PR_NUMBER --issue ISSUE_NUMBER # Don't do this!
```

If no issue number or pr number is specified, the pulls page or issues page will be opened.
