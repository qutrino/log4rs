use humantime;
use log::LogLevelFilter;
use serde::de;
use std::time::Duration;

#[derive(PartialEq, Debug)]
pub struct DeLogLevelFilter(pub LogLevelFilter);

impl de::Deserialize for DeLogLevelFilter {
    fn deserialize<D>(d: &mut D) -> Result<DeLogLevelFilter, D::Error>
        where D: de::Deserializer
    {
        struct V;

        impl de::Visitor for V {
            type Value = DeLogLevelFilter;

            fn visit_str<E>(&mut self, v: &str) -> Result<DeLogLevelFilter, E>
                where E: de::Error
            {
                v.parse().map(DeLogLevelFilter).map_err(|_| E::invalid_value(v))
            }
        }

        d.deserialize_str(V)
    }
}

#[derive(PartialEq, Debug)]
pub struct DeDuration(pub Duration);

impl de::Deserialize for DeDuration {
    fn deserialize<D>(d: &mut D) -> Result<DeDuration, D::Error>
        where D: de::Deserializer
    {
        struct V;

        impl de::Visitor for V {
            type Value = DeDuration;

            fn visit_str<E>(&mut self, v: &str) -> Result<DeDuration, E>
                where E: de::Error
            {
                match humantime::parse_duration(v) {
                    Ok(d) => Ok(DeDuration(d)),
                    Err(e) => Err(E::invalid_value(&e.to_string())),
                }
            }
        }

        d.deserialize(V)
    }
}
