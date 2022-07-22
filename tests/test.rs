use std::{path::PathBuf, process::Command};

use special_folder::SpecialFolder;

fn run_pwsh(arg: impl Into<String>) -> Result<String, Box<dyn std::error::Error>> {
    let mut cmd = Command::new("pwsh");

    let pwsh_stdin = format!("[System.Environment]::GetFolderPath('{}')", arg.into());
    cmd.args(&["-NoProfile", "-NoLogo", "-Command", pwsh_stdin.as_str()]);

    Ok(String::from_utf8(cmd.output()?.stdout)?.trim().to_string())
}

#[test]
fn test_admin_tools() -> Result<(), Box<dyn std::error::Error>> {
    let path = SpecialFolder::AdminTools.get();
    assert!(path.is_some());
    let path1 = path.unwrap();
    let path2: PathBuf = run_pwsh("AdminTools")?.into();

    assert_eq!(path1, path2);
    Ok(())
}

#[test]
fn test_application_data() -> Result<(), Box<dyn std::error::Error>> {
    let path = SpecialFolder::ApplicationData.get();
    assert!(path.is_some());
    let path1 = path.unwrap();
    let path2: PathBuf = run_pwsh("ApplicationData")?.into();

    assert_eq!(path1, path2);
    Ok(())
}

#[test]
#[ignore = "recyle bin path is not supported"]
fn test_recycle_bin() -> Result<(), Box<dyn std::error::Error>> {
    let path = SpecialFolder::RecycleBin.get();
    assert!(path.is_some());
    let path1 = path.unwrap();
    let path2: PathBuf = run_pwsh("RecycleBin")?.into();

    assert_eq!(path1, path2);
    Ok(())
}

#[test]
fn test_cd_burning() -> Result<(), Box<dyn std::error::Error>> {
    let path = SpecialFolder::CDBurning.get();
    assert!(path.is_some());
    let path1 = path.unwrap();
    let path2: PathBuf = run_pwsh("CDBurning")?.into();

    assert_eq!(path1, path2);
    Ok(())
}

#[test]
fn test_common_admin_tools() -> Result<(), Box<dyn std::error::Error>> {
    let path = SpecialFolder::CommonAdminTools.get();
    assert!(path.is_some());
    let path1 = path.unwrap();
    let path2: PathBuf = run_pwsh("CommonAdminTools")?.into();

    assert_eq!(path1, path2);
    Ok(())
}

#[test]
fn test_common_app_data() -> Result<(), Box<dyn std::error::Error>> {
    let path = SpecialFolder::CommonAppData.get();
    assert!(path.is_some());
    let path1 = path.unwrap();
    let path2: PathBuf = run_pwsh("CommonApplicationData")?.into();

    assert_eq!(path1, path2);
    Ok(())
}

#[test]
fn test_common_desktop_directory() -> Result<(), Box<dyn std::error::Error>> {
    let path = SpecialFolder::CommonDesktopDirectory.get();
    assert!(path.is_some());
    let path1 = path.unwrap();
    let path2: PathBuf = run_pwsh("CommonDesktopDirectory")?.into();

    assert_eq!(path1, path2);
    Ok(())
}

#[test]
fn test_common_documents() -> Result<(), Box<dyn std::error::Error>> {
    let path = SpecialFolder::CommonDocuments.get();
    assert!(path.is_some());
    let path1 = path.unwrap();
    let path2: PathBuf = run_pwsh("CommonDocuments")?.into();

    assert_eq!(path1, path2);
    Ok(())
}

#[test]
#[ignore]
fn test_common_favorites() -> Result<(), Box<dyn std::error::Error>> {
    let path = SpecialFolder::CommonFavorites.get();
    assert!(path.is_some());
    let path1 = path.unwrap();
    let path2: PathBuf = run_pwsh("CommonFavorites")?.into();

    assert_eq!(path1, path2);
    Ok(())
}

#[test]
fn test_common_music() -> Result<(), Box<dyn std::error::Error>> {
    let path = SpecialFolder::CommonMusic.get();
    assert!(path.is_some());
    let path1 = path.unwrap();
    let path2: PathBuf = run_pwsh("CommonMusic")?.into();

    assert_eq!(path1, path2);
    Ok(())
}

#[test]
#[ignore = "common oem folder is not supported"]
fn test_common_oem_links() -> Result<(), Box<dyn std::error::Error>> {
    let path = SpecialFolder::CommonOemLinks.get();
    assert!(path.is_some());
    let path1 = path.unwrap();
    let path2: PathBuf = run_pwsh("CommonOemLinks")?.into();

    assert_eq!(path1, path2);
    Ok(())
}
