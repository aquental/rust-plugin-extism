use extism::*;

fn main() {
    // let url =
    //     Wasm::url("https://github.com/extism/plugins/releases/latest/download/count_vowels.wasm");
    let url = Wasm::url("file:///Users/aquental/projects/rust/rust-plugin-extism/plugin/target/wasm32-unknown-unknown/debug/rust_pdk_template.wasm");
    let manifest = Manifest::new([url]);
    let mut plugin = Plugin::new(&manifest, [], true).unwrap();

    // let res = plugin
    //     .call::<&str, &str>("count_vowels", "Hello, world!")
    //     .unwrap();
    let res = plugin
        .call::<&str, &str>("Hello", "Antonio Quental")
        .unwrap();
    println!("{}\n", res);
}
