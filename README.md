# deno-ls-init (`denolsinit`)

`denolsinit` is a cli tool to create (edit) tsconfig.json that enables code completion for deno.

If you use VSCode, you should use the [Deno extension](https://marketplace.visualstudio.com/items?itemName=justjavac.vscode-deno).

This tool mainly focuses on Emacs users.

However, since this tool only rewrites tsconfig.json, I think it's usable with any IDEs that can use the typescript language server (which references tsconfig.json).

## Usage
### Installing

Emacs user:

- Install node.js and Rust
- Install [yarn](https://classic.yarnpkg.com/)
- Install and enable [Tide](https://github.com/ananthakumaran/tide) package

If you're a Doom Emacs user, just enable `lang: javascript` in setting file (`init.el`) and run `doom refresh`.

- `cargo install deno-ls-init`

### Enabling code completion for deno
- `cd` to the deno project root directory.
- `denolsinit`

That's all.
