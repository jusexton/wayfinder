use core::fmt;
use std::collections::HashMap;

use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer,
};
use serde_json::Value;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PathComponent {
    Key(String),
    Index(usize),
}

impl fmt::Display for PathComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PathComponent::Key(key) => write!(f, "{}", key),
            PathComponent::Index(idx) => write!(f, "[{}]", idx),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Path {
    components: Vec<PathComponent>,
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, component) in self.components.iter().enumerate() {
            if i > 0 && matches!(component, PathComponent::Key(_)) {
                write!(f, ".")?;
            }
            write!(f, "{}", component)?;
        }
        Ok(())
    }
}

impl Path {
    pub fn new() -> Self {
        Self {
            components: vec![PathComponent::Key("$root".to_string())],
        }
    }

    pub fn push_key(&mut self, key: String) {
        self.components.push(PathComponent::Key(key));
    }

    pub fn push_index(&mut self, index: usize) {
        self.components.push(PathComponent::Index(index));
    }

    pub fn pop(&mut self) -> Option<PathComponent> {
        self.components.pop()
    }
}

struct PathFindingVisitor {
    current_path: Path,
    found_paths: HashMap<String, Vec<Path>>,
}

impl PathFindingVisitor {
    fn new<S>(targets: Vec<S>) -> Self
    where
        S: Into<String>,
    {
        PathFindingVisitor {
            current_path: Path::new(),
            found_paths: targets.into_iter().fold(HashMap::new(), |mut acc, curr| {
                acc.entry(curr.into()).or_default();
                acc
            }),
        }
    }

    fn visit_value(&mut self, v: &Value) {
        match v {
            Value::Object(map) => {
                for (key, value) in map {
                    if self.found_paths.contains_key(key) {
                        let paths = self.found_paths.get_mut(key).unwrap();
                        paths.push(self.current_path.clone());
                    }
                    self.current_path.push_key(key.clone());
                    self.visit_value(value);
                    self.current_path.pop();
                }
            }
            Value::Array(arr) => {
                for (index, value) in arr.iter().enumerate().rev() {
                    self.current_path.push_index(index);
                    self.visit_value(value);
                    self.current_path.pop();
                }
            }
            _ => {}
        }
    }
}

impl<'de> Visitor<'de> for PathFindingVisitor {
    type Value = HashMap<String, Vec<Path>>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid JSON value")
    }

    fn visit_map<A>(mut self, map: A) -> Result<Self::Value, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let value: Value = Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))?;
        self.visit_value(&value);
        Ok(self.found_paths)
    }
}

pub fn find_paths<'de, D, S>(
    deserializer: D,
    target_properties: Vec<S>,
) -> Result<HashMap<String, Vec<Path>>, D::Error>
where
    D: Deserializer<'de>,
    S: Into<String>,
{
    let finder = PathFindingVisitor::new(target_properties);
    deserializer.deserialize_map(finder)
}
