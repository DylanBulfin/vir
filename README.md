# Vir
A terminal modal text editor, similar to vi(m). Supports rebinding for nearly all keys,
via a config file at `~/.config/vir/config.toml`. See default config at
`src/config/default.toml` for more explanation.

## Video Demo
[A YouTube demo that shows off the main features](https://youtu.be/RQ8O1kJQ5WQ)

## Current State
All features I wanted have been added. I will only be making updates if I notice bugs
    
## Non-Goals
- Plugin system
- File management unrelated to editing (e.g. delete/create file)
- LSP, DAP, treesitter, completions, etc
    - Maybe an extremely limited snippet completion system as a stretch goal
