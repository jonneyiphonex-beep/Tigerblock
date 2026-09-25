# Tigerblock

Tigerblock is a Windows firewall blocker written in Rust. The native executable currently uses Windows message boxes for permission requests.

## Web interface

The dashboard lives in [`web/index.html`](web/index.html), with styles in [`web/styles.css`](web/styles.css) and interactions in [`web/script.js`](web/script.js). When started by Rust, it reads a local monitor API and displays Windows processes, installed applications, listening ports, USB devices, keyboard and mouse device counts, and active network adapters.

## Run on Windows 10

1. Install Rust using [rustup](https://rustup.rs/) and choose the MSVC toolchain.
2. Open **PowerShell as Administrator**. The application changes Windows Firewall rules and must run elevated.
3. Build and start the native blocker:

	```powershell
	cd windows-blocker-rust\src\src\src
	cargo run --release
	```

4. Open `http://127.0.0.1:8765` after the program starts. To open only the static preview, double-click [`open-dashboard.bat`](open-dashboard.bat); the preview does not contain live Windows data.

The native program first blocks inbound and outbound traffic, then asks whether to allow Chrome and TCP port `8080`. Test it on a machine where losing network access temporarily is acceptable, and keep a second way to restore the firewall available with `netsh advfirewall reset`.

The monitor is defensive and read-only. It reports visible exposure such as listening ports; it does not record keyboard input, mouse movement, passwords, or claim to identify hidden access with certainty.