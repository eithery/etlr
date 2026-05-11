use serde_yaml::{Value, Mapping};
use crate::std::result::Result;
use crate::errors::EtlError;
use super::errors as err;


pub(crate) trait YamlReader {
    fn get_opt_str(&self, tag_name: &str) -> Result<Option<String>>;

    #[allow(dead_code)]
    fn get_opt_usize(&self, tag_name: &str) -> Result<Option<usize>>;

    fn get_bool(&self, tag_name: &str, default: bool) -> Result<bool>;

    fn get_value<'a, T>(&'a self, tag_name: &str) -> Result<T>
        where T: TryFrom<&'a Value, Error = EtlError>;

    fn get_opt_value<'a, T>(&'a self, tag_name: &str) -> Result<Option<T>>
        where T: TryFrom<&'a Value, Error = EtlError>;

    fn get_value_or_default<'a, T>(&'a self, tag_name: &str) -> Result<T>
        where T: TryFrom<&'a Value, Error = EtlError> + Default;

    fn get_vec<'a, T>(&'a self, tag_name: &str) -> Result<Vec<T>>
        where T: TryFrom<&'a Value, Error = EtlError>;
}


impl YamlReader for &Mapping {
    fn get_opt_str(&self, tag_name: &str) -> Result<Option<String>> {
        self.get(tag_name)
            .map(|val| {
                val.as_str()
                    .map(String::from)
                    .ok_or_else(|| err::invalid_yaml_value(tag_name, "Expected string"))
            })
            .transpose()
    }


    fn get_opt_usize(&self, tag_name: &str) -> Result<Option<usize>> {
        self.get(tag_name)
            .map_or(Ok(None), |val| {
                val.as_u64()
                    .map(|n| Some(n as usize))
                    .ok_or_else(|| err::invalid_yaml_value(tag_name, "Expected number"))
            })
    }


    fn get_bool(&self, tag_name: &str, default: bool) -> Result<bool> {
        self.get(tag_name)
            .map_or(Ok(default), |val| {
                val.as_bool()
                    .ok_or_else(|| err::invalid_yaml_value(tag_name, "Expected boolean"))
            })
    }


    fn get_value<'a, T>(&'a self, tag_name: &str) -> Result<T>
        where T: TryFrom<&'a Value, Error = EtlError>
    {
        self.get(tag_name)
            .ok_or_else(|| err::missing_required_yaml_value("pos"))?
            .try_into()
    }


    fn get_opt_value<'a, T>(&'a self, tag_name: &str) -> Result<Option<T>>
        where T: TryFrom<&'a Value, Error = EtlError>
    {
        self.get(tag_name)
            .map(TryInto::try_into)
            .transpose()
    }


    fn get_value_or_default<'a, T>(&'a self, tag_name: &str) -> Result<T>
        where T: TryFrom<&'a Value, Error = EtlError> + Default
    {
        self.get_opt_value(tag_name)
            .map(Option::unwrap_or_default)
    }


    fn get_vec<'a, T>(&'a self, tag_name: &str) -> Result<Vec<T>>
        where T: TryFrom<&'a Value, Error = EtlError>
    {
        let value = self.get(tag_name)
            .ok_or_else(|| err::missing_required_yaml_value(tag_name))?;
        match value {
            Value::Sequence(items) => {
                items.iter()
                    .map(TryInto::try_into)
                    .collect()
            }
            _ => Err(err::invalid_yaml_format(tag_name, "Expected a sequence"))
        }
    }
}
