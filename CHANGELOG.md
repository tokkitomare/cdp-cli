<div align="center">
    <span style="font-size: 2em; font-weight: bold; vertical-align: middle;">cdp-cli: 0.3.0</span>
    <img src="images/cdplogo-nobg.png" width="50" style="vertical-align: middle;" />
</div>

---

**What's new?**

* **`aliases` command**
  * Added 3 new flags:
    * `--list`, `-l`, `--ls`
    * `--edit`, `-e`,
    * `--remove`, `-r`, `--rm`
  * Run `cdp aliases -h` or `cdp alias -h` for usage details.

* **`create-project` command**
  * Added a new flag:
    * `--path`, `-p`
  * Run `cdp create-project -h` or `cdp cp -h` for usage details.

---

**What was changed?**

* **`aliases` command**
  * The arguments `<PATH>` and `<ALIAS>` are now optional.