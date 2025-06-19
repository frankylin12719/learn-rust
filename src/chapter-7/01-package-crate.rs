fn main() {
    println!("crate 是 Rust 在编译时最小的代码单位。");
    println!("crate 有两种形式：二进制 crate 和库 crate。");
    println!("二进制crate (Binary Crate): 编译成可执行文件 (如 cargo 命令)。");
    println!("库crate (Library Crate): 编译成库 (rlib, .so, .dll 等)，包含可复用的代码");
    println!("包(package)是提供一系列功能的一个或者多个 crate 的捆绑");
    println!("一个包会包含一个 Cargo.toml 文件，阐述如何去构建这些 crate。");
    println!(
        "包中可以包含至多一个库 crate(library crate)。包中可以包含任意多个二进制 crate(binary crate)，但是必须至少包含一个 crate(无论是库的还是二进制的）。"
    );
    // println!("cargo约定: src/main.rs 就是一个与包同名的二进制 crate 的 crate 根。");
    // println!("目录中包含 src/lib.rs, 则包带有与其同名的库 crate, 且 src/lib.rs 是 crate 根");
}
