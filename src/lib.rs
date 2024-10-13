use std::{io, process::Command};

use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};

#[derive(Copy, Clone)]
pub enum Game {
    Genshin,
    WutheringWaves,
}

/// Find the game executable in the registry and start the game.
///
/// # Example
///
/// ```rust
/// use start_gaming::{find_exe, Game};
///
/// fn main() {
///    match find_exe(Game::Genshin) {
///       Ok(_) => println!("Game Found!"),
///      Err(_) => println!("Game Not Found!"),
///   };
/// }
/// ```
///
/// # Errors
///
/// This function will return an error if the game is not found in the registry.
///
/// # Panics
///
/// This function will return an error if the game executable is not found.
///
/// # Safety
///
/// This function is not marked as unsafe, but it is not safe to use in a concurrent environment.
///
/// # Note
///
/// This function is only available on Windows.
///
/// # Platform-specific
///
/// This function is only available on Windows.
///

pub fn find_exe(game: Game) -> io::Result<()> {
    let reg_key = RegKey::predef(HKEY_LOCAL_MACHINE);
    let uninstall_key =
        reg_key.open_subkey(r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall")?;

    for sub_key_name in uninstall_key.enum_keys().map(|x| x.unwrap()) {
        let sub_key = uninstall_key.open_subkey(&sub_key_name)?;
        match information(sub_key, game) {
            Some(_) => return Ok(()),
            None => continue,
        };
    }

    let uninstall_key =
        reg_key.open_subkey(r"SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall")?;

    for sub_key_name in uninstall_key.enum_keys().map(|x| x.unwrap()) {
        let sub_key = uninstall_key.open_subkey(&sub_key_name)?;
        match information(sub_key, game) {
            Some(_) => return Ok(()),
            None => continue,
        };
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "Game not found in registry",
    ))
}

fn information(sub_key: RegKey, game: Game) -> Option<()> {
    if let Ok(display_name) = sub_key.get_value::<String, _>("DisplayName") {
        match game {
            Game::Genshin => {
                if display_name.eq("原神") {
                    println!("{}", display_name);
                    match sub_key.get_value::<String, _>("InstallPath") {
                        Ok(install_path) => {
                            println!("InstallPath: {}", install_path);
                            Command::new(format!(
                                "{}/Genshin Impact Game/YuanShen.exe",
                                install_path
                            ))
                            .spawn()
                            .expect("Failed to execute command");
                        }
                        Err(_) => println!("InstallPath: None"),
                    };
                    match sub_key.get_value::<String, _>("InstallLocation") {
                        Ok(install_location) => println!("InstallLocation : {}", install_location),
                        Err(_) => println!("InstallLocation : None"),
                    };
                    match sub_key.get_value::<String, _>("UninstallString") {
                        Ok(uninstall_string) => println!("UninstallString : {}", uninstall_string),
                        Err(_) => println!("UninstallString : None"),
                    };
                    return Some(());
                }
            }
            Game::WutheringWaves => {
                if display_name.eq("鸣潮") {
                    println!("{}", display_name);
                    match sub_key.get_value::<String, _>("InstallPath") {
                        Ok(install_path) => {
                            println!("InstallPath: {}", install_path);
                            Command::new(format!(
                                "{}/Wuthering Waves Game/Wuthering Waves.exe",
                                install_path
                            ))
                            .spawn()
                            .expect("Failed to execute command");
                        }
                        Err(_) => println!("InstallPath: None"),
                    };
                    match sub_key.get_value::<String, _>("InstallLocation") {
                        Ok(install_location) => println!("InstallLocation : {}", install_location),
                        Err(_) => println!("InstallLocation : None"),
                    };
                    match sub_key.get_value::<String, _>("UninstallString") {
                        Ok(uninstall_string) => println!("UninstallString : {}", uninstall_string),
                        Err(_) => println!("UninstallString : None"),
                    };
                    return Some(());
                }
            }
        };
    }
    None
}
