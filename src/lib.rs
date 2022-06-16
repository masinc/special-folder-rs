use std::{ffi::OsString, os::windows::prelude::OsStringExt, path::PathBuf};

use windows_sys::Win32::{
    Foundation::MAX_PATH,
    UI::Shell::{
        SHGetFolderPathW, CSIDL_ADMINTOOLS, CSIDL_APPDATA, CSIDL_BITBUCKET, CSIDL_CDBURN_AREA,
        CSIDL_COMMON_ADMINTOOLS, CSIDL_COMMON_APPDATA, CSIDL_COMMON_DESKTOPDIRECTORY,
        CSIDL_COMMON_DOCUMENTS, CSIDL_COMMON_FAVORITES, CSIDL_COMMON_MUSIC, CSIDL_COMMON_OEM_LINKS,
        CSIDL_COMMON_PICTURES, CSIDL_COMMON_PROGRAMS, CSIDL_COMMON_STARTMENU, CSIDL_COMMON_STARTUP,
        CSIDL_COMMON_TEMPLATES, CSIDL_COMMON_VIDEO, CSIDL_COMPUTERSNEARME, CSIDL_CONNECTIONS,
        CSIDL_CONTROLS, CSIDL_COOKIES, CSIDL_DESKTOP, CSIDL_DESKTOPDIRECTORY, CSIDL_DRIVES,
        CSIDL_FAVORITES, CSIDL_FONTS, CSIDL_HISTORY, CSIDL_INTERNET, CSIDL_INTERNET_CACHE,
        CSIDL_LOCAL_APPDATA, CSIDL_MYDOCUMENTS, CSIDL_MYMUSIC, CSIDL_MYPICTURES, CSIDL_MYVIDEO,
        CSIDL_NETHOOD, CSIDL_NETWORK, CSIDL_PRINTERS, CSIDL_PROFILE, CSIDL_PROGRAMS,
        CSIDL_PROGRAM_FILES, CSIDL_PROGRAM_FILESX86, CSIDL_PROGRAM_FILES_COMMON,
        CSIDL_PROGRAM_FILES_COMMONX86, CSIDL_RECENT, CSIDL_RESOURCES, CSIDL_SENDTO,
        CSIDL_STARTMENU, CSIDL_STARTUP, CSIDL_SYSTEM, CSIDL_SYSTEMX86, CSIDL_TEMPLATES,
        CSIDL_WINDOWS,
    },
};

/// The `SpecialFolder` enum represents the special folders on Windows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum SpecialFolder {
    /// The folder for the user's administrative tools.
    AdminTools = CSIDL_ADMINTOOLS as isize,
    /// The folder for the user's application data.
    ApplicationData = CSIDL_APPDATA as isize,
    /// The folder for the user's recycle bin.
    RecycleBin = CSIDL_BITBUCKET as isize,
    /// The folder for the user's CD burning area.
    CDBurning = CSIDL_CDBURN_AREA as isize,

    /// The folder for the user's common application data.
    CommonAdminTools = CSIDL_COMMON_ADMINTOOLS as isize,
    /// The folder for the user's common application data.
    CommonAppData = CSIDL_COMMON_APPDATA as isize,
    /// The folder for the user's common desktop.
    CommonDesktopDirectory = CSIDL_COMMON_DESKTOPDIRECTORY as isize,
    /// The folder for the user's common documents.
    CommonDocuments = CSIDL_COMMON_DOCUMENTS as isize,
    /// The folder for the user's common favorites.
    CommonFavorites = CSIDL_COMMON_FAVORITES as isize,
    /// The folder for the user's common music.
    CommonMusic = CSIDL_COMMON_MUSIC as isize,
    /// The folder for the user's common oem links.
    CommonOemLinks = CSIDL_COMMON_OEM_LINKS as isize,
    /// The folder for the user's common pictures.
    CommonPictures = CSIDL_COMMON_PICTURES as isize,
    /// The folder for the user's common programs.
    CommonPrograms = CSIDL_COMMON_PROGRAMS as isize,
    /// The folder for the user's common start menu.
    CommonStartMenu = CSIDL_COMMON_STARTMENU as isize,
    /// The folder for the user's common startup.
    CommonStartup = CSIDL_COMMON_STARTUP as isize,
    /// The folder for the user's common templates.
    CommonTemplates = CSIDL_COMMON_TEMPLATES as isize,
    /// The folder for the user's common video.
    CommonVideos = CSIDL_COMMON_VIDEO as isize,

    /// The folder for the user's computers near me.
    NetworkFolder = CSIDL_COMPUTERSNEARME as isize,
    /// The folder for the user's network connections.
    Connections = CSIDL_CONNECTIONS as isize,

    /// The folder for the user's control panel.
    ControlPanel = CSIDL_CONTROLS as isize,

    /// The folder for the user's cookies.
    Cookies = CSIDL_COOKIES as isize,
    /// The folder for the user's desktop.
    Desktop = CSIDL_DESKTOP as isize,
    /// The folder for the user's desktop.
    DesktopDirectory = CSIDL_DESKTOPDIRECTORY as isize,
    /// The folder for the user's drives.
    Drives = CSIDL_DRIVES as isize,
    /// The folder for the user's favorites.
    Favorites = CSIDL_FAVORITES as isize,
    /// The folder for the user's fonts.
    Fonts = CSIDL_FONTS as isize,
    /// The folder for the user's history.
    History = CSIDL_HISTORY as isize,
    /// The folder for the user's internet.
    Internet = CSIDL_INTERNET as isize,
    /// The folder for the user's internet cache.
    InternetCache = CSIDL_INTERNET_CACHE as isize,
    /// The folder for the user's local application data.
    LocalApplicationData = CSIDL_LOCAL_APPDATA as isize,
    /// The folder for the user's local documents.
    MyDocuments = CSIDL_MYDOCUMENTS as isize,
    /// The folder for the user's local music.
    MyMusic = CSIDL_MYMUSIC as isize,
    /// The folder for the user's local pictures.
    MyPictures = CSIDL_MYPICTURES as isize,
    /// The folder for the user's local video.
    MyVideos = CSIDL_MYVIDEO as isize,
    /// The folder for the user's network.
    NetHood = CSIDL_NETHOOD as isize,
    /// The folder for the user's network shortcut.
    NetworkShortcuts = CSIDL_NETWORK as isize,
    /// The folder for the user's printer shortcut.
    PrinterShortcuts = CSIDL_PRINTERS as isize,

    /// The folder for the user's prgram files.
    ProgramFiles = CSIDL_PROGRAM_FILES as isize,
    /// The folder for the user's program files (x86).
    ProgramFilesX86 = CSIDL_PROGRAM_FILESX86 as isize,

    /// The folder for the user's common program files.
    CommonProgramFiles = CSIDL_PROGRAM_FILES_COMMON as isize,
    /// The folder for the user's common program files (x86).
    CommonProgramFilesX86 = CSIDL_PROGRAM_FILES_COMMONX86 as isize,

    /// The folder for the user's programs.
    Programs = CSIDL_PROGRAMS as isize,
    /// The folder for the user's recent.
    Recent = CSIDL_RECENT as isize,
    /// The folder for the user's resources.
    Resource = CSIDL_RESOURCES as isize,
    /// The folder for the user's send to.
    SendTo = CSIDL_SENDTO as isize,
    /// The folder for the user's start menu.
    StartMenu = CSIDL_STARTMENU as isize,
    /// The folder for the user's startup.
    Startup = CSIDL_STARTUP as isize,
    /// The folder for the user's system.
    System = CSIDL_SYSTEM as isize,
    /// The folder for the user's system x86.
    SystemX86 = CSIDL_SYSTEMX86 as isize,
    /// The folder for the user's templates.
    Templates = CSIDL_TEMPLATES as isize,
    /// The folder for the user's profile.
    UserProfile = CSIDL_PROFILE as isize,
    /// The folder for the user's windows.
    Windows = CSIDL_WINDOWS as isize,
}

/// Get the path for a special folder.
pub fn get_special_folder(special_folder: SpecialFolder) -> Option<PathBuf> {
    let mut path = [0; MAX_PATH as usize];
    let result = unsafe { SHGetFolderPathW(0, special_folder as _, 0, 0, &mut path[0]) };

    if result == 0 {
        let p: Vec<u16> = path.into_iter().take_while(|&x| x != 0).collect();
        let s = OsString::from_wide(&p);
        Some(s.into())
    } else {
        None
    }
}
