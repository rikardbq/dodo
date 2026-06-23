use dodo::get_file_content;

fn main() {
    let file_string = String::from_utf8_lossy(&get_file_content("example"))
        .replace("\r\n", " ");
    let file_string_split = file_string
        .split("#####")
        .map(|x| x.trim())
        .collect::<Vec<_>>();
    println!("BEFORE: {file_string:?}");
    assert!(file_string_split.len() == 3);
    println!("AFTER: {file_string_split:?}");
    println!("TITLE={}\nDESC={}\nKEYWORDS={}", file_string_split[0], file_string_split[1], file_string_split[2]);
    let keywords_split = file_string_split[2]
    .split(",")
    .collect::<Vec<_>>();
    println!("KEYWORDS_SPLIT: {keywords_split:?}");
    println!("todo: create dodo, make no mistakes!");
}
