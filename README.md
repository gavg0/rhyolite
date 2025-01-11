# Rhyolite

<img src='assets\Rhyolite_is_cool_2.png'>
<img src='assets\Rhyolite_is_cool_3.png'>

## A simple text editor written in Tauri, inspired by Obsidian. 


This is a **prototype**, not a fully polished version. **!!CAUTION!!** The project is going under a lot of improvements, migrations and upgrades, it is advised to wait till a new binary is released and then test the project! If you want to compile the project from source clone from the master-v0.1.4-old branch.

# Changes include but are not limited too:-
- Rewriting the backend and frontend using Rust and Svelte.
- Changing the default editor from ckeditor to quill(since ckeditor 5 is not free anymore.)
- ~~Maybe a renaming and icon change in future(planned).~~(Renamed to Rhyolite)

I decided to redo the whole project and continue developement on it cause I really liked the project and as a new dev I also wanted to make an open source project of my own, but before that I needed some experience so decided to contribute to this! I also renamed the project to Rhyolite(a volcanic rock just like obsidian!).

# For New Contributers:-
- The source code in the master branch went through a massive restructuring and as a result we now have a default styling of the Flowbite/Tiptap editor and will not look the same as portrayed in the image. If you want to compile the source for latest release of the app I will suggest that you compile the source from master-v0.1.4-old branch instead.
- After we restyle the UI again, the app will look similar to v0.1.4!
- If you want to work on the latest version of the app, I will suggest that you join discord!

# How to run?

## Windows

### For 64bit systems:
Run the `Rhyolite_[version]_x64_en-US.msi` msi installer or run the `Rhyolite_[version]_x64-setup.exe` installer included in the [releases section](https://github.com/RedddFoxxyy/Rhyolite/releases).

### For 32bit systems:
Run the `Rhyolite_[version]_x86_en-US.msi` msi installer or run the `Rhyolite_[version]_x86-setup.exe` installer included in the [releases section](https://github.com/RedddFoxxyy/Rhyolite/releases).

### For Arm64(snapdragon processors) systems:
Run the `Rhyolite_[version]_arm64_en-US.msi` msi installer or run the `Rhyolite_[version]_arm64-setup.exe` installer included in the [releases section](https://github.com/RedddFoxxyy/Rhyolite/releases).

## Mac-OS (You might face issues opening the mac version since the app is not signed.)

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

1. Run the `Rhyolite.exe` included in this folder. (or run `npm run tauri build` to compile)
2. Press `CTRL` + `P` to open the command pallet. It will give you a good idea of what's going on. Otherwise, explore!

~~Alternatively, you can join our [Discord server](https://discord.gg/K6FAd8FTma) and post it on the `fextify-themes` forum!~~

# Known bugs

1. The themeing is not good, so some stuff might be hard to see.
2. Tab icon can sometimes glitch out.
3. Opening a lot of tabs can sometimes glitch the title.

# Licensing

```
Copyright 2025 RedddFoxxyy

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
```
