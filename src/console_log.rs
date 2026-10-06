use rust_extensions::StrOrString;

pub fn console_log<'s>(message: impl Into<StrOrString<'s>>) {
    write(ConsoleLevel::Log, message.into().as_str());
}

pub fn console_debug<'s>(message: impl Into<StrOrString<'s>>) {
    write(ConsoleLevel::Debug, message.into().as_str());
}

pub fn console_info<'s>(message: impl Into<StrOrString<'s>>) {
    write(ConsoleLevel::Info, message.into().as_str());
}

pub fn console_warn<'s>(message: impl Into<StrOrString<'s>>) {
    write(ConsoleLevel::Warn, message.into().as_str());
}

pub fn console_error<'s>(message: impl Into<StrOrString<'s>>) {
    write(ConsoleLevel::Error, message.into().as_str());
}

enum ConsoleLevel {
    Log,
    Debug,
    Info,
    Warn,
    Error,
}

#[cfg(not(feature = "server"))]
fn write(level: ConsoleLevel, message: &str) {
    let method = match level {
        ConsoleLevel::Log => "log",
        ConsoleLevel::Debug => "debug",
        ConsoleLevel::Info => "info",
        ConsoleLevel::Warn => "warn",
        ConsoleLevel::Error => "error",
    };
    let escaped_message = escape_for_java_script_string(message);
    let js = format!(
        r#"
        console.{}('{}');
    "#,
        method, escaped_message
    );

    crate::eval(&js);
}

#[cfg(feature = "server")]
fn write(level: ConsoleLevel, message: &str) {
    match level {
        ConsoleLevel::Warn | ConsoleLevel::Error => eprintln!("{}", message),
        ConsoleLevel::Log | ConsoleLevel::Debug | ConsoleLevel::Info => println!("{}", message),
    }
}

#[cfg(not(feature = "server"))]
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
