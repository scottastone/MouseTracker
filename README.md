# Mouse Tracker: multiplatform cursor tracking

## Mouse tracking
Simply tracks the X,Y position of the cursor on the screen. Ability to save (coming soon) and pipe data into LSL (also .. coming soon).

## Run
`cargo b`

`./tracker -s <samplerate>  -l 1`

## TODOs:
- [ ] Make it _actually_ multiplatform. Doesn't build on linux.
- [ ] Ability to change sample rate on the fly
- [x] Add LSL for synchronization
- [x] Change sample rate through command line arguments
