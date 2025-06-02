# bic
A custom version control system implementation written in Rust!

## Installation
Make sure you have rust installed. If not, you can do so via [rustup](https://rustup.rs/).\
Then run:
```bash
git clone https://github.com/jashith1/bic/
cd bic
cargo install --path .
```

This installs bic into your cargo binaries.\
Make sure ~/.cargo/bin is in your PATH
```bash
export PATH="$HOME/.cargo/bin:$PATH"
```
For it to be permanently in your PATH, you'll have to add it to .bashrc 

## Usage
To initialize the repository:
```bash
bic init
```

To commit a change:
```bash
bic commit -m "your message"
```
directories or files in your .bic_ignore file are excluded from commits.

View commit history:
```bash
bic log
```

Reset to a commit:
```bash
bic reset <commit_hash>
```
Resets workspace back to a previous commit

## How it works
Bic has all the core features you'd expect in a VCS
- When you run bic commit, it hashes the contents of all tracked files and stores them in .bic/objects/, avoiding duplicates.
- A commit metadata file is created in .bic/commits/ which stores the commit message, timestamp, parent hash, and a list of all tracked files as a key value pair of their hashes and actual file name.
- The latest commit hash is saved in .bic/HEAD.
- bic log reads the commit chain backwards using the parent links starting from HEAD.
- bic reset <commit_hash> deletes all files in the workspace (except ignored ones) and restores files from the specified commit.