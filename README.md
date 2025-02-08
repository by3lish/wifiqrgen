
# WiFiQRGen

### Generate Wi-Fi QR codes directly in your terminal!

WiFiQRGen is a lightweight and efficient CLI tool that fetches your saved Wi-Fi connections, retrieves their passwords, and generates a scannable QR code in the terminal. No need to manually enter Wi-Fi credentials anymore—just scan and connect!

---

## 🔥 Features

✅ Lists saved Wi-Fi networks (supports `nmcli`)✅ Fetches Wi-Fi passwords securely✅ Generates QR codes directly in the terminal✅ Works on Linux distributions using NetworkManager

---

## 🚀 Installation

### Prerequisites

Ensure you have the following installed:

- **Rust** (for compiling the program) → [Install Rust](https://www.rust-lang.org/tools/install)
- **NetworkManager** (preinstalled in most Linux distros)

### Clone & Build

```bash
git clone https://github.com/by3lish/wifiqrgen.git  
cd wifiqrgen  
cargo build --release  
```

### Run the Program

```bash
./target/release/wifiqrgen  
```

Alternatively, install it globally:

```bash
cargo install --path .  
```

Then run:

```bash
wifiqrgen  
```

---

## 📝 Usage

Just execute the tool:

```bash
wifiqrgen  
```

You'll see a list of saved Wi-Fi networks:

```
Available saved Wi-Fi networks:
1: HomeWiFi
2: OfficeNetwork
3: CoffeeShop_Free
```

Select a network by entering its number. The program will then fetch the password and display a QR code like this:

```
█████████████████████████████
████ ▄▄▄▄▄ █ ▀▀▀▄█ ▄▄▄▄▄ ████
████ █   █ █▀ ▀ ▀█ █   █ ████
████ █▄▄▄█ ██▄  ▀█ █▄▄▄█ ████
████▄▄▄▄▄▄▄█ ▀▄▀ █▄▄▄▄▄▄▄████
████ ▄▄▄▄▄ █   ▄▄██▄ ▄ ▀▀████
████ █   █ █▀▄▄▀▄▄ ▄▄▄▄ ▄████
████ █▄▄▄█ █▄  █▄▀▄▀██▄█▀████
████▄▄▄▄▄▄▄█▄████▄█▄██▄██████
█████████████████████████████
```

Now, just scan the code with your phone and **connect instantly**! 🚀

### Other Functions

You can also use the following options:

- **Interactive Mode**:
  ```bash
  wifi_qr
  ```
  This will launch the interactive mode to select and generate QR codes for saved Wi-Fi networks.

- **List Saved Networks**:
  ```bash
  wifi_qr --list
  ```
  This will display a list of your saved Wi-Fi networks.

- **Generate QR for Specific Network**:
  ```bash
  wifi_qr "SSID"
  ```
  Replace `"SSID"` with your network's name to directly generate the QR code.

---

## 🛠️ Troubleshooting

**1. No Wi-Fi networks found?**

- Ensure `nmcli` is installed and running (`nmcli connection show`).

**2. Password not retrieved?

- You might not have permission to access network passwords. Try running:
  ```bash
  sudo wifiqrgen  
  ```
- If `nmcli` is not working, check if NetworkManager is enabled:
  ```bash
  systemctl status NetworkManager  
  ```
- Another possibility is that the network is **open (passwordless)**. In this case, no password retrieval is needed, and therefore no QR code will be generated.
**

- You might not have permission to access network passwords. Try running:
  ```bash
  sudo wifiqrgen  
  ```
- If `nmcli` is not working, check if NetworkManager is enabled:
  ```bash
  systemctl status NetworkManager  
  ```

---

## 🤝 Contributing

Pull requests are welcome! If you find bugs or have feature suggestions, open an issue.

1. Fork the project
2. Create a new branch: `git checkout -b feature-idea`
3. Commit your changes: `git commit -m "Added awesome feature"`
4. Push to your branch: `git push origin feature-idea`
5. Open a Pull Request

---

## 🐝 License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.

---

## ✨ Author

**Elish** – [GitHub](https://github.com/by3lish)💎 Email: [byelish6@gmail.com](mailto\:byelish6@gmail.com)

Give a ⭐ if you like this project! 🚀
