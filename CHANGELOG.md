<div align="center">
    <span style="font-size: 2em; font-weight: bold; vertical-align: middle;">cdp-cli: 0.4.0</span>
    <img src="images/cdplogo-nobg.png" width="50" style="vertical-align: middle;" />
</div>

---

**What's new?**

* **`setup` command created**
  * Flags:
    * `--verbose`, `-v`
  * Run `cdp setup -h` for usage details.

* **`command-aliases` command created**
  * Flags:
    * `--execute`, `-E`, `--exe`
    * `--list`, `-l`, `--ls`
    * `--edit`, `-e`,
    * `--remove`, `-r`, `--rm`
  * Run `cdp command-aliases -h`, `cdp cmdal -h` or `cdp cmd-alias -h` for usage details.

---

**What was changed?**

* **Path parsing system**
  * Now fully supports **relative path**.
    * Examples: `../:cdp`, `:cdp/src`, `../../:cdp/src`

* **Command handlers**
  * Optimization in handlers that required the CDP utilities folder (`/.cdputils`).