use std::{
    error::Error,
    fmt, fs,
    path::{Path, PathBuf},
};

use serde::Deserialize;

use crate::catalog::SoftwareId;

const TEMPLATE_DIR: &str = "templates";

#[derive(Clone, Debug)]
pub struct Template {
    pub name: String,
    pub description: Option<String>,
    pub software: Vec<SoftwareId>,
    pub path: PathBuf,
}

impl Template {
    pub fn slug(&self) -> String {
        self.name.to_lowercase().replace(' ', "-")
    }
}

#[derive(Debug)]
pub enum TemplateError {
    Io {
        path: PathBuf,
        source: std::io::Error,
    },
    Parse {
        path: PathBuf,
        source: toml::de::Error,
    },
    UnknownSoftware {
        key: String,
        template: String,
    },
    NotFound(String),
}

impl fmt::Display for TemplateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TemplateError::Io { path, .. } => {
                write!(f, "Failed to read template file {}", path.display())
            }
            TemplateError::Parse { path, source } => {
                write!(f, "Failed to parse {}: {}", path.display(), source)
            }
            TemplateError::UnknownSoftware { key, template } => write!(
                f,
                "Template `{}` references unknown software id `{}`",
                template, key
            ),
            TemplateError::NotFound(name) => write!(f, "Template `{}` was not found", name),
        }
    }
}

impl Error for TemplateError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            TemplateError::Io { source, .. } => Some(source),
            TemplateError::Parse { source, .. } => Some(source),
            _ => None,
        }
    }
}

#[derive(Debug, Deserialize)]
struct TemplateFile {
    pub name: Option<String>,
    pub description: Option<String>,
    #[serde(default)]
    pub software: Vec<String>,
}

pub fn load_named(name: &str) -> Result<Template, TemplateError> {
    let explicit = Path::new(TEMPLATE_DIR).join(format!("{name}.toml"));
    if explicit.exists() {
        return load_from_path(explicit);
    }

    let normalized = name.to_lowercase();
    for template in load_all()? {
        if template.slug() == normalized || template.name.eq_ignore_ascii_case(name) {
            return Ok(template);
        }
    }
    Err(TemplateError::NotFound(name.to_string()))
}

pub fn load_all() -> Result<Vec<Template>, TemplateError> {
    let dir = Path::new(TEMPLATE_DIR);
    if !dir.exists() {
        return Ok(vec![]);
    }
    let mut templates = Vec::new();
    for entry in fs::read_dir(dir).map_err(|source| TemplateError::Io {
        path: dir.to_path_buf(),
        source,
    })? {
        let entry = entry.map_err(|source| TemplateError::Io {
            path: dir.to_path_buf(),
            source,
        })?;
        if !entry
            .path()
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.eq_ignore_ascii_case("toml"))
            .unwrap_or(false)
        {
            continue;
        }
        templates.push(load_from_path(entry.path())?);
    }
    Ok(templates)
}

fn load_from_path(path: PathBuf) -> Result<Template, TemplateError> {
    let contents = fs::read_to_string(&path).map_err(|source| TemplateError::Io {
        path: path.clone(),
        source,
    })?;
    let parsed: TemplateFile =
        toml::from_str(&contents).map_err(|source| TemplateError::Parse {
            path: path.clone(),
            source,
        })?;
    let name = if let Some(name) = parsed.name {
        name
    } else if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
        stem.to_string()
    } else {
        path.display().to_string()
    };
    let mut ids = Vec::new();
    for key in parsed.software {
        match SoftwareId::from_key(&key) {
            Some(id) => ids.push(id),
            None => {
                return Err(TemplateError::UnknownSoftware {
                    key,
                    template: name.clone(),
                });
            }
        }
    }
    Ok(Template {
        description: parsed.description,
        software: ids,
        name,
        path,
    })
}
