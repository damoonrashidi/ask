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
- [x] Press enter to execute command. `q`/`ESC` to abort.
- [ ] Editing the command before executing when pressing `e`.
- [ ] Copy command to clipboard when pressing `c`.
- [ ] Add command to shell history after executing.
- [ ] Add config file to enable/disable command caching, overriding shell detection etc.

## Shells that can be recognized
- [x] bash
- [x] zsh
- [x] nushell
- [ ] Fish
- [ ] Powershell

## Demo

[![asciicast](https://asciinema.org/a/ndUtX47ehTMplYa8ybHN9Rt0o.svg)](https://asciinema.org/a/ndUtX47ehTMplYa8ybHN9Rt0o)

## Installation

Ensure that an `OPENAI_APIKEY` environment variable is available in your shell. API keys can be found in your [api key settings](https://platform.openai.com/api-keys).

```bash
export OPENAI_APIKEY="sk-...xxxx"
```

### Pre-built binaries

Pre-built binaries for linux, macos and windows are available under [releases](https://github.com/damoonrashidi/ask/releases) in this repo.

### From Source
```bash
git clone https://github.com/damoonrashidi/ask
cd ./ask
cargo install --path .
```
