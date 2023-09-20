fn main() {
    println!("!cargo:rerun-if-changed=src/shaders.hlsl");
    println!("!cargo:rerun-if-changed=src/vertex_shader.vs_4_0");
    println!("!cargo:rerun-if-changed=src/pixel_shader.ps_4_0");
    std::fs::copy(
        "src/shaders.hlsl",
        std::env::var("OUT_DIR").unwrap() + "/../../../shaders.hlsl",
    )
    .expect("Copy");
    std::fs::copy(
        "src/vertex_shader.vs_4_0",
        std::env::var("OUT_DIR").unwrap() + "/vertex_shader.vs_4_0",
    )
    .expect("Copy");
    std::fs::copy(
        "src/pixel_shader.ps_4_0",
        std::env::var("OUT_DIR").unwrap() + "/pixel_shader.ps_4_0",
    )
    .expect("Copy");
}
