use cpython::{py_fn, py_module_initializer, PyResult, Python};

#[tokio::main]
async fn chat_complete(_py: Python, val: &str) -> PyResult<String> {
    let engine = String::from("gpt-4");
    let max_tokens = 256;

    let resp = gptrust_api::chat::complete(
        vec!["user".to_string()],
        vec![val.to_string()],
        Some(engine.to_string()),
        Some(max_tokens),
    )
    .await
    .expect("Couldn't complete the prompt");

    let ans = resp
        .iter()
        .map(|x| x.message.content.clone())
        .collect::<Vec<String>>();

    Ok(ans[0].clone())
}

py_module_initializer!(gptrustpy, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust")?;
    m.add(py, "chat_complete", py_fn!(py, chat_complete(val: &str)))?;
    Ok(())
});
