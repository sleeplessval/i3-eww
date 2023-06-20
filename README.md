
# i3 Workspaces Section for EWW

This project is an i3 state reader built for [EWW](https://github.com/elkowar/eww).
I switched my main bar from [Polybar](https://github.com/polybar/polybar) to EWW,
and this project bridges basic IPC from EWW to i3 (something Polybar did natively).

It emits the i3 workspace state as a single element in EWW's unnecessary and
poorly-documented in-house [Yuck](https://elkowar.github.io/eww/configuration.html)
format.

The workspace numbers emitted will have the `ws-btn` class, `focused` and
`urgent` classes where appropriate, and an `onclick` handler that switches to the
workspace it represents.

The program will also issue commands to EWW to update the `WM_MODE` variable with
the current i3 mode.

## Libraries

- [i3ipc](https://crates.io/crates/i3ipc) — handles IPC to i3

