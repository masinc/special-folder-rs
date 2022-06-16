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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum SpecialFolder {
    AdminTools = CSIDL_ADMINTOOLS as isize,
    ApplicationData = CSIDL_APPDATA as isize,
    RecycleBin = CSIDL_BITBUCKET as isize,
    CDBurning = CSIDL_CDBURN_AREA as isize,

    CommonAdminTools = CSIDL_COMMON_ADMINTOOLS as isize,
    CommonAppData = CSIDL_COMMON_APPDATA as isize,
    CommonDesktopDirectory = CSIDL_COMMON_DESKTOPDIRECTORY as isize,
    CommonDocuments = CSIDL_COMMON_DOCUMENTS as isize,
    CommonFavorites = CSIDL_COMMON_FAVORITES as isize,
    CommonMusic = CSIDL_COMMON_MUSIC as isize,
    CommonOemLinks = CSIDL_COMMON_OEM_LINKS as isize,
    CommonPictures = CSIDL_COMMON_PICTURES as isize,
    CommonPrograms = CSIDL_COMMON_PROGRAMS as isize,
    CommonStartMenu = CSIDL_COMMON_STARTMENU as isize,
    CommonStartup = CSIDL_COMMON_STARTUP as isize,
    CommonTemplates = CSIDL_COMMON_TEMPLATES as isize,
    CommonVideos = CSIDL_COMMON_VIDEO as isize,

    NetworkFolder = CSIDL_COMPUTERSNEARME as isize,
    Connections = CSIDL_CONNECTIONS as isize,

    ControlPanel = CSIDL_CONTROLS as isize,

    Cookies = CSIDL_COOKIES as isize,
    Desktop = CSIDL_DESKTOP as isize,
    DesktopDirectory = CSIDL_DESKTOPDIRECTORY as isize,
    Drives = CSIDL_DRIVES as isize,
    Favorites = CSIDL_FAVORITES as isize,
    Fonts = CSIDL_FONTS as isize,
    History = CSIDL_HISTORY as isize,
    Internet = CSIDL_INTERNET as isize,
    InternetCache = CSIDL_INTERNET_CACHE as isize,
    LocalApplicationData = CSIDL_LOCAL_APPDATA as isize,
    MyDocuments = CSIDL_MYDOCUMENTS as isize,
    MyMusic = CSIDL_MYMUSIC as isize,
    MyPictures = CSIDL_MYPICTURES as isize,
    MyVideos = CSIDL_MYVIDEO as isize,
    NetHood = CSIDL_NETHOOD as isize,
    NetworkShortcuts = CSIDL_NETWORK as isize,
    PrinterShortcuts = CSIDL_PRINTERS as isize,

    ProgramFiles = CSIDL_PROGRAM_FILES as isize,
    ProgramFilesX86 = CSIDL_PROGRAM_FILESX86 as isize,

    CommonProgramFiles = CSIDL_PROGRAM_FILES_COMMON as isize,
    CommonProgramFilesX86 = CSIDL_PROGRAM_FILES_COMMONX86 as isize,

    Programs = CSIDL_PROGRAMS as isize,
    Recent = CSIDL_RECENT as isize,
    Resource = CSIDL_RESOURCES as isize,
    SendTo = CSIDL_SENDTO as isize,
    StartMenu = CSIDL_STARTMENU as isize,
    Startup = CSIDL_STARTUP as isize,
    System = CSIDL_SYSTEM as isize,
    SystemX86 = CSIDL_SYSTEMX86 as isize,
    Templates = CSIDL_TEMPLATES as isize,
    UserProfile = CSIDL_PROFILE as isize,
    Windows = CSIDL_WINDOWS as isize,
}

pub fn get_special_folder(folder: SpecialFolder) -> Option<PathBuf> {
    let mut path = [0; MAX_PATH as usize];
    let result = unsafe { SHGetFolderPathW(0, folder as _, 0, 0, &mut path[0]) };

    if result == 0 {
        let p: Vec<u16> = path.into_iter().take_while(|&x| x != 0).collect();
        let s = OsString::from_wide(&p);
        Some(s.into())
    } else {
        None
    }
}
