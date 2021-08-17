use verkle_trie::ffi_inventory; // example_hello_world -> name of the module
use interoptopus::Interop;

#[test]
#[cfg_attr(miri, ignore)]
fn bindings_csharp() -> Result<(), interoptopus::Error> {
    use interoptopus_backend_csharp::{Config, Generator};

    Generator::new(
        Config::default(),
        ffi_inventory(),
    )
    .write_file("bindings/csharp/Interop.cs")?; // "bindings/csharp/Interop.cs" -> path with respect to the root project folder 

    Ok(())
}