pub fn reload_page() {
    let js = r"location.reload();";
    super::eval(js);
}
