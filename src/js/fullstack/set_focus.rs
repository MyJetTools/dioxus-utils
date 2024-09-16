use dioxus::prelude::*;
pub fn set_focus(id: &str, mut set_focus: Signal<bool>) {
    if *set_focus.read() {
        return;
    }

    set_focus.set(true);

    let js = format!(
        r#"
        setTimeout(() => {{
  let el = document.getElementById('{}');
            if (el) {{
              el.focus();
    }}else{{
       console.log("Element not found");
    }}
    }}, 100);
  
    "#,
        id
    );
    js_sys::eval(js.as_str()).unwrap();
}
