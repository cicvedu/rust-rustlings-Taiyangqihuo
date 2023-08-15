// use std::env;
fn main(){
    // let time=std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
    // std::env::set_var("TEST_FOO", time.to_string());

    let time=std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
    println!("cargo:rustc-env=TEST_FOO={}",time.to_string());
    println!("cargo:rustc-cfg=feature=\"pass\"");

}
