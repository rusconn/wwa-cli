# wwa-cli

A WWA(World Wide Adventure) helper.

## Install

```sh
cargo install --git https://github.com/rusconn/wwa-cli.git
```

## Example

./enemies.json5:
```json5
[
  { name: "goblin", hp: 30, atk: 10, def: 1 },
  { name: "wolf", hp: 25, atk: 12, def: 3 },
  { name: "golem", hp: 22, atk: 15, def: 22 },
]
```

### Analyze

```sh
wwa analyze ./enemies.json5 --atk 11 --def 5
```

output:
```sh
goblin
  †+5 ⇒ -5 # † means atk
  ⛨+1 ⇒ -2 # ⛨ means def
wolf
  †+1 ⇒ -7
  ⛨+1 ⇒ -3
golem
  †+12 ⇒ ok
```

### Breakpoints

```sh
wwa breakpoints ./enemies.json5 --min 11 --max 20
```

output:
```sh
11: goblin
12: wolf
16: goblin,wolf
```
