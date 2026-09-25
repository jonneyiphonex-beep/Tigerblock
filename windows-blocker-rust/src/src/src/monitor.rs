use std::process::Command;
use std::thread;
use std::time::Duration;

const INDEX_HTML: &str = include_str!("../../../../web/index.html");
const STYLES_CSS: &str = include_str!("../../../../web/styles.css");
const SCRIPT_JS: &str = include_str!("../../../../web/script.js");

pub fn run_server() {
    let server = match tiny_http::Server::http("127.0.0.1:8765") {
        Ok(server) => server,
        Err(error) => {
            eprintln!("Monitor server could not start: {}", error);
            return;
        }
    };

    println!("Live monitor: http://127.0.0.1:8765");
    for request in server.incoming_requests() {
        let (body, content_type, status) = match request.url() {
            "/" | "/index.html" => (INDEX_HTML.to_string(), "text/html; charset=utf-8", 200),
            "/styles.css" => (STYLES_CSS.to_string(), "text/css; charset=utf-8", 200),
            "/script.js" => (SCRIPT_JS.to_string(), "text/javascript; charset=utf-8", 200),
            "/api/status" => (collect_snapshot(), "application/json; charset=utf-8", 200),
            _ => ("Not found".to_string(), "text/plain; charset=utf-8", 404),
        };

        let response = tiny_http::Response::from_string(body)
            .with_status_code(status)
            .with_header(tiny_http::Header::from_bytes("Content-Type", content_type).unwrap());
        let _ = request.respond(response);
    }
}

pub fn keep_running() -> ! {
    loop {
        thread::sleep(Duration::from_secs(60));
    }
}

fn collect_snapshot() -> String {
    let script = r#"
$processes = @(Get-Process -ErrorAction SilentlyContinue)
$apps = @(Get-ItemProperty 'HKLM:\Software\Microsoft\Windows\CurrentVersion\Uninstall\*','HKLM:\Software\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall\*' -ErrorAction SilentlyContinue | Where-Object DisplayName)
$ports = @(Get-NetTCPConnection -State Listen -ErrorAction SilentlyContinue)
$usb = @(Get-PnpDevice -Class USB -Status OK -ErrorAction SilentlyContinue)
$keyboards = @(Get-PnpDevice -Class Keyboard -Status OK -ErrorAction SilentlyContinue)
$mice = @(Get-PnpDevice -Class Mouse -Status OK -ErrorAction SilentlyContinue)
$adapters = @(Get-NetAdapter -Physical -ErrorAction SilentlyContinue | Where-Object Status -eq 'Up')
$listeners = @($ports | Select-Object -First 8 LocalAddress,LocalPort,OwningProcess)
[ordered]@{
  processes = $processes.Count
  applications = $apps.Count
  listening_ports = $ports.Count
  usb_devices = $usb.Count
  keyboards = $keyboards.Count
  mice = $mice.Count
  network_adapters = $adapters.Count
  listeners = $listeners
  collected_at = (Get-Date).ToString('o')
} | ConvertTo-Json -Depth 4 -Compress
"#;

    let output = Command::new("powershell.exe")
        .args(["-NoProfile", "-NonInteractive", "-ExecutionPolicy", "Bypass", "-Command", script])
        .output();

    match output {
        Ok(output) if output.status.success() => {
            String::from_utf8_lossy(&output.stdout).trim().to_string()
        }
        Ok(output) => error_snapshot(&String::from_utf8_lossy(&output.stderr)),
        Err(error) => error_snapshot(&error.to_string()),
    }
}

fn error_snapshot(error: &str) -> String {
    format!(r#"{{"error":"{}"}}"#, error.replace('"', "\\\""))
}