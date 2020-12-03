use bevy::prelude::*;
use bevy_tts::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_tts::TtsPlugin)
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .add_startup_system(setup)
        .add_system(event_poll)
        .add_system(greet)
        .run();
}

// Speaks a bunch of messages and changes TTS properties.
fn setup(mut tts: ResMut<TTS>) {
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
fn event_poll(
    mut tts_event_reader: Local<EventReader<TtsEvent>>,
    tts_events: Res<Events<TtsEvent>>,
) {
    for event in tts_event_reader.iter(&tts_events) {
        println!("{:?}", event);
    }
}

// Shows how to output speech in response to a keypress.
fn greet(input: Res<Input<KeyCode>>, mut tts: ResMut<TTS>) {
    if input.just_pressed(KeyCode::G) {
        tts.speak("Hey there!", true).unwrap();
    }
}
