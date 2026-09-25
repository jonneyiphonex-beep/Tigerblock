mod firewall;
mod monitor;
mod ui;

fn main() {
    println!("=== بدء تطبيق حظر الشبكة والمنافذ (Rust) ===");
    std::thread::spawn(monitor::run_server);

    // 1. تفعيل الحظر الشامل
    match firewall::block_all_network() {
        Ok(_) => println!("[✔] تم حظر جميع الاتصالات والمنافذ بنجاح."),
        Err(e) => {
            eprintln!("[✘] فشل الحظر الشامل (تأكد من تشغيل البرنامج كـ Administrator): {}", e);
            return;
        }
    }

    // 2. طلب موافقة فتح تطبيق معين
    let app_name = "Google Chrome";
    let app_path = r#"C:\Program Files\Google\Chrome\Application\chrome.exe"#;

    let app_prompt = format!("هل ترغب في السماح للتطبيق التالي بالاتصال بالشبكة؟\n\n{}", app_name);
    if ui::ask_permission("طلب إذن تطبيق", &app_prompt) {
        match firewall::allow_app(app_name, app_path) {
            Ok(_) => println!("[✔] تم السماح للتطبيق: {}", app_name),
            Err(e) => eprintln!("[✘] حدث خطأ أثناء إضافة إذن التطبيق: {}", e),
        }
    } else {
        println!("[!] تم رفض الطلب، التطبيق محظور حالياً.");
    }

    // 3. طلب موافقة فتح منفذ (Port) معين
    let port = 8080;
    let port_prompt = format!("هل تريد فتح المنفذ المحلي {} (TCP)؟", port);
    if ui::ask_permission("طلب فتح منفذ", &port_prompt) {
        match firewall::allow_port("Custom_Port_8080", port, "TCP") {
            Ok(_) => println!("[✔] تم فتح المنفذ {} بنجاح.", port),
            Err(e) => eprintln!("[✘] حدث خطأ أثناء فتح المنفذ: {}", e),
        }
    } else {
        println!("[!] تم رفض فتح المنفذ.");
    }

    monitor::keep_running();
}