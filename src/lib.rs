use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use regex::{Captures, Regex, RegexBuilder};
use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::OnceLock;

#[pyclass]
struct Pattern {
    regex: Regex,
}

#[pyclass]
struct Match {
    #[allow(dead_code)]
    mat: regex::Match<'static>,
    captures: Captures<'static>,
    text: String,
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

static REGEX_CACHE: OnceLock<Mutex<HashMap<(String, u32), Regex>>> = OnceLock::new();

fn get_regex_cache() -> &'static Mutex<HashMap<(String, u32), Regex>> {
    REGEX_CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

#[pymethods]
impl Match {
    fn group(&self, idx: usize) -> Option<String> {
        self.captures.get(idx).map(|m| m.as_str().to_string())
    }

    fn groups(&self) -> Vec<Option<String>> {
        self.captures
            .iter()
            .skip(1)
            .map(|m| m.map(|mat| mat.as_str().to_string()))
            .collect()
    }

    fn start(&self, idx: usize) -> Option<usize> {
        self.captures.get(idx).map(|m| {
            self.text[..m.start()].chars().count()
        })
    }

    fn end(&self, idx: usize) -> Option<usize> {
        self.captures.get(idx).map(|m| {
            self.text[..m.end()].chars().count()
        })
    }

    fn span(&self, idx: usize) -> Option<(usize, usize)> {
        self.captures.get(idx).map(|m| {
            let start = self.text[..m.start()].chars().count();
            let end = self.text[..m.end()].chars().count();
            (start, end)
        })
    }
}

#[pyfunction]
fn compile(pattern: &str, flags: Option<u32>) -> PyResult<Pattern> {
    let flags = flags.unwrap_or(0);
    let mut cache = get_regex_cache().lock().unwrap();
    
    if let Some(regex) = cache.get(&(pattern.to_string(), flags)) {
        return Ok(Pattern { regex: regex.clone() });
    }

    let mut builder = RegexBuilder::new(pattern);
    if flags & 0b0001 != 0 {
        builder.case_insensitive(true);
    }
    if flags & 0b0010 != 0 {
        builder.multi_line(true);
    }
    if flags & 0b0100 != 0 {
        builder.dot_matches_new_line(true);
    }
    // Add other flags as needed

    let regex = builder
        .build()
        .map_err(|e| PyValueError::new_err(e.to_string()))?;
    
    cache.insert((pattern.to_string(), flags), regex.clone());
    Ok(Pattern { regex })
}

#[pyfunction]
fn search(pattern: &Pattern, text: &str) -> PyResult<Option<Match>> {
    pattern.regex.captures(text).map(|captures| {
        let mat = captures.get(0).unwrap();
        Ok(Some(Match {
            mat: unsafe { std::mem::transmute(mat) },
            captures: unsafe { std::mem::transmute(captures) },
            text: text.to_string(),
        }))
    }).unwrap_or(Ok(None))
}

#[pyfunction(name = "fmatch")]
fn fmatch(pattern: &Pattern, text: &str) -> PyResult<Option<Match>> {
    pattern.regex.captures(text).and_then(|captures| {
        let mat = captures.get(0).unwrap();
        if mat.start() == 0 {
            Some(Ok(Some(Match {
                mat: unsafe { std::mem::transmute(mat) },
                captures: unsafe { std::mem::transmute(captures) },
                text: text.to_string(),
            })))
        } else {
            None
        }
    }).unwrap_or(Ok(None))
}

#[pyfunction]
fn fullmatch(pattern: &Pattern, text: &str) -> PyResult<Option<Match>> {
    pattern.regex.captures(text).and_then(|captures| {
        let mat = captures.get(0).unwrap();
        if mat.as_str() == text {
            Some(Ok(Some(Match {
                mat: unsafe { std::mem::transmute(mat) },
                captures: unsafe { std::mem::transmute(captures) },
                text: text.to_string(),
            })))
        } else {
            None
        }
    }).unwrap_or(Ok(None))
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
        .captures_iter(text)
        .map(|captures| {
            let mat = captures.get(0).unwrap();
            Match {
                mat: unsafe { std::mem::transmute(mat) },
                captures: unsafe { std::mem::transmute(captures) },
                text: text.to_string(),
            }
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
    get_regex_cache().lock().unwrap().clear();
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
    m.add("__version__", "0.1.4")?;
    m.add(
        "__doc__",
        "",
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