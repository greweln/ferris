#!/bin/bash


# Make sure Ferris  runs only once
pid=$$
pgrep -fi /home/me/.config/ferris/start.sh | grep -v "^$pid$" | xargs -I{} kill {}

# Make sure services are running
dunst &

# Enable autorandr and set the wallpaper
autorandr -f --change


# Autoload Xresources
[[ -f ~/.Xresources ]] && xrdb -merge ~/.Xresources || echo "Failed to load .Xresources" >>  ~/logs/xresources.log
 

#Swap Caps -> Escape
xmodmap ~/.Xmodmap

# Enable tap-click for the touchpad
xinput set-prop "SynPS/2 Synaptics TouchPad" "libinput Tapping Enabled" 1


# Set the default X cursor pointer to left arrox
xsetroot -cursor_name left_ptr

# xset - counts to 600 seconds and sends an "Idle" signal to X11 if no activity is detected.
# xss-lock - hears this signal  and executes xsecurelock which locks the screen.
xset s 600 600
xss-lock --transfer-sleep-lock -- xsecurelock &
