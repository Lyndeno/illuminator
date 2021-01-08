# autoddc (better name pending)

The purpose of this project is to allow proper time-based automatic brightness adjustments on desktops (or laptops) with external monitors.

The usage of this program will aim to be similar to the execution of utilities like [Redshift](https://github.com/jonls/redshift) and [Gammastep](https://gitlab.com/chinstrap/gammastep), the main difference being that this utility adjusts brightness and not screen temperature. The goal is that you will call this program at startup with minimal arguments and get the functionality.

## Requirements

This program requires that you have loaded the ```i2c-dev``` kernel module loaded on your Linux system. You will also need access to ```/dev/i2c-*``` from your user, [this section of my guide](https://lyndeno.ca/posts/setting-up-external-monitor-brightness/#permitting-user-access-to-devi2c-) will show you how to do that.

## To-do

- [ ] Automatic sunset and sunrise detection based on location
- [ ] Take input arguments (latitude, longitude, day brightness, and night brightness)
- [ ] Implement custom transition periods for smooth adjustments
- [ ] Maybe a GTK front-end?
- [ ] Maybe automatic location detection?
