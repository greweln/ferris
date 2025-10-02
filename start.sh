#!/bin/bash


# Make sure Ferris  runs only once
pid=$$
pgrep -fi /home/me/.config/ferris/start.sh | grep -v "^$pid$" | xargs -I{} kill {}

# Make sure services are running
dunst &


# Load/reload Xresources
xrdb /home/me/.Xresources

# Setup monitors
/home/me/Git/tpad/scripts/auto-setup-monitors.sh

# Autoload Xresources
[[ -f ~/.Xresources ]] && xrdb -merge ~/.Xresources || echo "Failed to load .Xresources" >>  ~/logs/xresources.log
 

#Swap Caps -> Escape
xmodmap ~/.Xmodmap

# Enable tap-click for the touchpad
xinput set-prop "SynPS/2 Synaptics TouchPad" "libinput Tapping Enabled" 1


# Set the default X cursor pointer to left arrox
xsetroot -cursor_name left_ptr

# Delete old history.tmp files.
find ~ -maxdepth 1 -name ".bash_history-*.tmp" -type f -mtime +0 -delete


# Disable urxvt bell sound
xset -b

# Wallpaper
if [ -e "/home/me/.fehbg" ]; then
    "/home/me/.fehbg" &
else
    feh --bg-fill /home/me/.config/ferris/wallpaper.png &
fi
