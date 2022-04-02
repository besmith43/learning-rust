use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "resources/"]
#[prefix = "prefix/"]
struct Asset;

fn main() {
    let version_file = Asset::get("prefix/version.txt").unwrap();
    println!("{:?}", std::str::from_utf8(version_file.data.as_ref()));
}
