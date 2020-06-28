use std::collections::HashSet;
use std::path::Path;
use serde::{Serialize, Deserialize};

type AnyError = Box<dyn std::error::Error>;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Config {
    path: String,
    version: HashSet<String>,
    show: bool
}

impl Config {
    fn new() -> Config {
        return Config { 
            path: String::new(),
            version: HashSet::new(),
            show: true,
        }
    }
}

fn write_config(config:&Config, config_path:& Path) -> Result<(), AnyError> {
    use winreg::enums::*;
    use winreg::RegKey;
    
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (key, _) = hkcu.create_subkey(&config_path)?;
    let config_str = serde_yaml::to_string(&config).unwrap();
    key.set_value("config", &config_str)?;

    println!("reg write");
    Ok(())
}

fn read_config(config_path: &Path) -> Result<Config, AnyError> {
    use winreg::enums::*;
    use winreg::RegKey;

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (key, _) = hkcu.create_subkey(&config_path)?;

    let config_str: String = key.get_value("config")?;

    println!("reg read");
    Ok(serde_yaml::from_str(&config_str)?)
}

fn main() -> Result<(), AnyError> {
    let config_path = Path::new("Software\\Bitwiglauncher");

    // load config
    let mut reset_count = 0;
    let mut config: Config;
    loop {
        match read_config(config_path) {
            Ok(c) => {
                config = c;
            }
            Err(e) => {
                if reset_count < 1 { // reset only once
                    let default_config = Config::new();
                    write_config(&default_config, config_path)?;
                    reset_count += 1;
                    continue;
                } else {
                    return Err(e);
                }
            }
        }
        break;
    }

    run(&mut config);
    write_config(&config, &config_path)?;
    Ok(())
}

fn run(config: &mut Config) {
    // collect all available versions
    let mut versions: HashSet<String> = HashSet::new();

    if let Ok(paths) = std::fs::read_dir(&config.path) {
        for path in paths {
            let mut buf = path.unwrap().path();
            let p = buf.as_path();
            
            if p.exists() && p.is_dir() { // check directory
                if let Some(name) = p.file_name() { // extract directory name
                    let version_str = name.to_string_lossy().to_string();

                    buf.push("Bitwig Studio.exe"); // check Bitwig Studio.exe exists
                    if buf.exists() && buf.is_file() {
                        versions.insert(version_str);
                    }
                }
            }
        }
        
        // when no version is found prompt path selection
        if versions.len() == 0 {

        }
    } else {
        // TODO: show folder picker
    }

    // detect difference
    if config.version == versions {
        dbg!(true);
    }
}
