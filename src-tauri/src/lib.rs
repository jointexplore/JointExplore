use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf, process::Command};
use tauri::{webview::WebviewWindowBuilder, WebviewUrl};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SiteApp {
    id: String,
    name: String,
    url: String,
    width: f64,
    height: f64,
    icon_text: String,
    resizable: bool,
    fullscreen: bool,
}

fn data_file() -> Result<PathBuf, String> {
    let exe = std::env::current_exe().map_err(|e| e.to_string())?;
    let dir = exe.parent().ok_or("无法确定程序目录")?.join("data");
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    Ok(dir.join("apps.json"))
}

fn read_apps() -> Result<Vec<SiteApp>, String> {
    let path = data_file()?;
    if !path.exists() {
        fs::write(&path, "[]").map_err(|e| e.to_string())?;
    }
    let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
    serde_json::from_str(&content).map_err(|e| e.to_string())
}

fn write_apps(apps: &[SiteApp]) -> Result<(), String> {
    let path = data_file()?;
    let content = serde_json::to_string_pretty(apps).map_err(|e| e.to_string())?;
    fs::write(path, content).map_err(|e| e.to_string())
}

#[tauri::command]
fn list_apps() -> Result<Vec<SiteApp>, String> {
    read_apps()
}

#[tauri::command]
fn save_app(app: SiteApp) -> Result<(), String> {
    if app.name.trim().is_empty() {
        return Err("应用名称不能为空".into());
    }
    let parsed = url::Url::parse(&app.url).map_err(|_| "网址格式不正确".to_string())?;
    if parsed.scheme() != "https" && parsed.scheme() != "http" {
        return Err("只允许 http:// 或 https:// 网站地址".into());
    }
    if app.width < 480.0 || app.height < 360.0 || app.width > 3840.0 || app.height > 2160.0 {
        return Err("窗口尺寸超出允许范围".into());
    }
    let mut apps = read_apps()?;
    if let Some(existing) = apps.iter_mut().find(|item| item.id == app.id) {
        *existing = app;
    } else {
        apps.push(app);
    }
    write_apps(&apps)
}

#[tauri::command]
fn delete_app(id: String) -> Result<(), String> {
    let mut apps = read_apps()?;
    apps.retain(|item| item.id != id);
    write_apps(&apps)
}

#[tauri::command]
fn launch_app(id: String) -> Result<(), String> {
    let app = read_apps()?.into_iter().find(|item| item.id == id)
        .ok_or("找不到指定应用")?;
    let exe = std::env::current_exe().map_err(|e| e.to_string())?;
    Command::new(exe)
        .arg("--container")
        .arg(app.id)
        .spawn()
        .map_err(|e| format!("启动失败：{e}"))?;
    Ok(())
}

pub fn run() {
    tauri::Builder::default()
        .setup(|app_handle| {
            let args: Vec<String> = std::env::args().collect();
            if let Some(pos) = args.iter().position(|arg| arg == "--container") {
                let id = args.get(pos + 1).ok_or("缺少应用 ID")?;
                let site = read_apps()?.into_iter().find(|item| &item.id == id)
                    .ok_or("配置中不存在该应用")?;
                let url = url::Url::parse(&site.url)?;
                WebviewWindowBuilder::new(
                    app_handle,
                    "site-window",
                    WebviewUrl::External(url),
                )
                .title(site.name)
                .inner_size(site.width, site.height)
                .resizable(site.resizable)
                .fullscreen(site.fullscreen)
                .build()?;
            } else {
                WebviewWindowBuilder::new(
                    app_handle,
                    "main",
                    WebviewUrl::App("index.html".into()),
                )
                .title("Desktop Container Manager")
                .inner_size(1120.0, 760.0)
                .min_inner_size(760.0, 560.0)
                .resizable(true)
                .build()?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            list_apps,
            save_app,
            delete_app,
            launch_app
        ])
        .run(tauri::generate_context!())
        .expect("Desktop Container Manager 启动失败");
}
