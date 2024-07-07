use bevy::prelude::*;
use bevy_tts::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, bevy_tts::TtsPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, (event_poll, greet))
        .run();
}

// Speaks a bunch of messages and changes TTS properties.
fn setup(mut tts: ResMut<Tts>) {
    tts.speak("Hello, world.", false).unwrap();
    let Features { rate, .. } = tts.supported_features();
    if rate {
        let original_rate = tts.get_rate().unwrap();
        tts.speak(format!("Current rate: {}", original_rate), false)
            .unwrap();
        let max_rate = tts.max_rate();
        tts.set_rate(max_rate).unwrap();
        tts.speak("This is very fast.", false).unwrap();
        let min_rate = tts.min_rate();
        tts.set_rate(min_rate).unwrap();
        tts.speak("This is very slow.", false).unwrap();
        let normal_rate = tts.normal_rate();
        tts.set_rate(normal_rate).unwrap();
        tts.speak("This is the normal rate.", false).unwrap();
        tts.set_rate(original_rate).unwrap();
    }
    let Features { pitch, .. } = tts.supported_features();
    if pitch {
        let original_pitch = tts.get_pitch().unwrap();
        let max_pitch = tts.max_pitch();
        tts.set_pitch(max_pitch).unwrap();
        tts.speak("This is high-pitch.", false).unwrap();
        let min_pitch = tts.min_pitch();
        tts.set_pitch(min_pitch).unwrap();
        tts.speak("This is low pitch.", false).unwrap();
        let normal_pitch = tts.normal_pitch();
        tts.set_pitch(normal_pitch).unwrap();
        tts.speak("This is normal pitch.", false).unwrap();
        tts.set_pitch(original_pitch).unwrap();
    }
    let Features { volume, .. } = tts.supported_features();
    if volume {
        let original_volume = tts.get_volume().unwrap();
        let max_volume = tts.max_volume();
        tts.set_volume(max_volume).unwrap();
        tts.speak("This is loud!", false).unwrap();
        let min_volume = tts.min_volume();
        tts.set_volume(min_volume).unwrap();
        tts.speak("This is quiet.", false).unwrap();
        let normal_volume = tts.normal_volume();
        tts.set_volume(normal_volume).unwrap();
        tts.speak("This is normal volume.", false).unwrap();
        tts.set_volume(original_volume).unwrap();
    }
    tts.speak("Press G for a greeting.", false).unwrap();
}

// Reports events from TTS subsystem.
fn event_poll(mut events: EventReader<TtsEvent>) {
    for event in events.read() {
        println!("{:?}", event);
    }
}

// Shows how to output speech in response to a keypress.
fn greet(input: Res<ButtonInput<KeyCode>>, mut tts: ResMut<Tts>) {
    if input.just_pressed(KeyCode::KeyG) {
        tts.speak("Hey there!", true).unwrap();
    }
}
