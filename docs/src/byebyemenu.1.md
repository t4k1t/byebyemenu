% byebyemenu(1) | General Commands Manual
% Thomas Kager
% October 2025

# NAME

byebyemenu - A minimal, customizable power menu for Wayland compositors

# SYNOPSIS

**byebyemenu**

# DESCRIPTION

A minimal, customizable power menu for Wayland compositors (such as [Niri](https://github.com/YaLTeR/niri)). Written in Rust with GTK4.

# OPTIONS

**-h**, **\--help**
: Print help information.

**-V**, **\--version**
: Print version information.

# ENVIRONMENT

**byebyemenu** can be configured using environment variables.

**BBMENU_ACTION{N}\_CMD**
: Command to execute for button N (where N is 1–6). The value is parsed as a shell command. If unset, defaults are provided for buttons 1–3:
  - 1: `/usr/bin/loginctl terminate-user $USER` (log out)
  - 2: `/usr/bin/systemctl poweroff` (shut down)
  - 3: `/usr/bin/systemctl reboot` (reboot)
  Buttons 4–6 are hidden unless both CMD and LABEL are set.

**BBMENU_ACTION{N}\_LABEL**
: Label for button N (where N is 1–6). An underscore (`_`) before a character sets a GTK mnemonic (keyboard shortcut). Defaults for buttons 1–3 are `_exit`, `_shutdown`, and `_reboot`.

**BBMENU_CSS_PATH**
: Path to a custom GTK CSS file for styling. If unset, defaults to `$XDG_CONFIG_HOME/byebyemenu/style.css` or `$HOME/.config/byebyemenu/style.css`. If missing or invalid, a built-in theme is used.

**RUST_LOG**
: Logging is configured via the RUST_LOG environment variable. Possible values are "error", "warn", "info", "debug", "trace", or "off" (and these values are case-insensitive). Defaults to "error".

# BUGS

Issue reports or feature requests can be filed at https://github.com/t4k1t/byebyemenu/issues
