use wasm_bindgen::JsValue;

pub fn dom(n: usize) -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");
    let container = document.create_element("div")?;
    body.append_child(&container)?;

    for i in 0..n {
        let header = document.create_element("h3")?;
        header.set_text_content(Some(&format!("Header {}", i)));
        let paragraph = document.create_element("p")?;
        paragraph.set_text_content(Some(&format!("Paragraph {}", i)));
        container.append_child(&header)?;
        container.append_child(&paragraph)?;
    }

    body.remove_child(&container)?;

    Ok(())
}