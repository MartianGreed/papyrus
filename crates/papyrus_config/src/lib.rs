use std::collections::HashMap;

pub type ParamPath = String;
#[derive(Debug, Clone)]
pub enum ParamValue {
    String(String),
    Usize(usize),
}
impl Default for ParamValue {
    fn default() -> Self {
        ParamValue::String("".to_string())
    }
}
impl From<ParamValue> for String {
    fn from(value: ParamValue) -> Self {
        match value {
            ParamValue::String(s) => s,
            ParamValue::Usize(u) => u.to_string(),
        }
    }
}
impl From<ParamValue> for usize {
    fn from(value: ParamValue) -> Self {
        match value {
            ParamValue::String(s) => {
                usize::from_str_radix(&s, 10).expect("should be able to convert to usize")
            }
            ParamValue::Usize(u) => u,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct ParamMetadata {
    pub long: String,
    pub short: Option<String>,
    pub description: String,
    pub default_value: Option<ParamValue>,
}
impl ParamMetadata {
    pub fn new(
        long: String,
        short: Option<String>,
        description: String,
        default_value: Option<ParamValue>,
    ) -> Self {
        Self { description, long, short, default_value }
    }
}

#[derive(Debug, Default, Clone)]
pub struct ConfigurationBuilder {
    configuration: HashMap<ParamPath, ParamValue>,
}

#[derive(Debug, Default, Clone)]
pub struct Config {
    configuration: HashMap<ParamPath, ParamValue>,
}

impl Config {
    /// Get value out of configuration.
    /// # Arguments
    /// * `path` - The path of the parameter.
    /// * `message` - The exception message to yeet
    /// # Examples
    ///
    ///  ```
    /// let config = ConfigurationBuilder::apply_default()
    ///     .apply_config_file()
    ///     .apply_env()
    ///     .apply_command_line()
    ///     .build();
    ///
    /// let value: usize = config.get("gateway.max_events_chunk_size", "should have
    /// max_events_chunk_size").into();
    /// ```
    pub fn get(&self, path: &str, message: &str) -> ParamValue {
        self.configuration.get(path).expect(message).to_owned()
    }
}

fn parse_common_value(config: &HashMap<ParamPath, ParamValue>, v: String) -> String {
    if v.starts_with("${") && v.ends_with('}') {
        let ParamValue::String(common_value) =
            config.get(&v[2..v.len() - 1]).expect("common param was not provided") else {
                todo!();

            };
        return common_value.to_string();
    }
    v
}
impl ConfigurationBuilder {
    // Reads the default configuration from the default configuration file and adds all the
    // parameters to the configuration mapping.
    pub fn apply_default() -> Self {
        // TODO: use different file format and crates to make this function better.
        let mut configuration = HashMap::new();
        let default_config_file = include_str!("default_configuration.txt");
        println!("{default_config_file}");

        for line in default_config_file.lines() {
            if line.is_empty() || line.starts_with("//") {
                continue;
            }
            let v: Vec<_> = line.split(" = ").collect();
            let k: ParamPath = v[0].to_owned();
            let v: ParamValue =
                ParamValue::String(parse_common_value(&configuration, v[1].to_owned()));

            configuration.insert(k, v);
        }
        Self { configuration }
    }

    // Reads a configuration file and applies it on the builder mapping.
    pub fn apply_config_file(mut self) -> Self {
        // TODO: implement parsing of config file into a hashmap.
        let file_config = HashMap::new();
        for (k, v) in file_config {
            *self.configuration.entry(k).or_insert(ParamValue::String(String::default())) = v;
        }
        self
    }

    // Applies env settings on the builder mapping.
    pub fn apply_env(mut self) -> Self {
        // TODO: implement parsing of env variables into a hashmap.
        let env_config = HashMap::new();
        for (k, v) in env_config {
            *self.configuration.entry(k).or_insert(ParamValue::String(String::default())) = v;
        }
        self
    }

    // Applies command line arguments on the builder mapping.
    pub fn apply_command_line(mut self) -> Self {
        let cla_config = HashMap::new();
        for (k, v) in cla_config {
            *self.configuration.entry(k).or_insert(ParamValue::String(String::default())) = v;
        }
        self
    }

    pub fn build(self) -> Config {
        Config { configuration: self.configuration }
    }
}

/// Enables your component to have its configuration centralized.
pub trait Configurable {
    // Reads all the necessary values from the mapping and creates an instance of the components
    // configuration.
    // Should be called after applying all of the providers.
    fn new(built: &Config) -> Self;

    // Returns the components configuration + metadata.
    // Used for multiple purposes:
    //  1. Creating the default configuration file.
    //  2. Monitoring the node at runtime - getting the configuration in which it runs.
    fn dump(&self) -> Vec<(ParamPath, ParamValue, ParamMetadata)>;
}
