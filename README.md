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
bic reset commit_hash
```
Resets workspace back to a previous commit
