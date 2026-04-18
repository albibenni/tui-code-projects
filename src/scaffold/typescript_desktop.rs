use std::path::Path;
use std::sync::mpsc::Sender;

use super::command::run_in;
use super::params::ScaffoldParams;
use super::writer;

pub fn scaffold(params: &ScaffoldParams, base: &Path, tx: &Sender<String>) -> Result<(), String> {
    let framework = params.sel("Framework").unwrap_or("Electron");
    let pm = params.sel("Package Manager").unwrap_or("npm");

    match framework {
        "Electron" => scaffold_electron(params, base, pm, tx),
        "NeutralinoJS" => scaffold_neutralino(base, pm, tx),
        _ => Ok(()),
    }
}

fn scaffold_electron(
    params: &ScaffoldParams,
    base: &Path,
    pm: &str,
    tx: &Sender<String>,
) -> Result<(), String> {
    let name = &params.project_name;

    // 1. Create directory structure
    let src_dir = base.join("src");
    std::fs::create_dir_all(&src_dir).map_err(|e| format!("Failed to create src/: {e}"))?;

    // 2. package.json
    let package_json = format!(
        r#"{{
  "name": "{name}",
  "version": "0.1.0",
  "main": "dist/main.js",
  "scripts": {{
    "build": "tsc",
    "watch": "tsc -w",
    "start": "npm run build && electron .",
    "dev": "concurrently \"npm run watch\" \"electron .\""
  }},
  "devDependencies": {{
    "electron": "latest",
    "typescript": "latest",
    "concurrently": "latest"
  }}
}}
"#
    );

    // 3. tsconfig.json
    let tsconfig = r#"{
  "compilerOptions": {
    "target": "ESNext",
    "module": "CommonJS",
    "outDir": "./dist",
    "rootDir": "./src",
    "strict": true,
    "esModuleInterop": true,
    "skipLibCheck": true,
    "forceConsistentCasingInFileNames": true
  },
  "include": ["src/**/*"]
}
"#;

    // 4. src/main.ts
    let main_ts = r#"import { app, BrowserWindow } from 'electron';
import * as path from 'path';

function createWindow() {
  const win = new BrowserWindow({
    width: 800,
    height: 600,
    webPreferences: {
      preload: path.join(__dirname, 'preload.js'),
      contextIsolation: true,
      nodeIntegration: false
    }
  });

  win.loadFile('index.html');
}

app.whenReady().then(createWindow);

app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') app.quit();
});

app.on('activate', () => {
  if (BrowserWindow.getAllWindows().length === 0) createWindow();
});
"#;

    // 5. src/preload.ts (The secure bridge)
    let preload_ts = r#"import { contextBridge } from 'electron';

contextBridge.exposeInMainWorld('electronAPI', {
  platform: process.platform,
  version: process.versions.electron
});
"#;

    // 6. index.html
    let index_html = r#"<!DOCTYPE html>
<html>
  <head>
    <meta charset="UTF-8">
    <title>Hello Electron + TS!</title>
    <style>
      body { font-family: sans-serif; text-align: center; padding-top: 50px; }
    </style>
  </head>
  <body>
    <h1>Hello Electron + TypeScript!</h1>
    <p>Security: Preload script is active.</p>
    <div id="info"></div>
    <script>
      const info = document.getElementById('info');
      info.innerText = `Running on ${window.electronAPI.platform} (Electron v${window.electronAPI.version})`;
    </script>
  </body>
</html>
"#;

    send(tx, "Writing configuration and source files...");
    writer::write_file(base, "package.json", &package_json)?;
    writer::write_file(base, "tsconfig.json", tsconfig)?;
    writer::write_file(base, "index.html", index_html)?;
    writer::write_file(&src_dir, "main.ts", main_ts)?;
    writer::write_file(&src_dir, "preload.ts", preload_ts)?;

    send(tx, format!("Running {pm} install..."));
    let (prog, args): (&str, &[&str]) = match pm {
        "pnpm" => ("pnpm", &["install"]),
        "yarn" => ("yarn", &[]),
        "bun" => ("bun", &["install"]),
        _ => ("npm", &["install"]),
    };
    run_in(base, prog, args, tx)
}

fn scaffold_neutralino(base: &Path, pm: &str, tx: &Sender<String>) -> Result<(), String> {
    send(tx, "Running npx neu create...");
    // Neutralino usually creates a subfolder, we might need to handle that or use flags
    // For simplicity, we can use the manual approach if neu create is too complex to target .
    run_in(base, "npx", &["@neutralinojs/neu", "create", ".", "--template", "default"], tx)?;
    
    send(tx, format!("Running {pm} install..."));
    let (prog, args): (&str, Vec<&str>) = match pm {
        "pnpm" => ("pnpm", vec!["install"]),
        "yarn" => ("yarn", vec!["install"]),
        "bun" => ("bun", vec!["install"]),
        _ => ("npm", vec!["install"]),
    };
    run_in(base, prog, &args, tx)
}

fn send(tx: &Sender<String>, msg: impl Into<String>) {
    let _ = tx.send(msg.into());
}
