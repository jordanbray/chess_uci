use engine::engine_command::EngineCommand;
use error::Error;
use gui::gui_command::GuiCommand;
use std::fmt;
use std::str::FromStr;

#[derive(Clone, PartialEq, Debug)]
pub enum Command {
    Engine(EngineCommand),
    Gui(GuiCommand),
    Unknown(String),
}

impl Command {
    pub fn new_from_engine(c: EngineCommand) -> Command {
        Command::Engine(c)
    }

    pub fn new_from_gui(c: GuiCommand) -> Command {
        Command::Gui(c)
    }
}

impl FromStr for Command {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed = EngineCommand::from_str(s);
        if let Ok(engine_command) = parsed {
            Ok(Command::new_from_engine(engine_command))
        } else {
            let gui_parsed = GuiCommand::from_str(s);
            if let Ok(gui_command) = gui_parsed {
                Ok(Command::new_from_gui(gui_command))
            } else {
                Ok(Command::Unknown(s.to_string()))
            }
        }
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Command::Engine(ref e) => write!(f, "{}", e),
            Command::Gui(ref g) => write!(f, "{}", g),
            Command::Unknown(ref s) => write!(f, "{}", s),
        }
    }
}
