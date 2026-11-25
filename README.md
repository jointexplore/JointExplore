# Desktop Container Manager 1.0

Windows 10/11 x64 portable website-container manager. One executable runs both the manager and all configured website windows. Each website has an independent configuration entry.

## What it does

- Add, edit, delete and launch website entries.
- Open each website in a separate native desktop window without browser tabs or address bar.
- Configure window width, height, resizability and fullscreen.
- Store configuration beside the executable in `data/apps.json`.
- Use one shared executable for all websites.

## Portable use

1. Extract `DesktopContainerManager-Windows-x64-Portable.zip`.
2. Keep `DesktopContainerManager.exe` and the `data` folder together.
3. Run `DesktopContainerManager.exe`.
4. Click **添加网站**, enter a name and an `https://` or `http://` URL, then save.
5. Click **启动应用** to open the website in its own window.

The app writes its configuration into `data/apps.json`. Keep the folder writable and back up this file to preserve your entries.

## Runtime requirement

The target Windows computer needs Microsoft Edge WebView2 Runtime. Many current Windows 10 systems already have it. If a website does not open and WebView2 is missing, install the Evergreen Runtime from Microsoft:
https://developer.microsoft.com/microsoft-edge/webview2/

## Build without installing development tools locally

This repository includes `.github/workflows/build-windows.yml`. Upload the source folder to a GitHub repository, open **Actions**, run **Build Windows portable app**, and download the generated ZIP from the workflow artifacts. GitHub-hosted Windows runners compile the executable.

## Security and limitations in 1.0

- Website windows load remote websites. They do not receive Tauri filesystem or shell permissions.
- Only HTTP and HTTPS URLs are accepted.
- This is a web-site wrapper, not a full browser. Some websites may block embedded WebView sign-in, popups, DRM, extensions or third-party cookies.
- The icon text is currently used for the manager's app tile. Per-site Windows shortcuts and custom executable icons are not included in this first build.
- This starter source has not yet been compiled or runtime-tested in this environment. Build validation should be completed by the Windows GitHub Actions workflow before distribution.
