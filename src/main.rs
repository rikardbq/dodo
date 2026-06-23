use dodo::get_file_content;

fn main() {
    let file_string = String::from_utf8_lossy(&get_file_content("example"))
        .replace("\r\n", " ");
    let file_string_split = file_string
        .split("#####")
        .map(|x| x.trim())
        .collect::<Vec<&str>>();
    println!("BEFORE: {file_string:?}");
    println!("AFTER: {file_string_split:?}");
    println!("todo: create dodo, make no mistakes!");
}
