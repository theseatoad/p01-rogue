# Prototype 1 -- Rogue
## Twelve months - Twelve Prototypes

<img width="319" alt="final" src="https://user-images.githubusercontent.com/109775391/217137341-eb4cf6f5-f729-4a08-a388-a4b6abae4b29.png">

## Summary
*All coding stops at the end of the month, follow socials for updates on future project*
For the first month of 2023, I prototyped a rough "clone" of rogue and nethack. It contains:
- procedural map generation (through 2d-map-gen crate)
- 2d-ascii art
- enemies
- dynamic lighting

## To play

### Download executable
- Navigate to releases and download p01-rogue-linux.tar.gz from v0.1.0
- Open and run p01-rogue

### Build from source
- Ensure rust toolchain is installed with ```$ rustc --version```
- - If you need to install it: ```$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh```
- Clone repository or download source from releases.
- Navigate to root directory and ```$ cargo run ```


## Change the seed in the seedable random generation

To change the seed, navigate to `line 26 in /src/map.rs`. Change the ```XXXX``` to any u64 in ```SeedableRng::seed_from_u64(XXXX)```.
