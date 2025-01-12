# Rhyolite

![Rhyolite Logo](assets/Rhyolite_is_cool_2.png)  
![Rhyolite Editor Preview](assets/Rhyolite_is_cool_3.png)

## A Simple Text Editor Built with Tauri!  
*Inspired by Obsidian.*

---

**This is a prototype.** Please note that **major changes are ongoing**. For the most stable experience, wait for a new binary release or compile the source from the `master-v0.1.4-old` branch.

---

## **Current Updates**  
### Changes Underway:  
- Frontend is being restyled to match v0.1.4 after we restructured the frontend code.  
- Working on a custom markdown engine using html2md crate.  
- Project renamed to **Rhyolite**.

> Rhyolite is inspired by volcanic rocks, much like Obsidian. The project is my way to contribute and grow as a developer while sharing something meaningful with the open-source community.

---

## For New Contributors  
1. The `master` branch has undergone **massive restructuring**.  
2. The app now uses **Flowbite/Tiptap editor styling**, which differs from earlier visuals.  
3. For the latest stable release visuals, compile the source from the `master-v0.1.4-old` branch.  
4. Join our **[Discord server](https://discord.gg/K6FAd8FTma)** to collaborate effectively.

---

## How to Install?  

### **Windows**  
#### 64-bit Systems:  
- Use the `Rhyolite_[version]_x64_en-US.msi` or `Rhyolite_[version]_x64-setup.exe` installer from the [Releases section](https://github.com/RedddFoxxyy/Rhyolite/releases).

#### 32-bit Systems:  
- Use the `Rhyolite_[version]_x86_en-US.msi` or `Rhyolite_[version]_x86-setup.exe` installer.

#### ARM64 Systems (Snapdragon processors):  
- Use the `Rhyolite_[version]_arm64_en-US.msi` or `Rhyolite_[version]_arm64-setup.exe`.

---

### **MacOS**  
- Use the `Rhyolite_[version]_x64.dmg` for Intel Macs.  
- Use the `Rhyolite_[version]_aarch64.dmg` for M-series Macs.

> **Note:** You may encounter issues since the app isn’t signed yet.

---

### **Linux**  
#### Universal Linux Installer:  
Run this command in your terminal:  
```bash
curl -f https://raw.githubusercontent.com/RedddFoxxyy/Rhyolite/master/packaging/linux/install.sh | sh
```

#### Debian/Ubuntu:
- Install the .deb package from the Releases section.

#### RHEL/Fedora:
- Install the .rpm package from the same section.

#### AppImage/Raw Binary:
Make the file executable and run:
```bash
chmod +x Rhyolite_[version]_.AppImage
./Rhyolite_[version]_.AppImage
```

---

### **Manual Compilation**
- To build the app manually, run:
```bash
npm run tauri build
```
- For debugging, add `--verbose`:
```bash
npm run tauri build --verbose
```

> **Note:** You may face errors when using package manager other than NPM.

---

## First Startup

1. Run Rhyolite.exe or compile using npm run tauri build.
2. Open the Command Palette using CTRL + P.
3. Explore the features or refer to our Discord community for guidance.

## Known Bugs

1. Theming might cause visibility issues.
2. Tab icons occasionally glitch.
3. Large numbers of open tabs can distort the title.
