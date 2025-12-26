#!/bin/bash


# xhost +SI:localuser:me ?? lock screen

# Make sure Ferris  runs only once
pid=$$
pgrep -fi /home/me/.config/ferris/start.sh | grep -v "^$pid$" | xargs -I{} kill {}

# Make sure services are running
dunst &



# Setup monitors and wallpapers
# /home/me/Git/configs/tpad/scripts/auto-setup-monitors.sh

# Autoload Xresources
[[ -f ~/.Xresources ]] && xrdb -merge ~/.Xresources || echo "Failed to load .Xresources" >>  ~/logs/xresources.log
 

#Swap Caps -> Escape
xmodmap ~/.Xmodmap

# Enable tap-click for the touchpad
xinput set-prop "SynPS/2 Synaptics TouchPad" "libinput Tapping Enabled" 1


# Set the default X cursor pointer to left arrox
xsetroot -cursor_name left_ptr


#  xautolock detects inacivity from x11 and triggers the suspend which locks the screen due to my screen-lock service
xautolock -time 10 -locker "systemctl suspend" &
