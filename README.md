# Ask

Ask tries to understand what shell you're using and then fetches a command specifically for that shell based on your natural language input using ChatGPT. A per shell history is saved to offer quicker lookups and avoiding spamming OpenAI requests.

Most shells on linux and unix can be automatically detected. Windows support is limited but can be enabled by hard-setting the shell via the [config](#configuration)

## Example

```bash
~/❯ ask list all branches and their authors
git for-each-ref --format='%(authorname) %09 %(refname:short)' refs/heads/
```

## Demo

[![asciicast](https://asciinema.org/a/635793.svg)](https://asciinema.org/a/635793)

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

A config can be created in the users config directory `~/.config/ask/config.toml` on macos/linux.

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
# If set, ask will not try to guess the shell and instead use the force_use shell name. Must be the actual shell binary name such as "nu" for nushell.
#
# default: None
force_use = "pwsh"

# If set, and if ask cannot reliably determine the shell it will fallback to
# this shell. Overriden by `force_use`. Must be the actual shell binary name
# such as "nu" for nushell.
#
# default: "bash"
fallback = "fish"
```
