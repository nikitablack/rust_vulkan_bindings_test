pub fn string_to_cstr(s: &String) -> std::ffi::CString {
    std::ffi::CString::new(s.as_str()).expect("Wrong string parameter")
}

pub fn strings_to_cstrings(v: &Vec<String>) -> Vec<std::ffi::CString> {
    v.iter().map(|s: &String| string_to_cstr(s)).collect()
}

pub fn cstrings_to_ccharptrs(v: &Vec<std::ffi::CString>) -> Vec<*const std::os::raw::c_char> {
    v.iter().map(|s| s.as_ptr()).collect()
}
