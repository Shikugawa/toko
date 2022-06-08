# toko
A simple snippet manager

### How to use

Create/Edit snippet file as follows:

```sh
toko edit
```

It opens editor (vim). You should write down your snippets as follows:

```.toko.yaml
- tag:
  - "normal"
  desc: "sample cmd"
  cmd: "ls"
```

You can lookup a snippet with fuzzy search feature and it outputs matched command.

```sh
# if you selected "simple cmd", it outputs "ls".
# Thus it will be good to send it with pipe and execute the command.
toko search | sh -
```
