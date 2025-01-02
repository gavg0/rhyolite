# Rhyolite

<img src='assets\Rhyolite_is_Cool!.png'>
A simple text editor written in Tauri, inspired by Obsidian. 

<br>


This is a **prototype**, not a fully polished version and is a fork of [fextify](https://github.com/face-hh/fextify) since the original repo was not being updated or maintained anymore. **!!CAUTION!!** The project is going under a lot of improvements, migrations and upgrades, it is advised to wait till a new binary is released and then test the project! 

# Changes include but are not limited too:-
- Rewriting the backend and frontend using Tauri and Svelte.
- Changing the default editor from ckeditor to quill(since ckeditor 5 is not free anymore.)
- ~~Maybe a renaming and icon change in future(planned).~~(Renamed to Rhyolite)

I decided to redo the whole project and continue developement on it cause I really liked the project and as a new dev I also wanted to make an open source project of my own, but before that I needed some experience so decided to contribute to this! I also renamed the project to Rhyolite(a volcanic rock just like obsidian!).

# How to run?

## Windows

Run the `Rhyolite_[version]_x64_en-US.msi` msi installer or run the `Rhyolite_[version]_x64-setup.exe` installer included in the [releases section](https://github.com/RedddFoxxyy/Rhyolite/releases).

## Mac-OS

Run the `Rhyolite_[version]_x64.dmg` or run the `Rhyolite_[version]_aarch64.dmg` for M series Macbooks, included in the [releases section](https://github.com/RedddFoxxyy/Rhyolite/releases).

## GNU/Linux

### All Linux Distros:-

Run this command on your terminal: `curl -f https://raw.githubusercontent.com/RedddFoxxyy/Rhyolite/master/packaging/linux/install.sh | sh`, it will install the app for you.

### Debian and/or Ubuntu:

Install the `.deb` version of the package from the [releases section](https://github.com/RedddFoxxyy/Rhyolite/releases).

### RHEL and/or FEDORA based linux distros:

Install the `.rpm` version of the package from the [releases section](https://github.com/RedddFoxxyy/Rhyolite/releases).

### AppImage and raw linux:

For any other GNU/Linux distribution you can use the `.AppImage` or run the `raw linux(linux_binary)` version from the [the releases section/](https://github.com/RedddFoxxyy/Rhyolite/releases).

Note: You might need to make them executable by running `chmod +x Rhyolite_[version]_.AppImage` or `chmod +x Rhyolite_[version]_linux_binary`.

# Manual compilation

For manual compilation run `npm run tauri build` in the source dir.

NOTE: if you run into any compilation error you can always debug the error with `npm run tauri build --verbose`.

# On first startup

1. Run the `fextify.exe` included in this folder. (or run `npm run tauri build -- --target x86_64-pc-windows-msvc` to compile)
2. Press `CTRL` + `P` to open the command pallet. It will give you a good idea of what's going on. Otherwise, explore!

# ~~Themes~~(soon)

### Premade

~~You can press `CTRL` + `ALT` + `S` to open the Theme Selector.~~

### Creating

~~You can duplicate the `src/themes/default.css` and modify its colors. We recommend you import the theme automatically on restart by adding `<link rel="stylesheet" href="themes/my_theme.css" />` in `src/index.html` and use `npm run tauri dev` to have the application reset on save.~~

### Publishing

You can open a pull request to add your theme in `/src/styles/themes`. We will add it if it's good. The app does not yet support multiple theme choosing, will implement that soon.

~~Alternatively, you can join our [Discord server](https://discord.gg/K6FAd8FTma) and post it on the `fextify-themes` forum!~~

# Known bugs

1. The themeing is not good, so some stuff might be hard to see.
2. Tab icon can sometimes glitch out.
3. Opening a lot of tabs can sometimes glitch the title.
