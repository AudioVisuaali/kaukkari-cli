pub struct Command {
    pub command_type: CommandType,
    pub commands: Vec<String>,
    pub description: String,
}

pub enum CommandType {
    Help,
    On,
    Off,
    AppleTV,
    Chromecast,
    Disco,
    Fridge,
    StarrySky,
    Mute,
    Volume,
    VolumeUp,
    VolumeDown,
}
