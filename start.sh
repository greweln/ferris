#!/bin/sh


# Make sure Ferris  runs only once
pid=$$
pgrep -fi /home/me/.config/ferris/start.sh | grep -v "^$pid$" | xargs -I{} kill {}

# Make sure services are running
dunst &

# Setup monitors
/home/me/scripts/auto-setup-monitors.sh


#Swap Caps -> Escape
xmodmap ~/.Xmodmap

# Enable tap-click for the touchpad
xinput set-prop "SynPS/2 Synaptics TouchPad" "libinput Tapping Enabled" 1


# Set the default X cursor pointer to left arrox
xsetroot -cursor_name left_ptr

# Wallpaper
if [ -e "/home/me/.fehbg" ]; then
    "/home/me/.fehbg" &
else
    feh --bg-fill /home/me/.config/ferris/wallpaper.png &
fi
