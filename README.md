# Ballad

A simple, customizable, themable, and functional desktop environment
for tiling window managerwith a focus on Wayland.

> You'll never switch back to GNOME.

At this stage in development, Ballad looks like this:

#![Image of Ballad shell](todo)

Ballad will have first class support for [Niri](https://github.com/yalter/niri)
as well as more common TWMs like [Sway](https://github.com/swaywm/sway) and [Hyprland](https://github.com/hyprwm/hyprland).
In the future, X11 TWMs like i3 may get support.
Niri is the only TWM currently supported. (feel free to make a PR for your TWM of choice!)

At this stage in development, Ballad looks like this:


## Crates

Ballad is split into several smaller crates all held in the [`packages`](./packages/) directory:
- [`ballad-config`](./packages/ballad-config/): Writing, reading, serializing, and deserializing config files for all Ballad apps.
- [`ballad-search`](./packages/ballad-search/): Ballad's search anywhere implementation.
- [`ballad-services`](./packages/ballad-services/): GObject abstractions over system APIs including audio, battery, and config files.
- [`ballad-shell`](./packages/ballad-shell/): Ballad's sidebar and quick settings.

## TODO Crates

Some aspects of Ballad are a work in progress!
These are the crates that need to be implemented.

- [`ballad-greeter`](./packages/ballad-greeter): A simple greeter that discovers users using [`AccountsService`](https://www.freedesktop.org/wiki/Software/AccountsService/).
- [`ballad-settings`](./packages/ballad-settings): A full settings UI for every configurable aspect of Ballad.
- [`ballad-cli`](./packages/ballad-cli): A simple CLI tool for configuring and interacting with Ballad.

## Building

In order to build all ballad crates you need a few dependencies installed:

- `pkg-config`
- `gtk4`
- `gtk4-layer-shell`
- `glib`
- `librsvg`
- `cairo`

For the audio widget in Ballad shell to work as intended, you also need to install `alsa-lib`.

For Nix users, there is a devshell with all of these dependencies included.