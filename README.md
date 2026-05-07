
# 💢 angrylnk: LNK Poisoner & Persistence Tool

Demonstrates **LNK Stomping** (shortcut hijacking) used to achieve persistence and arbitrary code execution on Windows systems.

Leveraging the Windows COM, `angrylnk` modifies existing shell links (`.lnk` files) to trigger a ghost loader while maintaining the original visual identity of the shortcut. This exploits user trust and standard habits to execute payloads without raising suspicion.

⚠️ **Please Note:** This project is strictly for **Educational and Authorized Penetration Testing**. I am not responsible for any of the shenanigans you guys pull.

---

## ⚙️ How It Works

- **Target**: The tool loads a specified `.lnk` file using the `IShellLinkW` interface.
- **Telemetry**: Programmatically retrieves the original target path (e.g. `chrome.exe`) and caches it.
- **Visual Disguise**: Locks the shortcut’s icon to the original executable's path so the user sees no change on their desktop.
- **Loader Deployment**
    * Generates a VBScript (`.vbs`) in the `%APPDATA%` directory.
    * This script uses `WScript.Shell` to launch the payload in **hidden window (0)** and the original app in **normal window (1)**.
    * Marked with **Hidden (+h)** file attribute via `attrib`.
- **Execution Hijack**: The `.lnk` target is updated to point to `wscript.exe` passing the hidden loader as an argument.

---

## 🛠️ How To Run

### 1. Prerequisites
* **Rust Toolchain**: Installed via `rustup`.
* **Target Shortcut**: A `.lnk` file must exist (e.g. a shortcut to Notepad on your Desktop).

### 2. No Shortcut?
No probs, create a test shortcut using PowerShell.
```powershell
$s=(New-Object -ComObject WScript.Shell).CreateShortcut("$env:USERPROFILE\Desktop\TestApp.lnk"); $s.TargetPath="C:\Windows\System32\notepad.exe"; $s.Save()
```

### 3. Compilation
Build the project.
```cmd
cargo build --release
```

### 4. Execution
Run the binary with required arguments.

**Basic Usage**
```cmd
.\target\release\angrylnk.exe --lnk "C:\Users\Target\Desktop\TestApp.lnk"
```

**Advanced Stealth-based Usage**
```cmd
.\target\release\angrylnk.exe -l "C:\Users\Target\Desktop\Chrome.lnk" -p "powershell.exe -W Hidden -c <Payload>" -d "Google Chrome Update"
```

---

## 📈 Use Cases & Impact

* **Persistence Testing**: Researchers can use this to verify if an EDR can detect modifications to common shell links.
* **Social Engineering**: Shows how *Trusted* desktop icons can be weaponized against users.
* **Privilege Inheritance**: Research how `elevated` shortcuts (Run as Admin) pass high-integrity tokens to chained child processes.

---

## 🛡️ MITRE

| ID | Technique | Description |
| :--- | :--- | :--- |
| **T1547.009** | **Boot or Logon Autostart: Shortcut Modification** | Modifying existing shortcuts to execute commands during normal user activity. |
| **T1059.005** | **Command and Scripting Interpreter: Visual Basic** | Using `wscript.exe` and `.vbs` to execute commands. |
| **T1564.001** | **Hide Artifacts: Hidden Files and Directories** | Using `+h` attribute to hide the loader from standard file explorers. |
| **T1204.002** | **User Execution: Malicious File** | Relying on the user to click a shortcut to trigger execution. |

---

## 🚀 Improvement Ideas

* **NTFS Alternate Data Streams**: Instead of a `.vbs` file in AppData, hide the loader script inside the `.lnk` file's own hidden streams.
* **Self-Cleanup**: Implement a one-time execution mode wherein the shortcut restores itself to its original state after the payload is executed.
* **Direct Syscalls**: Use direct NT syscalls to set file attributes.

---

<p align="center">
  With ❤️ by <b>Aradhya</b>
</p>

