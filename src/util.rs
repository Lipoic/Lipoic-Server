pub fn get_string(opt: &Option<&'static str>) -> Option<String> {
    opt.and_then(|v| Some(v.to_string()))
}
