extern crate directories;
extern crate pathdiff;
#[macro_use]
extern crate serde_json;

use anyhow::{Result};
use directories::BaseDirs;
use pathdiff::diff_paths;
use serde_json::Value;
use std::env::current_dir;

#[derive(Debug)]
pub struct ConfigInfo {
    deno: String,
    deps_http: String,
    deps_https: String,
}

impl ConfigInfo {
    pub fn new() -> Option<ConfigInfo> {
        let dir = BaseDirs::new();
        if let Some(d) = dir {
            let current_dir = current_dir().expect("failed to get current directory");
            let cache_root = d.cache_dir();
            let deno = diff_paths(cache_root.join("deno/lib.deno.d.ts"), &current_dir)
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            let deps = cache_root.join("deps");
            let deps_http = diff_paths(deps.join("http/*"), &current_dir)
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            let deps_https = diff_paths(deps.join("https/*"), &current_dir)
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            Some(ConfigInfo {
                deno,
                deps_http,
                deps_https,
            })
        } else {
            None
        }
    }
}

pub fn deno_init(json_str: String, config_info: &ConfigInfo) -> Result<String> {
    let mut v: Value;
    if json_str != "" {
        v = serde_json::from_str(&json_str)?;
    } else {
        v = json!({});
    }

    if let Value::Null = v["compilerOptions"] {
        v.as_object_mut().unwrap().insert(
            "compilerOptions".to_string(),
            serde_json::to_value(json!({}))?,
        );
    }
    if let Value::Null = v["compilerOptions"]["baseUrl"] {
        v["compilerOptions"]
            .as_object_mut()
            .unwrap()
            .insert("baseUrl".to_string(), serde_json::to_value(json!({}))?);
    }
    if let Value::Null = v["compilerOptions"]["paths"] {
        v["compilerOptions"]
            .as_object_mut()
            .unwrap()
            .insert("paths".to_string(), serde_json::to_value(json!({}))?);
    }
    if let Value::Null = v["compilerOptions"]["plugins"] {
        v["compilerOptions"]
            .as_object_mut()
            .unwrap()
            .insert("plugins".to_string(), serde_json::to_value(json!([]))?);
    }

    *v["compilerOptions"].get_mut("baseUrl").unwrap() = serde_json::to_value(".").unwrap();
    v["compilerOptions"]["paths"]
        .as_object_mut()
        .unwrap()
        .insert("deno".to_string(), serde_json::to_value(&config_info.deno)?);
    println!("{:?}", &v);
    v["compilerOptions"]["paths"]
        .as_object_mut()
        .unwrap()
        .insert(
            "http://*".to_string(),
            serde_json::to_value(&config_info.deps_http)?,
        );
    v["compilerOptions"]["paths"]
        .as_object_mut()
        .unwrap()
        .insert(
            "https://*".to_string(),
            serde_json::to_value(&config_info.deps_https)?,
        );
    v["compilerOptions"]["plugins"]
        .as_array_mut()
        .unwrap()
        .push(json!({"name": "typescript-deno-plugin"}));

    println!("{:?}", &v);

    Ok(serde_json::to_string_pretty(&v).unwrap())
}

#[cfg(test)]
fn config_gen() -> ConfigInfo {
    ConfigInfo {
        deno: "../../this/is/deno/path".to_string(),
        deps_http: "../../this/is/http/path".to_string(),
        deps_https: "../../this/is/https/path".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string_test() {
        let config = config_gen();
        let result = deno_init("".to_string(), &config);
        let expected = r#"{
  "compilerOptions": {
    "baseUrl": ".",
    "paths": {
      "deno": "../../this/is/deno/path",
      "http://*": "../../this/is/http/path",
      "https://*": "../../this/is/https/path"
    },
    "plugins": [
      {
        "name": "typescript-deno-plugin"
      }
    ]
  }
}"#;
        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn no_compiler_options() {
        let config = config_gen();
        let test_str = r#"
{
  "testOption": {
    "testA": "test",
    "testB": [
      "testB-A",
      1234,
      "testB-B",
      "aiueo"
    ]
  }
}
"#;
        let result = deno_init(test_str.to_string(), &config);
        let expected = r#"{
  "compilerOptions": {
    "baseUrl": ".",
    "paths": {
      "deno": "../../this/is/deno/path",
      "http://*": "../../this/is/http/path",
      "https://*": "../../this/is/https/path"
    },
    "plugins": [
      {
        "name": "typescript-deno-plugin"
      }
    ]
  },
  "testOption": {
    "testA": "test",
    "testB": [
      "testB-A",
      1234,
      "testB-B",
      "aiueo"
    ]
  }
}"#;
        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn mix() {
        let config = config_gen();
        let test_str = r#"
{
  "compilerOptions": {
    "baseUrl": ".",
    "paths": {
      "pathA": "/path/to/path/A",
      "pathB": "/path/to/path/B"
    },
    "plugins": [
      {
        "name": "plugin-A"
      },
      {
        "name": "plugin-B"
      }
    ],
    "otherOption": "aaa"
  },
  "testOption": {
    "testA": "test",
    "testB": [
      "testB-A",
      1234,
      "testB-B",
      "aiueo"
    ]
  }
}
"#;
        let result = deno_init(test_str.to_string(), &config);
        let expected = r#"{
  "compilerOptions": {
    "baseUrl": ".",
    "otherOption": "aaa",
    "paths": {
      "deno": "../../this/is/deno/path",
      "http://*": "../../this/is/http/path",
      "https://*": "../../this/is/https/path",
      "pathA": "/path/to/path/A",
      "pathB": "/path/to/path/B"
    },
    "plugins": [
      {
        "name": "plugin-A"
      },
      {
        "name": "plugin-B"
      },
      {
        "name": "typescript-deno-plugin"
      }
    ]
  },
  "testOption": {
    "testA": "test",
    "testB": [
      "testB-A",
      1234,
      "testB-B",
      "aiueo"
    ]
  }
}"#;
        assert_eq!(expected, result.unwrap());
    }
}
