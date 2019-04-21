use engine::option_type::OptionType;
use engine::engine_option::EngineOption;
use std::collections::HashMap;
use std::fmt;
use crate::error::Error;
use std::str::FromStr;


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

    pub fn get_check(&self, name: &str) -> bool {
        match self.options.get(name) {
            Some(OptionType::Check(x)) => return *x,
            _ => panic!("Unknown option"),
        }
    }

    pub fn get_spin(&self, name: &str) -> i64 {
        match self.options.get(name) {
            Some(OptionType::Spin(x, _, _)) => return *x,
            _ => panic!("Unknown Option"),
        }
    }

    pub fn get_combo(&self, name: &str) -> String {
        match self.options.get(name) {
            Some(OptionType::Combo(x, _)) => return x.clone(),
            _ => panic!("Unknown Option"),
        }
    }

    pub fn get_string(&self, name: &str) -> String {
        match self.options.get(name) {
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

impl FromStr for EngineOptions {
    type Err = Error;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let options: Result<Vec<EngineOption>, Error> = s.lines()
                                                         .map(|v| EngineOption::from_str(&(v.to_string() + "\n")))
                                                         .collect();
        Ok(Self::new(options?))
    }
}

#[cfg(test)]
fn read_stockfish() -> Result<EngineOptions, Error> {
    EngineOptions::from_str("option name Debug Log File type string default\n\
                             option name Contempt type spin default 24 min -100 max 100\n\
                             option name Analysis Contempt type combo default Both var Off var White var Black var Both\n\
                             option name Threads type spin default 1 min 1 max 512\n\
                             option name Hash type spin default 16 min 1 max 131072\n\
                             option name Clear Hash type button\n\
                             option name Ponder type check default false\n\
                             option name MultiPV type spin default 1 min 1 max 500\n\
                             option name Skill Level type spin default 20 min 0 max 20\n\
                             option name Move Overhead type spin default 30 min 0 max 5000\n\
                             option name Minimum Thinking Time type spin default 20 min 0 max 5000\n\
                             option name Slow Mover type spin default 84 min 10 max 1000\n\
                             option name nodestime type spin default 0 min 0 max 10000\n\
                             option name UCI_Chess960 type check default false\n\
                             option name UCI_AnalyseMode type check default false\n\
                             option name SyzygyPath type string default <empty>\n\
                             option name SyzygyProbeDepth type spin default 1 min 1 max 100\n\
                             option name Syzygy50MoveRule type check default true\n\
                             option name SyzygyProbeLimit type spin default 7 min 0 max 7\n")
}


#[test]
fn read_defaults() {
    let eo = read_stockfish().unwrap();
    assert_eq!(eo.get_string("Debug Log File"), "");
    assert_eq!(eo.get_spin("Contempt"), 24);
    assert_eq!(eo.get_combo("Analysis Contempt"), "Both");
    assert_eq!(eo.get_spin("Threads"), 1);
    assert_eq!(eo.get_spin("Hash"), 16);
    assert_eq!(eo.get_check("Ponder"), false);
    assert_eq!(eo.get_spin("MultiPV"), 1);
    assert_eq!(eo.get_spin("Skill Level"), 20);
    assert_eq!(eo.get_spin("Move Overhead"), 30);
    assert_eq!(eo.get_spin("Minimum Thinking Time"), 20);
    assert_eq!(eo.get_spin("Slow Mover"), 84);
    assert_eq!(eo.get_spin("nodestime"), 0);
    assert_eq!(eo.get_check("UCI_Chess960"), false);
    assert_eq!(eo.get_check("UCI_AnalyseMode"), false);
    assert_eq!(eo.get_string("SyzygyPath"), "<empty>");
    assert_eq!(eo.get_spin("SyzygyProbeDepth"), 1);
    assert_eq!(eo.get_check("Syzygy50MoveRule"), true);
    assert_eq!(eo.get_spin("SyzygyProbeLimit"), 7);
}

