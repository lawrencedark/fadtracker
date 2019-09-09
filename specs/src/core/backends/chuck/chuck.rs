//@ - ruckus - A ChucK creative coding toolkit for Rust
//@     - [x] ruck - passes system command flag and all other arguments to `chuck` binary
//@         - [x] Call Rust from ChucK
//@         - [x] Call ChucK from Rust
//@     - [x] Test faust2ck, it generates chugins from FAUST code
//@     - [x] Test Faust ugen, it processes inline FAUST code
//@     - [ ] chuck-sys
//@         - [ ] cpal
//@         - [ ] Write C wrapper for libchuck
//@         - [ ] Instantiate ChucK VMs using libchuck
//@         - [ ] Connect libchuck VMs to audio IO
//@     - [ ] OSC
//@         - [ ] osc_address - type-safe abstraction over serde_osc
//@         - [ ] rosc - replaces liblo and CNMAT OSC-Kit
//@         - [ ] serde_osc - serialize OSC data
//@         - [ ] serde_bytes - for wrapping OSC blobs
