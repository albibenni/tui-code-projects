use std::fs;
use std::path::Path;
use std::sync::mpsc::Sender;

use super::params::ScaffoldParams;
use super::writer::write_file;

pub fn scaffold(params: &ScaffoldParams, base: &Path, tx: &Sender<String>) -> Result<(), String> {
    let ui = params.sel("UI Framework").unwrap_or("SwiftUI");
    let deployment = params.sel("iOS Deployment Target").unwrap_or("iOS 17");

    let _ = tx.send("Writing Swift iOS starter...".to_string());

    fs::create_dir_all(base.join("ios/App"))
        .map_err(|e| format!("Failed to create ios/App directory: {e}"))?;

    write_file(base, "README.md", &readme(ui, deployment))?;
    write_file(base, "ios/Config.xcconfig", &xcconfig(deployment))?;
    write_file(base, "ios/App/App.swift", app_file(ui))?;

    Ok(())
}

fn readme(ui: &str, deployment: &str) -> String {
    format!("# Swift Mobile (iOS)\n\n- UI Framework: {ui}\n- Deployment Target: {deployment}\n")
}

fn xcconfig(deployment: &str) -> String {
    format!(
        "PRODUCT_NAME = MobileApp\nPRODUCT_BUNDLE_IDENTIFIER = com.example.mobileapp\nSWIFT_VERSION = 5.9\nIPHONEOS_DEPLOYMENT_TARGET = {}\n",
        deployment_version(deployment)
    )
}

fn deployment_version(deployment: &str) -> &'static str {
    match deployment {
        "iOS 15" => "15.0",
        "iOS 16" => "16.0",
        _ => "17.0",
    }
}

fn app_file(ui: &str) -> &'static str {
    match ui {
        "UIKit" => {
            r#"import UIKit

@main
class AppDelegate: UIResponder, UIApplicationDelegate {
    var window: UIWindow?

    func application(
        _ application: UIApplication,
        didFinishLaunchingWithOptions launchOptions: [UIApplication.LaunchOptionsKey: Any]? = nil
    ) -> Bool {
        let window = UIWindow(frame: UIScreen.main.bounds)
        window.rootViewController = UIViewController()
        window.rootViewController?.view.backgroundColor = .systemBackground
        window.makeKeyAndVisible()
        self.window = window
        return true
    }
}
"#
        }
        _ => {
            r#"import SwiftUI

@main
struct MobileApp: App {
    var body: some Scene {
        WindowGroup {
            Text("Hello iOS")
        }
    }
}
"#
        }
    }
}
