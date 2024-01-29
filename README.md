# Ask

Ask tries to understand what shell you're using and then fetches a shell command based on your natural language input using ChatGPT.

## Example

```bash
~/❯ ask list all branches and their authors
git for-each-ref --format='%(authorname) %09 %(refname:short)' refs/heads/
```

## Features (Implemented or Planned)

- [x] Ask for a command using natural language
- [x] Per shell history to avoid spamming OpenAI requests. Works well with reverse history.
- [ ] Planned: Press enter to execute command. `q`/`ESC` to abort.

## Demo

### Using nushell


https://github.com/damoonrashidi/ask/assets/207421/33e9644e-5791-44e9-8674-953b98e95e95

### Using zsh

https://github.com/damoonrashidi/ask/assets/207421/b07409ce-bc3a-4db1-be6d-18d19db6194b



## Installation

Currently the only way to install is via git using Rust. I'm going to add this to homebrew in a bit.

````bash
git clone https://github.com/damoonrashidi/ask
cd ./ask
cargo install --path .
```
