use special_folder::{get_special_folder, SpecialFolder};

fn print_special_folder(folder: SpecialFolder) {
    let path = get_special_folder(folder);
    println!("{} = {:?}", &folder, path);
}

fn main() {
    print_special_folder(SpecialFolder::AdminTools);
    print_special_folder(SpecialFolder::ApplicationData);
    print_special_folder(SpecialFolder::RecycleBin);
    print_special_folder(SpecialFolder::CDBurning);

    print_special_folder(SpecialFolder::CommonAdminTools);
    print_special_folder(SpecialFolder::CommonAppData);
    print_special_folder(SpecialFolder::CommonDesktopDirectory);
    print_special_folder(SpecialFolder::CommonDocuments);
    print_special_folder(SpecialFolder::CommonFavorites);
    print_special_folder(SpecialFolder::CommonMusic);
    print_special_folder(SpecialFolder::CommonOemLinks);
    print_special_folder(SpecialFolder::CommonPictures);
    print_special_folder(SpecialFolder::CommonPrograms);
    print_special_folder(SpecialFolder::CommonStartMenu);
    print_special_folder(SpecialFolder::CommonStartup);
    print_special_folder(SpecialFolder::CommonTemplates);
    print_special_folder(SpecialFolder::CommonVideos);

    print_special_folder(SpecialFolder::NetworkFolder);
    print_special_folder(SpecialFolder::Connections);

    print_special_folder(SpecialFolder::ControlPanel);

    print_special_folder(SpecialFolder::Cookies);
    print_special_folder(SpecialFolder::Desktop);
    print_special_folder(SpecialFolder::DesktopDirectory);
    print_special_folder(SpecialFolder::Drives);
    print_special_folder(SpecialFolder::Favorites);
    print_special_folder(SpecialFolder::Fonts);
    print_special_folder(SpecialFolder::History);
    print_special_folder(SpecialFolder::Internet);
    print_special_folder(SpecialFolder::InternetCache);
    print_special_folder(SpecialFolder::LocalApplicationData);
    print_special_folder(SpecialFolder::MyDocuments);
    print_special_folder(SpecialFolder::MyMusic);
    print_special_folder(SpecialFolder::MyPictures);
    print_special_folder(SpecialFolder::MyVideos);
    print_special_folder(SpecialFolder::NetHood);
    print_special_folder(SpecialFolder::NetworkShortcuts);
    print_special_folder(SpecialFolder::PrinterShortcuts);

    print_special_folder(SpecialFolder::ProgramFiles);
    print_special_folder(SpecialFolder::ProgramFilesX86);

    print_special_folder(SpecialFolder::CommonProgramFiles);
    print_special_folder(SpecialFolder::CommonProgramFilesX86);

    print_special_folder(SpecialFolder::Programs);
    print_special_folder(SpecialFolder::Recent);
    print_special_folder(SpecialFolder::Resource);
    print_special_folder(SpecialFolder::SendTo);
    print_special_folder(SpecialFolder::StartMenu);
    print_special_folder(SpecialFolder::Startup);
    print_special_folder(SpecialFolder::System);
    print_special_folder(SpecialFolder::SystemX86);
    print_special_folder(SpecialFolder::Templates);
    print_special_folder(SpecialFolder::UserProfile);
    print_special_folder(SpecialFolder::Windows);
}
