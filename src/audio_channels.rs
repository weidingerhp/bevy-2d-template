use bevy_kira_audio::AudioChannel;

pub struct AudioChannels {
    _foreground: AudioChannel,
    _background: AudioChannel,   
}

impl AudioChannels {
    pub fn new() -> Self {
        Self {
            _foreground: AudioChannel::new("foreground".to_owned()),
            _background: AudioChannel::new("background".to_owned()),
        }
    }
}
