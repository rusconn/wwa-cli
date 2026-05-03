# wwa-cli

A WWA(World Wide Adventure) helper.

## Install

```sh
cargo install --git https://github.com/rusconn/wwa-cli.git
```

## Example

```sh
wwa breakpoints ./enemies.json5 --min 11 --max 20
```

./enemies.json5:
```json5
[
  { name: "goblin", hp: 30, def: 1 },
  { name: "wolf", hp: 25, def: 3 },
]
```

output:
```sh
11: goblin
12: wolf
16: goblin,wolf
```
