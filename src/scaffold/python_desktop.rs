use std::path::Path;
use std::sync::mpsc::Sender;

use super::params::ScaffoldParams;
use super::writer::write_file;

pub fn scaffold(params: &ScaffoldParams, base: &Path, tx: &Sender<String>) -> Result<(), String> {
    let framework = params.sel("Framework").unwrap_or("Tkinter");

    let _ = tx.send("Writing Python desktop starter...".to_string());
    write_file(base, "README.md", &readme(framework))?;
    write_file(base, "main.py", entry(framework))?;
    write_file(base, "requirements.txt", requirements(framework))?;
    write_file(base, "Makefile", makefile())?;
    Ok(())
}

fn readme(framework: &str) -> String {
    format!("# Python Desktop\n\n- Framework: {framework}\n")
}

fn requirements(framework: &str) -> &'static str {
    match framework {
        "PyQt6" => "PyQt6\n",
        "PySide6" => "PySide6\n",
        "wxPython" => "wxPython\n",
        "Kivy" => "kivy\n",
        _ => "",
    }
}

fn entry(framework: &str) -> &'static str {
    match framework {
        "PyQt6" => {
            r#"from PyQt6.QtWidgets import QApplication, QLabel

app = QApplication([])
label = QLabel("Hello PyQt6")
label.show()
app.exec()
"#
        }
        "PySide6" => {
            r#"from PySide6.QtWidgets import QApplication, QLabel

app = QApplication([])
label = QLabel("Hello PySide6")
label.show()
app.exec()
"#
        }
        "wxPython" => {
            r#"import wx

app = wx.App(False)
frame = wx.Frame(None, title="Hello wxPython")
frame.Show()
app.MainLoop()
"#
        }
        "Kivy" => {
            r#"from kivy.app import App
from kivy.uix.label import Label

class MainApp(App):
    def build(self):
        return Label(text="Hello Kivy")

MainApp().run()
"#
        }
        _ => {
            r#"import tkinter as tk

root = tk.Tk()
root.title("Hello Tkinter")
tk.Label(root, text="Hello Tkinter").pack(padx=20, pady=20)
root.mainloop()
"#
        }
    }
}

fn makefile() -> &'static str {
    r#"PYTHON ?= python3
PIP ?= pip

.PHONY: install run test lint

install:
	@$(PIP) install -r requirements.txt

run:
	@$(PYTHON) main.py

test:
	@pytest -q

lint:
	@ruff check .
"#
}
