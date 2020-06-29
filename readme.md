# Bitwig Launcher for Windows
Since version 3.2, Bitwig introduced new installer which allows different versions to be installed at the same time. This is a useful but it's quite annoying to manage shortcuts upon every new version.

This utility let you easily switch between versions when multiple versions are installed. 

---
## Requirements
* Windows 10 64bit
* This utility depends on directory structure and file names so all Bitwig versions must be neatly installed in one parent folder. <br>
    <b>Good:</b>
    <pre>...\(A Bitwig Directory)
                            \3.2
                            \3.2.1
                            \3.2.2</pre>
    <b>Bad:</b>
    <pre>...\(A Bitwig Directory)
                            \3.2 <br>...\(B Bitwig directory)                   
                            \3.2.1
                            \3.2.2</pre>

## Installation
1. Download Binary from released 
1. Place it in any folder you want.
1. Create Shortcut.
1. <a name="Installation41"></a>Change Icon (Optional)
    1. Right-click on the shortcut and choose "Properties"<br>
    In "Shortcut" tab, click "Change Icon" <br>
    ![](https://i.imgur.com/kbJESHp.png)
    1. Click "Browse" and locate Bitwig Executable. <br>
    The default path is <pre>C:\Program Files\Bitwig Studio\$(VERSION)\Bitwig Studio.exe</pre>
    ![](https://i.imgur.com/8MByrCn.png) ![](https://i.imgur.com/uynXkik.png) <br>
    1. Choose Bitwig icon and apply.

## Usage
1. If the program couldn't find Bitwig in the default installation path, you will be prompted to choose a folder. <b>Choose a parent folder containing all different versions.</b>
2. <b>If only one version is detected it will automatically launch.</b>
3. "Don't ask in the future" will remember your choice untill you install or uninstall different Bitwig versions.

## Troubleshooting
### Show the launcher window after "Don't ask in the future"
1. Open the shortcut properties and add <code> --reset</code>  at the end of <b>Target</b> (Refer to [Installation.4.1](#Installation41))
    <pre>..\bitwig_launcher.exe --reset</pre>
    Make sure a space after <code>exe</code>
2. Run the launcher via shortcut.
3. Revert the change made in Step 1.

### Bitwig won't open
* See if Bitwig is already running (Check in the task manager)