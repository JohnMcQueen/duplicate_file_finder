# Duplicate File Finder
First crack at a Rust application.

A Rust-based command-line tool to identify potential duplicate files based on their file size.

## Overview

This program recursively scans a directory and groups files by their size. Files with identical sizes are then hashed using sha-256 to confirm the contents are the same. The potential duplicate are then shown in the console to the user.

## Example Output
  ```
User inputed directory to scan: filez
 Duplicates found (hash: 73895211e3dacf5642f427f4b294e412d20b5b729be3d1fb5c66a40feef68ede)
    - filez/filez/dogs/golden_retriever.txt
    - filez/filez/dogs/beagle.txt
 Duplicates found (hash: e1586af65e836d956cbc68f0a23cb58c66529214af1c0a5667b40a1d578d9f5f)
    - filez/small1.txt
    - filez/small2.txt
 Duplicates found (hash: 27f41416391a7dfb298196b4196f6a86ccd08481d567a67d62cb2f88a580ecce)
    - filez/filez/dogs/labrador.txt
    - filez/filez/dogs/labrador1.txt
```

## Usage

```bash
cargo run -- <directory_path>
```

Example:
```bash
cargo run -- ~/Downloads
```

Testing:
```
cargo test
```

Testing with coverage (utilizing `llvm-cov`):
```
cargo llvm-cov --open
```

## How It Works

1. Recursively traverses the specified directory using `WalkDir`
2. Collects metadata for each file (size in bytes)
3. Groups files by their size
4. For groups with multiple files (potential duplicates):
   - Computes SHA-256 hash of each file's contents
   - Groups files by their hash value
   - Files with identical hashes are true duplicates (same content)
5. Displays duplicate groups to the user

## Current Status

- ✅ Directory scanning and file size collection
- ✅ Grouping files by size
- ✅ SHA-256 hash-based duplicate verification
- ✅ Basic console output
- ✅ Unit tests with coverage tracking

## Limitations

- **Read-only**: Only reports duplicates, does not delete or modify files
- **Single-threaded**: Files are hashed sequentially (large directories may be slow)
- **Memory usage**: Stores all file paths in memory
- **Symbolic links**: May not handle symlinks correctly
- **No progress indication**: No feedback during long-running scans

## Future Enhancements

- Interactive mode to select which duplicates to delete
- Parallel/multi-threaded hashing for improved performance
- Progress bar for large directory scans
- Export results to JSON/CSV format
- Whitelist/blacklist file patterns
- Option to compare only files above/below certain size thresholds
- Dry-run mode to simulate deletion
- Configuration file support
