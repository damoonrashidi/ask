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

[![asciicast](https://asciinema.org/a/ZrtA0UqjDyXBhwPfhRxeAip5f.svg)](https://asciinema.org/a/ZrtA0UqjDyXBhwPfhRxeAip5f)

### Using zsh

[![asciicast](https://asciinema.org/a/8a3O4znxFshyrpBhAx8Di6160.svg)](https://asciinema.org/a/8a3O4znxFshyrpBhAx8Di6160)

## Installation

### Pre-built binaries

Pre-built binaries are available under [releases](https://github.com/damoonrashidi/ask/releases) in this repo.

### From Source
```bash
git clone https://github.com/damoonrashidi/ask
cd ./ask
cargo install --path .
```
