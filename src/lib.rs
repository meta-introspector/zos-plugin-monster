mod plugin_trait;
use plugin_trait::*;
use serde_json::json;
use std::os::raw::c_char;

#[no_mangle] pub extern "C" fn zos_plugin_name() -> *mut c_char { to_c("monster") }
#[no_mangle] pub extern "C" fn zos_plugin_version() -> *mut c_char { to_c("0.2.0") }
#[no_mangle] pub extern "C" fn zos_plugin_commands() -> *mut c_char { to_c("orbifold,crown,dimension,hecke") }

#[no_mangle] pub extern "C" fn zos_plugin_execute(cmd: *const c_char, arg: *const c_char) -> *mut c_char {
    let cmd = unsafe { std::ffi::CStr::from_ptr(cmd) }.to_str().unwrap_or("");
    let arg = unsafe { std::ffi::CStr::from_ptr(arg) }.to_str().unwrap_or("42");
    let n: u64 = arg.parse().unwrap_or(42);
    let result = match cmd {
        "orbifold" => json!({"coords": [n%71, n%59, n%47]}),
        "crown" => json!({"product": 196883, "factors": [47,59,71]}),
        "dimension" => json!({"dimension": 196883}),
        "hecke" => json!({"T_p": n, "eigenvalue": (n*n+1) % 196883}),
        _ => json!({"error": cmd}),
    };
    let shard = DA51Shard::from_result("monster", cmd, &result);
    to_c(&serde_json::to_string(&json!({"result": result, "shard": shard})).unwrap())
}

#[no_mangle] pub extern "C" fn zos_plugin_render() -> *mut c_char {
    let gui = vec![
        GuiComponent::Heading { level: 2, text: "🧮 Monster Group".into() },
        GuiComponent::KeyValue { pairs: vec![("Crown".into(), "47×59×71=196883".into())] },
        GuiComponent::Button { label: "Orbifold".into(), command: "orbifold".into() },
    ];
    to_c(&serde_json::to_string(&gui).unwrap())
}

#[no_mangle] pub extern "C" fn zos_plugin_init() -> i32 { 0 }
