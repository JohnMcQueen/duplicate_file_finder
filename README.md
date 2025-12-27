# Duplicate File Finder

A Rust-based command-line tool to identify potential duplicate files based on their file size.

## Overview

This program recursively scans a directory and groups files by their size. Files with identical sizes are potential duplicates and will be displayed to the user for further review.

## Usage

```bash
cargo run -- <directory_path>
```

Example:
```bash
cargo run -- ~/Downloads
```

## How It Works

1. Recursively traverses the specified directory using `WalkDir`
2. Collects metadata for each file (size in bytes)
3. Groups files by their size
4. Identifies files with matching sizes as potential duplicates

## Current Status

- ✅ Directory scanning and file size collection
- ✅ Grouping files by size
- 🚧 Display formatting (TBD)

## Note

Files with identical sizes are not guaranteed to be duplicates - they may have different content. Future enhancements may include hash-based verification for true duplicate detection.
