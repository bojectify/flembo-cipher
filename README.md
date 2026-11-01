# flembo-cipher

A command-line tool that turns a phrase into a letter-substitution code, the
kind used in cryptogram puzzles. It cleans the phrase, swaps every letter for a
different one, and prints JSON containing the cleaned phrase, the code, and the
key needed to solve it.

```bash
$ flembo-cipher "Hello, world!" --seed 42
{"phrase":"hello world","cipher":"wbyyh vhmyc","key":{"b":"e","c":"d","h":"o","m":"r","v":"w","w":"h","y":"l"}}
```

## Installation

### Homebrew (macOS and Linux)

```bash
brew install bojectify/tap/flembo-cipher
```

### Shell installer (macOS and Linux)

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/bojectify/flembo-cipher/releases/latest/download/flembo-cipher-installer.sh | sh
```

### PowerShell installer (Windows)

```powershell
powershell -ExecutionPolicy Bypass -c "irm https://github.com/bojectify/flembo-cipher/releases/latest/download/flembo-cipher-installer.ps1 | iex"
```

Prebuilt binaries for every platform are also attached to each
[GitHub release](https://github.com/bojectify/flembo-cipher/releases).

### From source

You'll need [Rust](https://rustup.rs) (edition 2024).

```bash
cargo install --git https://github.com/bojectify/flembo-cipher
```

## Usage

```
flembo-cipher [OPTIONS] <PHRASE>
```

| Argument / option | Description |
| --- | --- |
| `<PHRASE>` | The phrase to turn into a code (required) |
| `--seed <SEED>` | Seed for repeatable output. Without it, each run is random. |
| `--max-length <MAX_LENGTH>` | Maximum length of the cleaned phrase, in characters. Default: `150` |
| `-h`, `--help` | Show help (`--help` gives more detail) |
| `-V`, `--version` | Show the version |

### Output

JSON on stdout, on a single line:

| Field | Description |
| --- | --- |
| `phrase` | The cleaned phrase: the solved code |
| `cipher` | The phrase with every letter swapped: what the player sees |
| `key` | Cipher letter → real letter, for every letter used in the phrase. Keys are in alphabetical order. |

```bash
$ flembo-cipher "Don't panic -- it's well-known!" --seed 7
```

```json
{
  "phrase": "dont panic its well known",
  "cipher": "zgiw xsinv nwb jpmm rigji",
  "key": {
    "b": "s", "g": "o", "i": "n", "j": "w", "m": "l", "n": "i", "p": "e",
    "r": "k", "s": "a", "v": "c", "w": "t", "x": "p", "z": "d"
  }
}
```

(Formatted here for readability; the real output is one line.)

## How a phrase is processed

### 1. Validation

The phrase is rejected if it contains:

- **digits**, e.g. `4 eggs`
- **letters outside A–Z**, e.g. `café`

These are errors rather than being stripped, because removing them would change
the meaning of the phrase.

### 2. Cleaning

- Everything is lowercased.
- Apostrophes are removed: `don't` → `dont`.
- Hyphens and dashes (`-`, `‐`, `‑`, `‒`, `–`, `—`, `―`, `−`) become spaces:
  `well-known` → `well known`.
- All other punctuation is removed.
- Repeated spaces are collapsed, and leading/trailing spaces are trimmed.

The cleaned phrase must then contain at least one letter and be no longer than
`--max-length` characters.

### 3. Scrambling

A random mapping is built across the whole alphabet, with these guarantees:

- **Consistent**: a letter is always replaced by the same letter throughout
  the phrase.
- **One-to-one**: no two letters are replaced by the same letter.
- **No letter maps to itself**: an `e` never appears as `e`.

Spaces are kept so players can see word lengths. The `key` only includes letters
that appear in the phrase, so it never reveals anything extra.

With the same `--seed` and the same phrase, the output is identical every time.

## Errors

Errors go to stderr, and nothing is printed to stdout, so a calling program can
safely parse stdout as JSON.

| Situation | Message | Exit code |
| --- | --- | --- |
| Digit or non A–Z letter | `error: invalid character: 4` | 1 |
| No letters after cleaning | `error: no words found` | 1 |
| Cleaned phrase over the limit | `error: phrase is too long: 152 (max 150)` | 1 |
| Missing phrase, or invalid option | usage message from the argument parser | 2 |

## Contributing

Bug reports and pull requests are welcome. See [CONTRIBUTING.md](CONTRIBUTING.md)
for how to build, test and release the project.

## License

[MIT](LICENSE)
