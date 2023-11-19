use anyhow::{anyhow, Result};
use std::path::PathBuf;
use winreg::enums::*;
use winreg::RegKey;

fn get_wk_dir() -> Result<PathBuf> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let win_kits_key = r"SOFTWARE\Microsoft\Windows Kits\Installed Roots";
    let key = hklm.open_subkey(win_kits_key)?;
    let value: String = key.get_value("KitsRoot10")?;

    Ok(value.into())
}

fn get_km_dir(windows_kits_dir: &PathBuf) -> Result<PathBuf> {
    match windows_kits_dir.join("lib").read_dir() {
        Ok(read_dir) => {
            match read_dir
                .filter_map(|dir| dir.ok())
                .map(|dir| dir.path())
                .filter(|dir| {
                    dir.components()
                        .last()
                        .and_then(|c| c.as_os_str().to_str())
                        .map(|c| c.starts_with("10.") && dir.join("km").is_dir())
                        .unwrap_or(false)
                })
                .max()
                .ok_or_else(|| {
                    format!(
                        "Can not find a valid kernel modules directory in `{:?}`",
                        windows_kits_dir
                    )
                }) {
                Ok(max_lib_dir) => Ok(max_lib_dir.join("km")),
                Err(msg) => Err(anyhow!(msg)),
            }
        }
        Err(_) => Err(anyhow!("Could not read directory: {:?}", windows_kits_dir)),
    }
}

fn main() -> Result<()> {
    let wk_dir = get_wk_dir()?;
    let km_dir = get_km_dir(&wk_dir)?;

    let target = std::env::var("TARGET")?;
    let arch = if target.contains("x86_64") {
        "x64"
    } else if target.contains("i686") {
        "x86"
    } else {
        panic!("Only support x64 and x86");
    };
    let wdk_lib_dir = km_dir.join(arch);

    println!(
        "cargo:rustc-link-search=native={}",
        wdk_lib_dir.to_string_lossy()
    );

    Ok(())
}
