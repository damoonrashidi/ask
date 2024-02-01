# Ask

Ask tries to understand what shell you're using and then fetches a command specifically for that shell based on your natural language input using ChatGPT. A per shell history is saved to offer quicker lookups and avoiding spamming OpenAI requests.

## Example

```bash
~/❯ ask list all branches and their authors
git for-each-ref --format='%(authorname) %09 %(refname:short)' refs/heads/
```

## Demo

[![asciicast](https://asciinema.org/a/ndUtX47ehTMplYa8ybHN9Rt0o.svg)](https://asciinema.org/a/ndUtX47ehTMplYa8ybHN9Rt0o)

## Installation

Ensure that an `OPENAI_APIKEY` environment variable is available in your shell. API keys can be found in your [api key settings](https://platform.openai.com/api-keys).

```bash
export OPENAI_APIKEY="sk-...xxxx"
```

### Homebrew

```bash
brew tap damoonrashidi/homebrew-ask https://github.com/damoonrashidi/homebrew-ask
brew install ask
```

### Pre-built binaries

Pre-built binaries for linux, macos and windows are available under [releases](https://github.com/damoonrashidi/ask/releases) in this repo.

### From Source

```bash
git clone https://github.com/damoonrashidi/ask
cd ./ask
cargo install --path .
```

## Configuration

A config can be created in the users config directory `~/.config/ask/config.toml`.

```toml
[command]
# If enabled ask will cache responses for each shell into a local history. This yields faster answers for repeated questions and helps avoiding API requests.
#
# default: true
enable_history = true

# Selects which ChatGPT model to query for answers. A full list can be found in the API documentation: https://platform.openai.com/docs/models
#
# default: "gpt-4-1106-preview"
model = "gpt-4-1106-preview"

# The number of choices to ask for, the higher the number the longer the requests will take.
#
# default: 2, min: 1
choice_count = 3

[shell]
# If set, ask will not try to guess the shell and instead use the force_use shell name.
#
# default: None
force_use = "powershell"

# If set, and if ask cannot reliably determine the shell it will fallback to this shell. Overriden by `force_use`
#
# default: "bash"
fallback = "fish"
```

## Shells that can be automatically recognized

- [x] bash
- [x] zsh
- [x] nushell
- [x] Fish
- [ ] Powershell
