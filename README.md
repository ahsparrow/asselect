# README

## Installation

### Install Rust

Follow the instructions from the [Rust website](https://www.rust-lang.org/tools/install)

### Install the WebAssembly target

`rustup target add wasm32-unknown-unknown`

### Install Trunk

`cargo install --locked trunk`

## Deployment

First create a new branch for GitHub pages

```shell
git checkout --orphan gh-pages
git rm -rf .
cat > .gitattributes
openair.txt eol=crlf
<ctrl-D>
git add .gitattributes
git commit -m "New branch"
git push -u origin gh-pages
git checkout main
```

### Copy new airspace files

Copy the airspace files `yaixm.json`, `openair.txt`, `overlay_105.txt`,
`overlay_195.txt`, and `overlay_atzdz.txt` to the data directory.

### Build

`trunk build --release`

### Deploy to GitHub pages

`./deploy.sh`
