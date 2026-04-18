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

    let package_json = format!(
        r#"{{
  "name": "{name}",
  "version": "0.1.0",
  "main": "main.js",
  "scripts": {{
    "start": "electron ."
  }},
  "devDependencies": {{
    "electron": "latest"
  }}
}}
"#
    );

    send(tx, "Writing package.json...");
    writer::write_file(base, "package.json", &package_json)?;

    let main_js = r#"const { app, BrowserWindow } = require('electron')

function createWindow () {
  const win = new BrowserWindow({
    width: 800,
    height: 600,
    webPreferences: {
      nodeIntegration: true
    }
  })

  win.loadFile('index.html')
}

app.whenReady().then(createWindow)

app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') {
    app.quit()
  }
})

app.on('activate', () => {
  if (BrowserWindow.getAllWindows().length === 0) {
    createWindow()
  }
})
"#;

    let index_html = r#"<!DOCTYPE html>
<html>
  <head>
    <meta charset="UTF-8">
    <title>Hello Electron!</title>
  </head>
  <body>
    <h1>Hello Electron!</h1>
    <p>Build something amazing.</p>
  </body>
</html>
"#;

    send(tx, "Writing Electron starter files...");
    writer::write_file(base, "main.js", main_js)?;
    writer::write_file(base, "index.html", index_html)?;

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
