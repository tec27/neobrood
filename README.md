# neobrood

neobrood aims to be a compatible implementation of StarCraft: Remastered's game engine, using
entirely new and original code based on the [Bevy](https://bevyengine.org/) game engine. It is
currently incomplete and a work-in-progress with many features either missing or incorrectly
implemented.

![gameplay-screenshot](https://github.com/tec27/neobrood/assets/360513/4e2b8f1c-2133-4dfb-81ab-bc23dd5c1070)

## Disclaimer

StarCraft is a registered trademark of Blizzard Entertainment, Inc.

neobrood is not affiliated or endorsed by Blizzard, and contains no copyrighted material from the
original game. All copyrighted graphics, sounds, music, and related assets must be provided by
players who may acquire them from Blizzard's
[Battle.net service](https://download.battle.net/en-us/desktop).

## Setup

### Git LFS

If you have not previously used/installed Git LFS, you'll need to run a couple more commands after
cloning to get the assets:

```shell
git lfs install
git lfs pull
```

### Rust build tools

This uses the LLD linker, which may not be installed by default. Run this once to get it:

```shell
cargo install -f cargo-binutils
rustup component add llvm-tools-preview
```

### CASC files

This makes use of assets from SC:R's CASC archive, but I haven't yet written the code to load them
at runtime. Instead, you should extract the CASC contents to `assets/casc-extracted/` using
[CascView](http://www.zezula.net/en/casc/main.html). Depending on what asset pack + quality settings
you have, you may need different files, but you generally want `anim`, `music`, `sound`, and
`tileset`. To generate new game rules code (`gen_rules`) you will additionally need the `arr` and
`scripts` folders, but unless you are making changes to those types this is not required.

## Running

```shell
cargo run --features bevy/dynamic_linking
```

It will probably take a while to build the first time, but subsequent builds should be faster
provided you haven't changed dependencies/rust versions.

If things are running slowly or you want to see it actualy production speed (this will take a long
time to build):

```shell
cargo run --release
```

A series of map paths can be specified as arguments. Any directories in the arguments will have
their contents added (non-recursively), so you can specify a directory of maps to open. To navigate
to the next map, press spacebar.

## Settings

The game will load settings from `My Documents\Starcraft\neobrood-settings.json`. See `GameSettings`
in [main.rs](src/settings.rs) for the available options. An example of a basic settings file might
be:

```json
{
    "windowMode": "windowed",
    "windowSize": [1920, 1080],
    "volumes": {
        "global": 0.3
    }
}
```
