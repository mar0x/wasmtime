use cranelift_codegen::isa;
use cranelift_codegen::print_errors::pretty_verifier_error;
use cranelift_codegen::settings::{self, Flags};
use cranelift_codegen::verifier;
use cranelift_wasm::{translate_module, DummyEnvironment, FuncIndex, ReturnMode};
use std::fs;
use std::path::Path;
use std::str::FromStr;
use target_lexicon::triple;

#[test]
fn testsuite() {
    let mut paths: Vec<_> = fs::read_dir("../wasmtests")
        .unwrap()
        .map(|r| r.unwrap())
        .filter(|p| {
            // Ignore files starting with `.`, which could be editor temporary files
            if let Some(stem) = p.path().file_stem() {
                if let Some(stemstr) = stem.to_str() {
                    return !stemstr.starts_with('.');
                }
            }
            false
        })
        .collect();
    paths.sort_by_key(|dir| dir.path());
    let flags = Flags::new(settings::builder());
    for path in paths {
        let path = path.path();
        println!("=== {} ===", path.display());
        let data = read_module(&path);
        handle_module(data, &flags, ReturnMode::NormalReturns);
    }
}

#[test]
fn use_fallthrough_return() {
    let flags = Flags::new(settings::builder());
    let path = Path::new("../wasmtests/use_fallthrough_return.wat");
    let data = read_module(&path);
    handle_module(data, &flags, ReturnMode::FallthroughReturn);
}

#[test]
fn use_name_section() {
    let data = wat::parse_str(
        r#"
        (module $module_name
            (func $func_name (local $loc_name i32)
            )
        )"#,
    )
    .unwrap();

    let flags = Flags::new(settings::builder());
    let triple = triple!("riscv64");
    let isa = isa::lookup(triple).unwrap().finish(flags.clone());
    let return_mode = ReturnMode::NormalReturns;
    let mut dummy_environ = DummyEnvironment::new(isa.frontend_config(), return_mode, false);

    translate_module(data.as_ref(), &mut dummy_environ).unwrap();

    assert_eq!(
        dummy_environ.get_func_name(FuncIndex::from_u32(0)).unwrap(),
        "func_name"
    );
}

fn read_module(path: &Path) -> Vec<u8> {
    match path.extension() {
        None => {
            panic!("the file extension is not wasm or wat");
        }
        Some(ext) => match ext.to_str() {
            Some("wasm") => std::fs::read(path).expect("error reading wasm file"),
            Some("wat") => wat::parse_file(path)
                .map_err(|e| e.to_string())
                .expect("failed to parse wat"),
            None | Some(&_) => panic!("the file extension for {:?} is not wasm or wat", path),
        },
    }
}

fn handle_module(data: Vec<u8>, flags: &Flags, return_mode: ReturnMode) {
    let triple = triple!("riscv64");
    let isa = isa::lookup(triple).unwrap().finish(flags.clone());
    let mut dummy_environ = DummyEnvironment::new(isa.frontend_config(), return_mode, false);

    translate_module(&data, &mut dummy_environ).unwrap();

    for func in dummy_environ.info.function_bodies.values() {
        verifier::verify_function(func, &*isa)
            .map_err(|errors| panic!(pretty_verifier_error(func, Some(&*isa), None, errors)))
            .unwrap();
    }
}
