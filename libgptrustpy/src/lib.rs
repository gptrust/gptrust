use pyo3::prelude::*;

#[tokio::main]
#[pyfunction]
async fn answer(_py: Python, val: &str) -> PyResult<String> {
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

#[pymodule]
fn gptrustpy(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(answer, m)?)?;

    Ok(())
}
