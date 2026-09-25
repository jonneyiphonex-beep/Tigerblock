const protectionToggle = document.querySelector("#protection-toggle");
const toggleLabel = document.querySelector("#toggle-label");
const statusLabel = document.querySelector("#status-label");
const toast = document.querySelector("#toast");
const liveMonitor = window.location.protocol === "http:" || window.location.protocol === "https:";
let protectionEnabled = true;
let toastTimer;

function showToast(message) {
  toast.textContent = message;
  toast.classList.add("show");
  clearTimeout(toastTimer);
  toastTimer = setTimeout(() => toast.classList.remove("show"), 2600);
}

protectionToggle.addEventListener("click", () => {
  protectionEnabled = !protectionEnabled;
  protectionToggle.setAttribute("aria-pressed", String(protectionEnabled));
  toggleLabel.textContent = protectionEnabled ? "Protection active" : "Protection paused";
  statusLabel.textContent = protectionEnabled ? "locked down" : "open to connections";
  protectionToggle.querySelector(".button-indicator").style.background = protectionEnabled ? "#65d2ad" : "#f1c985";
  showToast(protectionEnabled ? "Firewall protection enabled" : "Firewall protection paused");
});

document.querySelector("#add-app").addEventListener("click", () => showToast("Application picker is ready for Rust integration"));
document.querySelector("#add-port").addEventListener("click", () => showToast("Port approval is ready for Rust integration"));
document.querySelector("#clear-activity").addEventListener("click", () => {
  document.querySelector("#activity-list").innerHTML = "<p><span class=\"activity-time\">Now</span><span class=\"activity-dot allowed\"></span><span>Activity log cleared</span></p>";
  showToast("Activity cleared");
});

document.querySelectorAll(".remove-button").forEach((button) => {
  button.addEventListener("click", () => {
    button.closest(".access-row").remove();
    showToast("Access rule removed from this view");
  });
});

function setText(id, value) {
  const element = document.querySelector(`#${id}`);
  if (element) element.textContent = value;
}

function renderSnapshot(snapshot) {
  if (snapshot.error) {
    setText("last-updated", "Monitor unavailable");
    document.querySelector("#risk-result strong").textContent = "PowerShell unavailable";
    return;
  }
  setText("process-count", snapshot.processes ?? "--");
  setText("app-count", snapshot.applications ?? "--");
  setText("port-count", snapshot.listening_ports ?? "--");
  setText("usb-count", snapshot.usb_devices ?? "--");
  setText("keyboard-count", snapshot.keyboards ?? "--");
  setText("mouse-count", snapshot.mice ?? "--");
  setText("network-count", snapshot.network_adapters ?? "--");
  setText("last-updated", `Updated ${new Date(snapshot.collected_at).toLocaleTimeString()}`);
  const riskResult = document.querySelector("#risk-result strong");
  riskResult.textContent = snapshot.listening_ports > 0 ? `${snapshot.listening_ports} listener(s) to review` : "No listening ports found";
  document.querySelector("#risk-result").classList.toggle("warning", snapshot.listening_ports > 0);
}

async function refreshMonitor() {
  if (!liveMonitor) return;
  try {
    const response = await fetch("/api/status", { cache: "no-store" });
    renderSnapshot(await response.json());
  } catch (error) {
    setText("last-updated", "Monitor server offline");
  }
}

refreshMonitor();
if (liveMonitor) window.setInterval(refreshMonitor, 5000);