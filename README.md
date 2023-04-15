# Mouse Tracker: multiplatform cursor tracking

## Mouse tracking
Simply tracks the X,Y position of the cursor on the screen. If you have DPI settings, the pixel position will be scaled to whatever DPI scale you have set.

## Run
`cargo b --release`

`./tracker -s <samplerate>  -l 1`

## TODOs:
- [ ] Make it _actually_ multiplatform. Doesn't build on linux.
- [ ] Ability to change sample rate on the fly
- [x] Add LSL for synchronization
- [x] Change sample rate through command line arguments
- [x] Data is piped into LSL.
