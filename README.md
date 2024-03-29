# sourcepak
sourcepak is a Rust crate that provides support for working with VPK (Valve Pak) files. It allows you to read and write VPK files, as well as extract and pack their contents.

## Features
- [x] Read and write VPK files
- [x] Extract files from VPK archives
- [x] Optionally memory-map archive files for faster speeds (with the `mem-map` feature)
- [ ] Pack files into VPK archives

## Supported formats
### VPK v1 (Alien Swarm, Dota 2, L4D, L4D2, Portal 2, SFM)
- [x] Read directory files
- [x] Read file contents
- [ ] Patch existing VPKs
- [ ] Write new VPKs

### VPK v2 (CS:GO, CS:S, DoD:S, HL:S, HL2, HL2:DM, Portal, TF2, Source 2)
- [x] Read directory files
- [ ] Read file contents
- [ ] Patch existing VPKs
- [ ] Write new VPKs

### Respawn VPK (Titanfall)
- [x] Read directory files
- [x] Read file contents
- [x] Read audio files
- [ ] Patch existing VPKs
- [ ] Write new VPKs

## Documentation
Documentation can be found [here](https://docs.rs/sourcepak)

## Why does this crate exist?
I originally created the [TFVPKTool](https://github.com/barnabwhy/TFVPKTool) TypeScript library to support reading Respawn VPK files, along with [Harmony VPK Tool](https://github.com/harmonytf/HarmonyVPKTool) using Electron.

I very quickly noticed the issue that these often resulted in high memory use due to language and ecosystem I had used.

With sourcepak I am aiming to fix that.
