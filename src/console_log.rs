#[cfg(not(feature = "server"))]
pub fn console_log(message: &str) {
    let escaped_message = escape_for_java_script_string(message);
    let js = format!(
        r#"
        console.log('{}');
    "#,
        escaped_message
    );

    crate::eval(&js);
}

#[cfg(feature = "server")]
pub fn console_log(message: &str) {
    println!("{}", message);
}

fn escape_for_java_script_string(message: &str) -> String {
    let mut result = String::with_capacity(message.len());

    for ch in message.chars() {
        match ch {
            '\\' => result.push_str("\\\\"),
            '\'' => result.push_str("\\'"),
            '"' => result.push_str("\\\""),
            '\n' => result.push_str("\\n"),
            '\r' => result.push_str("\\r"),
            '\t' => result.push_str("\\t"),
            _ => result.push(ch),
        }
    }

    result
}
