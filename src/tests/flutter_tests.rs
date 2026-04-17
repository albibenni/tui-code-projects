use crate::scaffold::{launch_json_for, platforms_for};

#[test]
fn platforms_map_mobile() {
    assert_eq!(platforms_for("Mobile (Android + iOS)"), "android,ios");
}

#[test]
fn platforms_map_all() {
    assert_eq!(
        platforms_for("All Platforms"),
        "android,ios,web,linux,macos,windows"
    );
}

#[test]
fn web_config_is_included_for_web_start_config() {
    let launch_json = launch_json_for("Web");
    assert!(launch_json.contains("\"deviceId\": \"chrome\""));
}

#[test]
fn web_config_is_not_included_for_mobile_start_config() {
    let launch_json = launch_json_for("Mobile (Android + iOS)");
    assert!(!launch_json.contains("\"deviceId\": \"chrome\""));
}
