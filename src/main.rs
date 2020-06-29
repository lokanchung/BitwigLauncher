// #![windows_subsystem = "windows"]

use std::collections::HashSet;
use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};

type AnyError = Box<dyn std::error::Error>;

// ---- CONFIG ----

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Config {
    path: String,
    version: HashSet<String>,
    last_selected: String,
    remember: bool
}

impl Config {
    fn new() -> Config {
        return Config { 
            path: String::from("C:\\Program Files\\Bitwig Studio"),
            version: HashSet::new(),
            last_selected: String::new(),
            remember: false,
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

    Ok(())
}

fn read_config(config_path: &Path) -> Result<Config, AnyError> {
    use winreg::enums::*;
    use winreg::RegKey;

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (key, _) = hkcu.create_subkey(&config_path)?;

    let config_str: String = key.get_value("config")?;

    Ok(serde_yaml::from_str(&config_str)?)
}

// ---- APP ----
#[macro_use] extern crate native_windows_gui as nwg;
use nwg::{Event, Ui, simple_message, fatal_message, dispatch_events};
#[derive(Debug, Clone, Hash)]
pub enum Id {
    // Controls
    MainWindow,
    VersionList,
    BitwigLabel,
    BitwigDirLabel,
    LaunchButton,
    ChangeDirButton,
    ChangeDirDialog,
    RememberCheckBox,

    // Events
    Launch,
    ChangeDir,

    // Resource,
    Font,

    // Values
    Path,
}

const APP_WIDTH: u32 = 280;
const APP_HEIGHT: u32 = 360;
const APP_MARGIN: u32 = 10;

nwg_template!(
    head: setup_ui<Id>,
    controls: [
        (Id::MainWindow, 
            nwg_window!( 
                title="Bitwig Launcher"; 
                size=(APP_WIDTH, APP_HEIGHT); 
                position=(nwg::constants::CENTER_POSITION, nwg::constants::CENTER_POSITION);
                visible=false
            )
        ),
        (Id::LaunchButton, 
            nwg_button!(
                parent=Id::MainWindow; 
                text="Launch"; 
                position=((APP_WIDTH * 3 / 5 + APP_MARGIN * 2) as i32, (APP_MARGIN * 2 + APP_HEIGHT / 10) as i32); 
                size=(APP_WIDTH * 2 / 5 - APP_MARGIN * 3, APP_HEIGHT / 10);
                font=Some(Id::Font)
            )
        ),
        (Id::ChangeDirButton, 
            nwg_button!(
                parent=Id::MainWindow; 
                text="..."; 
                position=((APP_WIDTH * 8 / 10 + APP_MARGIN * 2) as i32, (APP_MARGIN + APP_HEIGHT / 20) as i32); 
                size=(APP_WIDTH * 2 / 10 - APP_MARGIN * 3, APP_HEIGHT / 20);
                font=Some(Id::Font)
            )
        ),
        (Id::BitwigLabel,
            nwg_label!(
                parent=Id::MainWindow;
                text="Bitwig Location: ";
                position=(APP_MARGIN as i32, APP_MARGIN as i32);
                size=(APP_WIDTH, APP_HEIGHT / 20);
                font=Some(Id::Font)
            )
        ),
        (Id::BitwigDirLabel,
            nwg_label!(
                parent=Id::MainWindow;
                text="........";
                position=(APP_MARGIN as i32, (APP_MARGIN + APP_HEIGHT / 20)  as i32);
                size=(APP_WIDTH, APP_HEIGHT / 20);
                font=Some(Id::Font)
            )
        ),
        (Id::VersionList,
            nwg_listbox!(
                parent=Id::MainWindow;
                position=(APP_MARGIN as i32, (APP_MARGIN * 2 + APP_HEIGHT / 10) as i32); 
                size=(APP_WIDTH * 3 / 5, APP_HEIGHT * 7 / 10);
                collection=Vec::<String>::new();
                font=Some(Id::Font)
            )
        ),
        (Id::RememberCheckBox,
            nwg_checkbox!(
                parent=Id::MainWindow;
                position=(APP_MARGIN as i32, (APP_HEIGHT * 9 / 10) as i32);
                size=(APP_WIDTH, APP_HEIGHT / 20);
                text="Remember selection";
                font=Some(Id::Font)
            )
        ),
        (Id::ChangeDirDialog,
            nwg_filedialog!(
                action=nwg::constants::FileDialogAction::OpenDirectory; 
                title="Bitwig Path..."
            )
        )
    ];
    events: [
        (Id::LaunchButton, Id::Launch, Event::Click, |ui,_,_,_| {
            let (path, version_list) = &nwg_get!(ui; [
                (Id::Path, String), (Id::VersionList, nwg::ListBox<String>)
            ]);
            
            match version_list.get_selected_index() {
                Some(idx) => {
                    let version = &version_list.collection()[idx];
                    match launch_bitwig(path, version) {
                        Ok(_) => {
                            nwg::exit();
                        }
                        Err(_) => ()
                    }
                }
                None => ()
            }
        }),
        (Id::ChangeDirButton, Id::ChangeDir, Event::Click, |ui,_,_,_| {
            change_dir(ui);
            let versions = get_versions(&nwg_get!(ui; [(Id::Path, String)]));
            update_version_list(ui, versions);
        })
    ];
    resources: [
        (Id::Font, nwg_font!(family="Segoe UI"; size=17))
    ];
    values: [
        (Id::Path, String::new())
    ]
);

fn main() -> Result<(), AnyError> {
    // load config
    let config_path = Path::new("Software\\Bitwiglauncher");
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

    // run and update config
    run(&mut config);
    write_config(&config, &config_path)?;
    Ok(())
}

fn get_versions(path:&str) -> HashSet<String> {
    let mut versions: HashSet<String> = HashSet::new();

    if let Ok(paths) = std::fs::read_dir(path) {
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
    }

    return versions;
}

fn change_dir(app: &Ui<Id>) -> Option<String> {
    let dialog = nwg_get_mut!(app; [
        (Id::ChangeDirDialog, nwg::FileDialog)
    ]);

    if dialog.run() {
        let new_path = dialog.get_selected_item().unwrap();
        set_path(app, &new_path);
        Some(new_path)
    } else {
        None
    }
}

fn set_path(app: &Ui<Id>, path:&str) {
    // update ui value (needs scoping to release reference)
    let mut ui_path = nwg_get_mut!(app; [(Id::Path, String)]);
    **ui_path = path.to_owned();
    // setup lables
    let dir_label = nwg_get_mut!(app; [
        (Id::BitwigDirLabel, nwg::Label)
    ]);

    dir_label.set_text(&path);
}

fn update_version_list<T>(app: &Ui<Id>, versions: T) 
    where T: IntoIterator,
        T::Item: AsRef<str> {
    let mut version_list = nwg_get_mut!(app; [
        (Id::VersionList, nwg::ListBox<String>)
    ]);

    let version_collection = version_list.collection_mut();
    version_collection.clear();
    versions.into_iter().for_each(|x| version_collection.push(String::from(x.as_ref())));
    version_collection.sort();
    version_list.sync();
}

fn launch_bitwig(path:&str, version:&str) -> Result<(), AnyError> {
    use std::process::Command;

    let bitwig_path = PathBuf::from(path)
        .join(version).join("Bitwig Studio.exe");

    Command::new(bitwig_path).spawn()?;
    Ok(())
}

fn run(config: &mut Config) {
    // create gui
    let app: Ui<Id>;
    match Ui::new() {
        Ok(_app) => { app = _app; },
        Err(e) => { fatal_message("Fatal Error", &format!("{:?}", e) ); }
    }

    if let Err(e) = setup_ui(&app) {
        fatal_message("Fatal Error", &format!("{:?}", e));
    }

    // collect all available versions
    let versions = get_versions(&config.path);
    // when no version is found prompt path selection
    if versions.len() == 0 {
        if let Some(new_path) = change_dir(&app) {
            config.path = new_path;
        }
    }

    // detect difference uncheck remember
    if config.version != versions {
        config.remember = false;
    }
    
    // if only one version available, launch it.
    if versions.len() == 1 {
        let ver = &versions.iter().next().unwrap();
        match launch_bitwig(&config.path, ver) {
            Ok(_) => { return; }
            _ => ()
        }
    }

    config.version = versions;

    if config.remember {
        match launch_bitwig(&config.path, &config.last_selected) {
            Ok(_) => { return; }
            _ => ()
        }
    }

    set_path(&app, &config.path);

    update_version_list(&app, &config.version);
    {
        let version_list = nwg_get_mut!(app; [
            (Id::VersionList, nwg::ListBox<String>)
        ]);

        match version_list.collection().iter().position(|x| x == &config.last_selected) {
            Some(idx) => {
                version_list.set_selected_index(idx);
            }
            None => ()
        }
    }

    let main_window = nwg_get_mut!(app; [
        (Id::MainWindow, nwg::Window)
    ]);

    // show window and do message loop
    dbg!(&config.last_selected);

    main_window.set_visibility(true);
    dispatch_events();

    // update config
    let (version_list, remember_checkbox, ui_path) = nwg_get!(app; [
        (Id::VersionList, nwg::ListBox<String>),
        (Id::RememberCheckBox, nwg::CheckBox),
        (Id::Path, String)
    ]);

    match version_list.get_selected_index() {
        Some(idx) => {
            config.last_selected = version_list.collection()[idx].clone();
        }
        None => {
            config.last_selected.clear();
        }
    }

    config.version.clear();
    version_list.collection().iter().for_each(|x| {config.version.insert(x.clone()); });
    config.path = *ui_path.clone();
    config.remember = remember_checkbox.get_checkstate() == nwg::constants::CheckState::Checked;
}
