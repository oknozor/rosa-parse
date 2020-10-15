#[macro_use]
extern crate anyhow;

use anyhow::Result;
use serde::Serialize;
use serde_json::Value as Json;

pub enum Format {
    Toml,
    Yaml,
    Json,
}

pub fn convert(content: &str, target: Format) -> Result<String> {
    let json_str = serde_json::from_str::<Json>(content)
        .map_err(|err| anyhow!("Error deserializing Json content : \n{}", err));

    if let Ok(json) = json_str {
        return Ok(get_output(&json, &target).unwrap());
    };

    let toml_str = toml::from_str::<Json>(content)
        .map_err(|err| anyhow!("Error deserializing Toml content : \n{}", err));

    if let Ok(toml) = toml_str {
        return Ok(get_output(&toml, &target).unwrap());
    }

    let yaml_str = serde_yaml::from_str::<Json>(&content)
        .map_err(|err| anyhow!("Error deserializing Yaml content : \n{}", err));

    if let Ok(yaml) = yaml_str {
        Ok(get_output(&yaml, &target).unwrap())
    } else {
        Err(anyhow!("err"))
    }
}

fn get_output<T>(content: &T, target: &Format) -> Result<String>
where
    T: Serialize,
{
    match target {
        Format::Toml => toml::to_string(&content)
            .map_err(|err| anyhow!("Error serializing Toml content : \n{}", err)),
        Format::Yaml => serde_yaml::to_string(&content)
            .map_err(|err| anyhow!("Error serializing Yaml content : \n{}", err)),
        Format::Json => serde_json::to_string_pretty(&&content)
            .map_err(|err| anyhow!("Error serializing Json content : \n{}", err)),
    }
}

#[cfg(test)]
mod tests {
    use crate::{convert, Format};
    use anyhow::Result;
    use std::fs;

    #[test]
    fn yaml_to_toml_ok() -> Result<()> {
        let from = fs::read_to_string("tests/replica-set.yaml")?;

        let to = convert(&from, Format::Toml)?;
        let expected = fs::read_to_string("tests/replica-set.toml")?;

        assert_eq!(to, expected);
        Ok(())
    }

    #[test]
    fn yaml_to_json_ok() -> Result<()> {
        let from = fs::read_to_string("tests/replica-set.yaml")?;

        let to = convert(&from, Format::Json)?;
        let expected = fs::read_to_string("tests/replica-set.json")?;

        assert_eq!(to, expected);
        Ok(())
    }

    #[test]
    fn yaml_to_yaml_ok() -> Result<()> {
        let from = fs::read_to_string("tests/replica-set.yaml")?;

        let to = convert(&from, Format::Yaml)?;
        let expected = fs::read_to_string("tests/replica-set.yaml")?;

        assert_eq!(to, expected);
        Ok(())
    }

    #[test]
    fn toml_to_toml_ok() -> Result<()> {
        let from = fs::read_to_string("tests/replica-set.toml")?;

        let to = convert(&from, Format::Toml)?;
        let expected = fs::read_to_string("tests/replica-set.toml")?;

        assert_eq!(to, expected);
        Ok(())
    }

    #[test]
    fn toml_to_json_ok() -> Result<()> {
        let from = fs::read_to_string("tests/replica-set.toml")?;

        let to = convert(&from, Format::Json)?;
        let expected = fs::read_to_string("tests/replica-set.json")?;

        assert_eq!(to, expected);
        Ok(())
    }

    #[test]
    fn toml_to_yaml_ok() -> Result<()> {
        let from = fs::read_to_string("tests/replica-set.toml")?;

        let to = convert(&from, Format::Yaml)?;
        let expected = fs::read_to_string("tests/replica-set.yaml")?;

        assert_eq!(to, expected);
        Ok(())
    }

    #[test]
    fn json_to_toml_ok() -> Result<()> {
        let from = fs::read_to_string("tests/replica-set.json")?;

        let to = convert(&from, Format::Toml)?;
        let expected = fs::read_to_string("tests/replica-set.toml")?;

        assert_eq!(to, expected);
        Ok(())
    }

    #[test]
    fn json_to_json_ok() -> Result<()> {
        let from = fs::read_to_string("tests/replica-set.json")?;

        let to = convert(&from, Format::Json)?;
        let expected = fs::read_to_string("tests/replica-set.json")?;

        assert_eq!(to, expected);
        Ok(())
    }

    #[test]
    fn json_to_yaml_ok() -> Result<()> {
        let from = fs::read_to_string("tests/replica-set.json")?;

        let to = convert(&from, Format::Yaml)?;
        let expected = fs::read_to_string("tests/replica-set.yaml")?;

        assert_eq!(to, expected);
        Ok(())
    }

    #[test]
    fn toml_with_value_after_table_ok() -> Result<()> {
        let from = fs::read_to_string("tests/values-after-table.yaml")?;

        let to = convert(&from, Format::Toml)?;
        let expected = fs::read_to_string("tests/values-reordered.toml")?;

        assert_eq!(to, expected);
        Ok(())
    }
}
