use qrcode::render::unicode::Dense1x2;
use qrcode::QrCode;
use std::env;
use std::io::{self, Write};
use std::process::Command;
const BLUE: &str = "\x1b[34m";
const GREEN: &str = "\x1b[32m";
const RED: &str = "\x1b[31m";
const RESET: &str = "\x1b[0m";
fn check_nmcli() -> bool {
    Command::new("which")
        .arg("nmcli")
        .output()
        .map_or(false, |output| output.status.success())
}
fn list_wifi_connections() -> Vec<String> {
    let output = Command::new("sh")
        .arg("-c")
        .arg("nmcli -f NAME connection show | grep -v NAME | grep -v lo | tr -s ' '")
        .output()
        .expect("Failed to execute nmcli command");

    let result = String::from_utf8_lossy(&output.stdout);
    result
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|ssid| !ssid.is_empty())
        .collect()
}
fn get_wifi_password(ssid: &str) -> Option<String> {
    let output = Command::new("nmcli")
        .arg("-s")
        .arg("-g")
        .arg("802-11-wireless-security.psk")
        .arg("connection")
        .arg("show")
        .arg(ssid)
        .output()
        .expect("Failed to execute nmcli command");
    let password = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if password.is_empty() {
        None
    } else {
        Some(password)
    }
}
fn generate_qr_code(ssid: &str, password: &str) {
    let qr_data = format!("WIFI:S:{};T:WPA;P:{};;", ssid, password);
    let code = QrCode::new(qr_data).unwrap();
    let image = code
        .render::<Dense1x2>()
        .dark_color(Dense1x2::Light)
        .light_color(Dense1x2::Dark)
        .build();
    println!("{}", GREEN);
    println!("{}", image);
    println!("{}", RESET);
}
fn select_wifi(ssids: &[String]) -> Option<String> {
    println!("{}Available Wi-Fi Networks:{}\n", BLUE, RESET);
    for (i, ssid) in ssids.iter().enumerate() {
        println!("{}{}. {}{}", GREEN, i + 1, ssid, RESET);
    }
    print!("\nEnter the number of the network: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    match input.trim().parse::<usize>() {
        Ok(num) if num > 0 && num <= ssids.len() => Some(ssids[num - 1].clone()),
        _ => {
            println!("{}Invalid selection!{}", RED, RESET);
            None
        }
    }
}
fn main() {
    if !check_nmcli() {
        eprintln!("{}Error: `nmcli` is not installed.{}", RED, RESET);
        return;
    }
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "--help" | "-h" => {
                println!(
                    "{}Usage:{}\n  wifi_qr            - Interactive mode\n  wifi_qr --list     - Show saved networks\n  wifi_qr \"SSID\"     - Generate QR for given network",
                    BLUE, RESET
                );
                return;
            }
            "--list" | "-l" => {
                let ssids = list_wifi_connections();
                if ssids.is_empty() {
                    println!("{}No saved Wi-Fi connections found.{}", RED, RESET);
                } else {
                    for ssid in ssids {
                        println!("{}", ssid);
                    }
                }
                return;
            }
            ssid => {
                let password = get_wifi_password(ssid);
                if let Some(pw) = password {
                    generate_qr_code(ssid, &pw);
                } else {
                    println!(
                        "{}Failed to retrieve password for `{}`.{}",
                        RED, ssid, RESET
                    );
                }
                return;
            }
        }
    }
    let ssids = list_wifi_connections();
    if ssids.is_empty() {
        println!("{}No saved Wi-Fi connections found.{}", RED, RESET);
        return;
    }
    if let Some(ssid) = select_wifi(&ssids) {
        if let Some(password) = get_wifi_password(&ssid) {
            generate_qr_code(&ssid, &password);
        } else {
            println!(
                "{}Failed to retrieve password for `{}`.{}",
                RED, ssid, RESET
            );
        }
    }
}
