fn main() {
  if std::env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("ohos") {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let manifest_path = std::path::Path::new(&manifest_dir);

    let shim_a = manifest_path
      .parent()
      .expect("workspace parent dir")
      .join("electron-napi-library/lib/libshim.a");

    println!("cargo:warning=libshim.a path: {}", shim_a.display());
    assert!(
      shim_a.exists(),
      "libshim.a not found at {}",
      shim_a.display()
    );

    // Embed every symbol from libshim.a into the .node file.  Without
    // --whole-archive only directly-referenced objects are pulled in; the
    // remaining napi_* objects are left as undefined dynamic symbols and get
    // resolved at runtime from the system libace_napi.z.so, which crashes
    // Electron (ArkTS VM != V8).
    println!(
      "cargo:rustc-link-arg=-Wl,--whole-archive,{},--no-whole-archive",
      shim_a.display()
    );

    // --- ELF symbol interposition fix ---
    //
    // Even after --whole-archive embeds the shim's napi_* locally, napi-rs
    // was compiled with extern "C" references to those symbols.  The linker
    // creates PLT entries for them, and the OHOS runtime resolves the PLT
    // through the global symbol table where libace_napi.z.so (loaded with
    // RTLD_GLOBAL) wins over our local definitions.
    //
    // Two complementary fixes:
    //
    // 1. Version script: marks every napi_* / uv_* symbol as "local" scope.
    //    LLD eliminates the PLT entries for local-scope symbols and replaces
    //    them with direct (non-interposable) calls to the embedded shim.
    //    Only napi_register_module_v1 remains global so Electron can call it.
    //
    // 2. -Bsymbolic-functions: belt-and-suspenders.  For any function that is
    //    defined in this .so, calls from within the same .so are resolved to
    //    the local definition without going through the PLT.
    let ver_script = manifest_path.join("scripts/ohos_napi.map");
    println!("cargo:warning=version script: {}", ver_script.display());
    println!(
      "cargo:rustc-link-arg=-Wl,--version-script={}",
      ver_script.display()
    );
    println!("cargo:rustc-link-arg=-Wl,-Bsymbolic-functions");

    println!("cargo:rustc-link-arg=-Wl,--allow-shlib-undefined");
  }

  napi_build::setup();
}
