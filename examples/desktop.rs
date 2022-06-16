use special_folder::{get_special_folder, SpecialFolder};

fn main() {
    let path = get_special_folder(SpecialFolder::Desktop).unwrap();
    println!("{path:?}")
}
