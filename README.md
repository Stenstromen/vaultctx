# VaultCTX

![vaultctx logo](./vault.png)

Context switching for Hashicorp Vault with support for multiple vaults and namespaces

## Installation via Homebrew (MacOS/Linux - x86_64/arm64)

```bash
brew install stenstromen/tap/vaultctx
```

## Download and Run Binary

* For **MacOS** and **Linux**: Checkout and download the latest binary from [Releases page](https://github.com/Stenstromen/vaultctx/releases/latest/)
* For **Windows**: Build the binary yourself.

## Build and Run Binary

```bash
cargo build --release
./target/release/vaultctx
```

## Interactive Mode

Interactive mode (FuzzyFind via skim create) enabled by default:

Set:

```bash
VAULTCTX_IGNORE_FZF=1
```

To disable

## Example Usage

```bash
Usage: vaultctx [OPTIONS] [VAULT_CONTEXT]

Arguments:
  [VAULT_CONTEXT]
          Vault context

Options:
  -s, --switchcontext
          Switch to previous context. Usage: -s

  -c, --currentcontext
          Show current context. Usage: -c

  -d, --delete <DELETE>
          Delete context. Usage: -d 'context_name'

  -r, --rename <RENAME>
          Rename context. Usage: -r 'old_name,new_name'

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
