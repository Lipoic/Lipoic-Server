use util::util::get_redirect_uri_by_path;

#[test]
fn get_redirect_uri_by_path_test() {
    let issuer = "https://www.lipoic.org";
    let path = "/login";
    let uri = get_redirect_uri_by_path(issuer, path);

    assert_eq!("https://www.lipoic.org/login", uri);
}
