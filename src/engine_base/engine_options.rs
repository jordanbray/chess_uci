use engine::option_type::OptionType;
use engine::engine_option::EngineOption;
use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Default)]
pub struct EngineOptions {
    options: HashMap<String, OptionType>,
    buttons: HashMap<String, fn() -> ()>,
}

impl EngineOptions {
    pub fn new<I>(options: I) -> EngineOptions
            where I: IntoIterator<Item=EngineOption> {
        let mut e = EngineOptions::default();

        for x in options.into_iter() {
            e.options.insert(x.get_name().clone(), x.get_option_type().clone());
        }

        e
    }

    fn get_engine_options(&self) -> Vec<EngineOption> {
        let mut result = vec!();
        for (name, option_type) in &self.options {
            result.push(EngineOption::new(name.clone(), option_type.clone()));
        }
        result
    }

    pub fn create_check(&mut self, name: String, default: bool) {
        self.options.insert(name, OptionType::Check(default));
    }

    pub fn create_spin(&mut self, name: String, default: i64, min: i64, max: i64) {
        self.options.insert(name, OptionType::Spin(default, min, max));
    }

    pub fn create_combo(&mut self, name: String, default: String, options: Vec<String>) {
        self.options.insert(name, OptionType::Combo(default, options));
    }

    pub fn create_string(&mut self, name: String, default: String) {
        self.options.insert(name, OptionType::Str(default));
    }

    pub fn create_button(&mut self, name: String, f: fn() -> ()) {
        self.options.insert(name.clone(), OptionType::Button);
        self.buttons.insert(name, f);
    }

    pub fn get_check(&self, name: String) -> bool {
        match self.options.get(&name) {
            Some(OptionType::Check(x)) => return *x,
            _ => panic!("Unknown option"),
        }
    }

    pub fn get_spin(&self, name: String) -> i64 {
        match self.options.get(&name) {
            Some(OptionType::Spin(x, _, _)) => return *x,
            _ => panic!("Unknown Option"),
        }
    }

    pub fn get_combo(&self, name: String) -> String {
        match self.options.get(&name) {
            Some(OptionType::Combo(x, _)) => return x.clone(),
            _ => panic!("Unknown Option"),
        }
    }

    pub fn get_string(&self, name: String) -> String {
        match self.options.get(&name) {
            Some(OptionType::Str(x)) => return x.clone(),
            _ => panic!("Unknown Option"),
        }
    }
}

impl fmt::Display for EngineOptions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for x in self.get_engine_options().iter() {
            write!(f, "{}", x)?;
        }
        write!(f, "")
    }
}
