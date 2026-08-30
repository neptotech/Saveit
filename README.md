# SaveIt
[Skip Images](#target-anchor)
![alt text](/images/image.png)
![alt text](/images/image.png)
![alt text](/images/image-1.png)
![alt text](/images/image-3.png)
![alt text](/images/image-4.png)
<a id="target-anchor"></a>
SaveIt is a fast, local-first desktop inbox for keeping text, rich content, links, images, and files organised in one place. Drop content onto the home screen, paste from the clipboard, or write a note in the editor, and save them into a folder.

Built with Rust and Tauri 2. [Thus light weight *8MB* portable app or *2MB installer*!]

**Flashback:** Made to store my college documents, notes, resources, informations etc organised into folders which are course names in my case, From widespread sources like *email,google groups,google classroom,drive,external links*. Free notion can have only 5mb file and free onenote only 100mb. But here the files are cleanly stored in a folder and used,and all the notes in a json file. Both can be setup to be backed up in onedrive(default). And I can thus use it with both laptop and pc using same onedrive...

## Features

- Drag and drop text, rich HTML, links, images, and files onto the home screen
- Drop files into the editor at the mouse-release position
- Upload files through the editor toolbar
- Store notes and copied files in the user's OneDrive Documents folder when available
- Fall back to the user's local Documents folder when OneDrive Documents is unavailable
- Change the storage root from the Settings menu; SaveIt copies existing data before reloading
- Organize notes with folders and drag notes between folders
- Search, filter, sort, and switch between grid and list views
- Edit rich text with formatting, links, images, colors, and highlights
- Open web links in the default browser
- Open saved files with their Windows file association or Open With dialog
- Multi-select notes with Ctrl/Cmd-click and Shift-click

## Download

For Windows, use one of the generated installers from the release assets: [Get it here](https://github.com/neptotech/Saveit/releases/latest/)

- `SaveIt_1.0.0_x64-setup.exe` for the NSIS installer
- `SaveIt_1.0.0_x64_en-US.msi` for the MSI installer

The standalone portable executable is `saveit.exe`.

## Requirements

- Windows 10 or later
- Microsoft Edge WebView2 Runtime
- Node.js and npm for development
- Rust and Cargo for development and building

SaveIt uses `%USERPROFILE%\OneDrive\Documents` by default when that folder exists, otherwise `%USERPROFILE%\Documents`. The application creates `SaveItFiles` when the first file is imported.

## Development

Install the JavaScript dependencies:

```powershell
npm install
```

Start the Tauri development application:

```powershell
npm run dev
```

Check the Rust backend:

```powershell
cargo check --manifest-path src-tauri\Cargo.toml
```

Run Rust tests:

```powershell
cargo test --manifest-path src-tauri\Cargo.toml
```

## Build

Create the optimized executable and Windows installers:

```powershell
npm run build
```

Build output is written to:

```text
src-tauri/target/release/saveit.exe
src-tauri/target/release/bundle/nsis/SaveIt_1.0.0_x64-setup.exe
src-tauri/target/release/bundle/msi/SaveIt_1.0.0_x64_en-US.msi
```

## Data and file storage

SaveIt keeps the main document at `<storage-root>\saveit.json`, where `<storage-root>` is selected automatically or in Settings.

```text
<storage-root>\saveit.json
```

Imported files are copied into:

```text
<storage-root>\SaveItFiles\
```

Notes reference the copied file rather than the original source location. A storage-path change only succeeds when the destination is new or empty; existing data is copied before the app reloads. Saved-file opening is restricted to the active `SaveItFiles` directory.

## Project layout

```text
frontend/index.html       Application UI and client-side behavior
src-tauri/src/lib.rs      Rust commands for JSON and file storage
src-tauri/src/main.rs     Tauri application entry point
src-tauri/tauri.conf.json Tauri window and bundle configuration
src-tauri/icons/          Application icon assets
```

## License

This project is licensed under the GNU General Public License, version 2.0 only. See [LICENSE](LICENSE) for the full license text.
