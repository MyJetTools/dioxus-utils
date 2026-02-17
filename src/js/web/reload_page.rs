pub fn reload_page() {
    let js = r"location.reload();";
    crate::eval(js);
}
