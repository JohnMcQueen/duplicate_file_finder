# Duplicate file finder

## 2. Plan your approach
The most efficient strategy is to:
* Group files by size first (duplicates must have the same size)
* For files with matching sizes, compute content hashes
* Files with identical hashes are duplicates

## 4. Implement the core logic
Your main components should be:
* File scanning: Use walkdir to recursively traverse directories and collect file paths
* Size grouping: Create a HashMap where keys are file sizes and values are vectors of file paths
* Hash computation: For size groups with multiple files, compute SHA-256 hashes of file contents
* Duplicate detection: Group files by their hashes to identify duplicates

## 5. Handle edge cases
Consider:
* Symbolic links (skip or follow them?)
* Permissions errors (some files may not be readable)
* Very large files (consider reading in chunks for memory efficiency)
* Empty files (all empty files have the same "content" but may not be useful duplicates)

## 6. Design the output
Decide how to present results:
* Print groups of duplicate files
* Show total wasted space
* Optionally provide deletion suggestions

## 7. Add CLI interface
Use clap to accept:
* Directory path(s) to scan
* Options like --min-size to skip small files
* Flags for follow symlinks, hidden files, etc.
