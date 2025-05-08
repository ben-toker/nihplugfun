# spectral freeze
I built this for the final project for my TECH350: Digital Signal Processing (for electronic musicians). The idea is to
build out a "spectral freeze," where we take a spectral frame (fast Fourier transform) and then write its inverse (output
back to real) to the output repeatedly (until we want it to stop). 

This is built off of [NIH-Plug](https://github.com/robbert-vdh/nih-plug?tab=readme-ov-file)
because I Rust > C++ (JUCE) and want to program in this language as much as possible.

## Installation and building
To get this running, clone the repo and run 
```
cargo xtask bundle spectralfreeze --release
```
