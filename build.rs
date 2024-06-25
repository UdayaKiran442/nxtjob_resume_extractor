fn main() {
    cc::Build::new()
        .cpp(true)
        .flag("-std=c++17")
        .file("libtch/torch_api.cpp")
        .include("/usr/local/lib/libtorch/include")
        .include("/usr/local/lib/libtorch/include/torch/csrc/api/include")
        .compile("torch_api");
}
