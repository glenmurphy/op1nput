# OP1NPUT

Maps the Teenage Engineering OP-1 midi output to keyboard keypresses; runs in the system tray in Windows.

## Status
- Windows-only, no user configuration, only a framework to get started.
- Control mappings are defined in [src/main.rs](./src/main.rs)

## Usage
- Plug in your OP-1
- Set it to MIDI mode by pressing Shift + COM, then 2
- Set your knobs to relative mode by pressing Shift and rolling the blue knob to the right
- Set your output to CC by pressing Shift and rolling the white knob to the right (this locks your keys to octave 0 and enables the < > keys to be sent; in future we could remove this dependency to let you access more of the board)
- Ensure you are on MIDI channel 1 (shift + green knob)
- Run op1nput