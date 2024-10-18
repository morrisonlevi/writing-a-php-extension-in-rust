fn php_script_num(arg: &str) -> u64 {
    let output = std::process::Command::new("php")
        .arg("-r")
        .arg(arg)
        .output()
        .expect("Unable to run `php-config`. Is it in your PATH?");
    if !output.status.success() {
        match String::from_utf8(output.stderr) {
            Ok(stderr) => panic!("`php` failed: {stderr}"),
            Err(err) => panic!("`php` failed, not utf-8: {err}"),
        }
    }
    std::str::from_utf8(output.stdout.as_slice())
        .expect("`php script stdout to contain valid utf-8")
        .trim()
        .parse::<u64>()
        .expect("php script to be a number and fit in u64")
}

fn main() {
    let zts = php_script_num("echo PHP_ZTS ? 1 : 0, PHP_EOL;");
    let debug = php_script_num("echo PHP_DEBUG ? 1 : 0, PHP_EOL;");

    println!("cargo::rustc-check-cfg=cfg(php_zts)");
    if zts != 0 {
        println!("cargo:rustc-cfg=php_zts");
    }

    println!("cargo::rustc-check-cfg=cfg(php_debug)");
    if debug != 0 {
        println!("cargo:rustc-cfg=php_debug");
    }
}
