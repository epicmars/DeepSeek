
use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyAny, PyDict, PyInt, PyLong};

// #[tauri::command]
fn token_num(model: &str, messages: &str) -> i32 {
    let mut token_num = -1;
    let openai = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/python/openai.py"));
    let result = Python::with_gil(|py| -> PyResult<i32> {
        let token_count =
            PyModule::from_code(py, openai, "", "")?.getattr("num_tokens_from_messages")?;

        let args = (messages,);
        let num: i32 = token_count.call1(args)?.extract()?;

        Ok(num)
    });

    if result.is_ok() {
        token_num = result.unwrap();
    }
    println!("Token number: {}", token_num);

    token_num
}

fn python_version() -> PyResult<()> {
    Python::with_gil(|py| {
        let sys = py.import("sys")?;
        let version: String = sys.getattr("version")?.extract()?;

        let locals = [("os", py.import("os")?)].into_py_dict(py);
        let code = "os.getenv('USER') or os.getenv('USERNAME') or 'Unknown'";
        let user: String = py.eval(code, None, Some(&locals))?.extract()?;

        println!("Hello {}, I'm Python {}", user, version);
        Ok(())
    })
}