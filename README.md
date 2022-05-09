# OP1NPUT

Maps the Teenage Engineering OP-1's MIDI output to keyboard keypresses so it may be used as a game controller.

This exists because many of the existing tools either require vJoy or emit keyboard events in a way that games can't detect.

OP1NPUT runs in the system tray in Windows, is a 0.3MB executable, and uses less than 3MB of RAM with no background CPU usage.

## Status
- Windows-only, no user configuration, only a framework to get started.
- Control mappings are defined in [src/main.rs](./src/main.rs)

## Usage
- Use the Mini-B USB port to plug your OP-1 into your PC
- Set it to MIDI mode by pressing Shift + COM, then 2
- Set your knobs to relative mode by pressing Shift and rolling the blue knob to the right
- Set your output to CC by pressing Shift and rolling the white knob to the right (this locks your keys to octave 0 and enables the < > keys to be sent; in future we could remove this dependency to let you access more of the board)
- Ensure you are on MIDI channel 1 (shift + green knob left)
- Run op1nput.exe