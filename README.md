# CDP – CD Program

**CDP** stands for **CD Program**, where `cd` refers to the directory‑changing command in Unix‑like shells and PowerShell.

This CLI helps you quickly navigate to a directory and perform useful operations on it.

---

## Supported OS

* Windows (PowerShell)
* Unix‑like (Linux, macOS)

## Supported Editors

* Visual Studio Code

---

## Usage

```bash
cdp [COMMAND]
```

### Example

```bash
cdp g "dir-rust-tests" --ls -C
```

**Explanation**:

* `g`
  Alias for **general** (`cdp general --help`)

* `dir-rust-tests`
  The directory to locate. CDP will search in:

  * `%USERPROFILE%\dir-rust-tests` on Windows
  * `~/dir-rust-tests` on Unix‑like

* `--ls`
  List all files and folders inside the target directory.

* `--current-user` or `-C`
  Tell CDP to look inside the current user’s home folder (e.g. `C:\Users\CDPTester` or `/home/CDPTester`).

---
To see all available options:

```bash
cdp --help
# or
cdp -h
```