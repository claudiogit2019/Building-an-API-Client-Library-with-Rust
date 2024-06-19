use endoflife::request::api_request_all_rust_cycles;
use endoflife::request::api_request_single_rust_cycle;

#[test]
fn single_version_success() {
    let known_rust_vers = ["1.78", "1.77", "1.76", "1.75", "1.74"];

    for rust_ver in known_rust_vers {
        let _ = api_request_single_rust_cycle(rust_ver).unwrap();
    }
}

#[test]
fn bool_or_date_eol_field() {
    let _all_cycles = api_request_all_rust_cycles().unwrap();
}