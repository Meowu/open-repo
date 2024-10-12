# open-repo

open-repo is a command-line tool that quickly opens the remote page of your current Git repository (supporting GitHub and GitLab) in your default browser.

## Features

- Open repository main page
- Open specific issues (Github only)
- Open specific pull requests (Github only)
- Open issues list or pull requests list (Github only)


## Usage

Execute the following commands under any local directory of a GitHub or GitLab repository:

### Open main repository page

```bash
open_repo
```

### Open a specific issue

```bash
open_repo --issue ISSUE_NUMBER
```

### Open a specific pull request

```bash
open_repo --pull PR_NUMBER
```

### Open issues list or pull requests list

```bash
open-repo --issue
open-repo --pull
```

> ⚠️  Caution

Don't use `--pull` and `--issue` arguments at the same time.

```bash
open_repo --pull PR_NUMBER --issue ISSUE_NUMBER # Don't do this!
```

## Error Handling

If used in a non-Git repository directory, the tool will display an error message and exit.


## Contributing

Issues and pull requests are welcome to help improve this project.


## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
