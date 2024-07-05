<div align="center">
  <img src="https://svgshare.com/i/17F2.svg" height="300" alt="flpc logo">

  # flpc: Lightning-Fast Python Regex

  ![Star](https://img.shields.io/badge/Please%20Give%20A%20Star%20%E2%AD%90-30323D?style=flat-square)
  ![PyPI - Implementation](https://img.shields.io/pypi/implementation/flpc?style=flat-square)
  ![GitHub Issues](https://img.shields.io/github/issues/itsmeadarsh2008/flpc?style=flat-square)
  ![PyPI - Downloads](https://img.shields.io/pypi/dd/flpc?style=flat-square)
  ![GitHub License](https://img.shields.io/github/license/itsmeadarsh2008/flpc?style=flat-square)
  ![GitHub last commit](https://img.shields.io/github/last-commit/itsmeadarsh2008/flpc?display_timestamp=committer&style=flat-square)



  🚀 Supercharge your Python regex with Rust-powered performance!
</div>

## 🌟 Why flpc?
Being in experimental stage. The code structure and dependencies may change. If your project is using this. You will have to manually configure the migrations to latest versions.

flpc is a powerful Python library that wraps the blazing-fast [Rust regex crate](https://crates.io/crates/regex), bringing enhanced speed to your regular expression operations. It's designed to be a drop-in replacement for Python's native `re` module, with some minor syntax differences.

## 🚀 Quick Start

1. Install flpc:
   ```
   pip install flpc
   ```

2. Use it in your code as shown in the API

## 🔧 API

flpc mirrors the `re` module's API, with a few small exceptions:

- Use `fmatch()` instead of `match()` (to avoid conflicts with Python's keyword)
- When using `group()` on a match object, always provide an index (e.g., `group(0)` for the entire match)

Common functions include:

- `compile()`
- `search()`
- `findall()`
- `finditer()`
- `split()`
- `sub()`
- `subn()`

## 💡 Pro Tips

- Pre-compile your patterns for faster execution
- Use raw strings (`r''`) for cleaner regex patterns
- Always check if a match is found before accessing groups
- Remember to use `group(0)` to get the entire match

## 🤝 Contributing

We welcome contributions! Whether it's bug reports, feature requests, or code contributions, please feel free to reach out. Check our [contribution guidelines](CONTRIBUTING.md) to get started.

## 📄 License

flpc is open-source software licensed under the MIT license.