# neobrood

## Setup

### Git LFS

If you have not previously used/installed Git LFS, you'll need to run a couple more commands after
cloning to get the assets:

```shell
git lfs install
git lfs pull
```

### CASC files

This makes use of assets from SC:R's CASC archive, but I haven't yet written the code to load them
at runtime. Instead, you should extract the CASC contents to `assets/casc-extracted/` using
[CascView](http://www.zezula.net/en/casc/main.html). For now, the `tileset` directory from the root
is the only one required.

## Running

```shell
cargo run
```

It will probably take a while to build the first time, but subsequent builds should be faster
provided you haven't changed dependencies/rust versions.

If things are running slowly or you want to see it actualy production speed (this will take a long
time to build):

```shell
cargo run --release
```
