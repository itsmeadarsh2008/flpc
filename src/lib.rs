use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use regex::{Regex, RegexBuilder};

#[pyclass]
struct Pattern {
    regex: Regex,
}

#[pyclass]
struct Match {
    #[allow(dead_code)]
    mat: regex::Match<'static>,
}

#[pyclass]
struct Scanner {
    // Implement as needed
}

#[pyclass]
struct RegexFlag {
    #[allow(dead_code)]
    pub bits: u32,
}

#[pyclass]
struct Constants;

#[pyclass]
struct Sre;

#[pyfunction]
fn compile(pattern: &str, flags: Option<u32>) -> PyResult<Pattern> {
    let mut builder = RegexBuilder::new(pattern);
    if let Some(f) = flags {
        if f & 0b0001 != 0 {
            builder.case_insensitive(true);
        }
        if f & 0b0010 != 0 {
            builder.multi_line(true);
        }
        if f & 0b0100 != 0 {
            builder.dot_matches_new_line(true);
        }
        // Add other flags as needed
    }
    let regex = builder
        .build()
        .map_err(|e| PyValueError::new_err(e.to_string()))?;
    Ok(Pattern { regex })
}

#[pyfunction]
fn search(pattern: &Pattern, text: &str) -> PyResult<Option<Match>> {
    if let Some(mat) = pattern.regex.find(text) {
        Ok(Some(Match {
            mat: unsafe { std::mem::transmute(mat) },
        }))
    } else {
        Ok(None)
    }
}

#[pyfunction(name = "fmatch")]
fn fmatch(pattern: &Pattern, text: &str) -> PyResult<Option<Match>> {
    if let Some(mat) = pattern.regex.find(text) {
        if mat.start() == 0 {
            Ok(Some(Match {
                mat: unsafe { std::mem::transmute(mat) },
            }))
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}

#[pyfunction]
fn fullmatch(pattern: &Pattern, text: &str) -> PyResult<Option<Match>> {
    if let Some(mat) = pattern.regex.find(text) {
        if mat.as_str() == text {
            Ok(Some(Match {
                mat: unsafe { std::mem::transmute(mat) },
            }))
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}

#[pyfunction]
fn split(pattern: &Pattern, text: &str) -> PyResult<Vec<String>> {
    Ok(pattern.regex.split(text).map(|s| s.to_string()).collect())
}

#[pyfunction]
fn findall(pattern: &Pattern, text: &str) -> PyResult<Vec<String>> {
    Ok(pattern
        .regex
        .find_iter(text)
        .map(|mat| mat.as_str().to_string())
        .collect())
}

#[pyfunction]
fn finditer(pattern: &Pattern, text: &str) -> PyResult<Vec<Match>> {
    Ok(pattern
        .regex
        .find_iter(text)
        .map(|mat| Match {
            mat: unsafe { std::mem::transmute(mat) },
        })
        .collect())
}

#[pyfunction]
fn sub(pattern: &Pattern, repl: &str, text: &str) -> PyResult<String> {
    Ok(pattern.regex.replace_all(text, repl).into_owned())
}

#[pyfunction]
fn subn(pattern: &Pattern, repl: &str, text: &str) -> PyResult<(String, usize)> {
    let result = pattern.regex.replace_all(text, repl);
    let replaced_text = result.clone().into_owned();
    Ok((replaced_text, result.len()))
}

#[pyfunction]
fn escape(text: &str) -> PyResult<String> {
    Ok(regex::escape(text))
}

#[pyfunction]
fn purge() -> PyResult<()> {
    // Implement cache purge if necessary
    Ok(())
}

#[pymodule]
fn flpc(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Pattern>()?;
    m.add_class::<Match>()?;
    m.add_class::<Scanner>()?;
    m.add_class::<RegexFlag>()?;
    m.add_class::<Constants>()?;
    m.add_class::<Sre>()?;
    m.add("__version__", "0.1.0")?;
    m.add(
        "__doc__",
        "A Rust-based regex port for Python3 to get faster performance. 👾",
    )?;
    m.add("__name__", "flpc")?;
    m.add("__package__", "flpc")?;
    m.add(
        "__all__",
        vec![
            "compile",
            "search",
            "fmatch",
            "fullmatch",
            "split",
            "findall",
            "finditer",
            "sub",
            "subn",
            "escape",
            "purge",
        ],
    )?;

    m.add_function(wrap_pyfunction!(compile, m)?)?;
    m.add_function(wrap_pyfunction!(search, m)?)?;
    m.add_function(wrap_pyfunction!(fmatch, m)?)?;
    m.add_function(wrap_pyfunction!(fullmatch, m)?)?;
    m.add_function(wrap_pyfunction!(split, m)?)?;
    m.add_function(wrap_pyfunction!(findall, m)?)?;
    m.add_function(wrap_pyfunction!(finditer, m)?)?;
    m.add_function(wrap_pyfunction!(sub, m)?)?;
    m.add_function(wrap_pyfunction!(subn, m)?)?;
    m.add_function(wrap_pyfunction!(escape, m)?)?;
    m.add_function(wrap_pyfunction!(purge, m)?)?;

    Ok(())
}
