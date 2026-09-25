use std::process::Command;

// تنفيذ أوامر CMD
fn run_command(cmd: &str) -> Result<String, String> {
    let output = Command::new("cmd")
        .args(["/C", cmd])
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

// 1. حظر جميع الاتصالات الواردة والصادرة
pub fn block_all_network() -> Result<(), String> {
    run_command("netsh advfirewall set allprofiles firewallpolicy blockinbound,blockoutbound")?;
    Ok(())
}

// 2. السماح لتطبيق محدد بالمرور
pub fn allow_app(app_name: &str, app_path: &str) -> Result<(), String> {
    let cmd = format!(
        r#"netsh advfirewall firewall add rule name="{}" dir=out action=allow program="{}" enable=yes"#,
        app_name, app_path
    );
    run_command(&cmd)?;
    Ok(())
}

// 3. السماح لمنفذ (Port) معين بالمرور
pub fn allow_port(rule_name: &str, port: u16, protocol: &str) -> Result<(), String> {
    let cmd = format!(
        r#"netsh advfirewall firewall add rule name="{}" dir=out action=allow protocol={} localport={} enable=yes"#,
        rule_name, protocol, port
    );
    run_command(&cmd)?;
    Ok(())
}

// 4. استعادة ضبط جدار الحماية الافتراضي
#[allow(dead_code)]
pub fn reset_firewall() -> Result<(), String> {
    run_command("netsh advfirewall reset")?;
    Ok(())
}