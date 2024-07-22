# Goals
- Support vim-like text editing
    - Three modes: Normal, Visual, Insert
- Basic configuration (probably TOML)
    - Editor options (tabstop, relativenumber, etc)
    - All keybindings reassignable (maybe except C-c?)
- Probably yank/paste registers?
    - Also probably try to interface with system clipboard
    
# Non-Goals
- Plugin system
- File management unrelated to editing (e.g. delete/create file)
- LSP, DAP, treesitter, completions, etc
    - Maybe an extremely limited snippet completion system as a stretch goal
