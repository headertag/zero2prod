# zero2prod

## Development Environment: Vim + rust-analyzer

### Prerequisites

1. **Rust toolchain** via [rustup](https://rustup.rs):

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

   Verify:

   ```bash
   rustc --version
   cargo --version
   ```

2. **rust-analyzer** (the Rust LSP server):

   ```bash
   rustup component add rust-analyzer
   ```

   Verify:

   ```bash
   rust-analyzer --version
   ```

3. **clippy and rustfmt**:

   ```bash
   rustup component add clippy rustfmt
   ```

4. **Vim 9.0+** with `+channel` and `+job` support (check with `vim --version`).

### Install the LSP Plugin

We use [yegappan/lsp](https://github.com/yegappan/lsp), a Vim9-native LSP client. Install it with Vim's built-in package manager — no external plugin manager needed:

```bash
mkdir -p ~/.vim/pack/plugins/start
git clone https://github.com/yegappan/lsp.git ~/.vim/pack/plugins/start/lsp
```

### Configure Vim

Add the following to `~/.vimrc`:

```vim
" General settings
set nocompatible
syntax on
filetype plugin indent on
set number
set relativenumber
set signcolumn=yes
set updatetime=300
set completeopt=menuone,popup,noinsert,noselect

" Rust file settings
autocmd FileType rust setlocal tabstop=4 shiftwidth=4 expandtab

" LSP configuration (yegappan/lsp with rust-analyzer)
let lspServers = [
    \   #{
    \     name: 'rust-analyzer',
    \     filetype: ['rust'],
    \     path: expand('~/.rustup/toolchains/stable-aarch64-apple-darwin/bin/rust-analyzer'),
    \     args: [],
    \     syncInit: v:true,
    \   }
    \ ]

autocmd VimEnter * call LspAddServer(lspServers)

" LSP keybindings
autocmd FileType rust nnoremap gd :LspGotoDefinition<CR>
autocmd FileType rust nnoremap gr :LspShowReferences<CR>
autocmd FileType rust nnoremap gi :LspGotoImpl<CR>
autocmd FileType rust nnoremap K :LspHover<CR>
autocmd FileType rust nnoremap <leader>rn :LspRename<CR>
autocmd FileType rust nnoremap <leader>ca :LspCodeAction<CR>
autocmd FileType rust nnoremap [d :LspDiagPrev<CR>
autocmd FileType rust nnoremap ]d :LspDiagNext<CR>
autocmd FileType rust nnoremap <leader>d :LspDiagCurrent<CR>
```

### Verify the Setup

From the command line:

```bash
# Project compiles
cargo check

# Clippy passes
cargo clippy

# Formatting is correct
cargo fmt --check

# rust-analyzer can analyze the project
rust-analyzer analysis-stats /path/to/zero2prod
```

From inside Vim:

1. Open a Rust file: `vim src/main.rs`
2. Verify LSP is running: `:LspServerStatus` — should show rust-analyzer as running.
3. Test go-to-definition: place cursor on `println!` and press `gd`.
4. Test hover: press `K` on any symbol to see type info.
5. Test diagnostics: introduce a type error and check that `[d` / `]d` navigate between errors.

### Keybinding Reference

| Key | Action |
|---|---|
| `gd` | Go to definition |
| `gr` | Show references |
| `gi` | Go to implementation |
| `K` | Hover documentation |
| `<leader>rn` | Rename symbol |
| `<leader>ca` | Code action |
| `[d` / `]d` | Previous / next diagnostic |
| `<leader>d` | Show current diagnostic |
| `:LspDiagShow` | Open quickfix list with all diagnostics |
