use std::sync::mpsc::{channel, Receiver};

use bevy::prelude::*;
pub use tts::{Error, Features, UtteranceId, TTS};

#[derive(Clone, Copy, Debug)]
pub enum TtsEvent {
    UtteranceBegin(UtteranceId),
    UtteranceEnd(UtteranceId),
    UtteranceStop(UtteranceId),
}

struct TtsChannel(Receiver<TtsEvent>);

fn poll_callbacks(_: &mut World, resources: &mut Resources) {
    let channel = resources.get_thread_local::<TtsChannel>().unwrap();
    if let Ok(msg) = channel.0.try_recv() {
        let mut events = resources.get_mut::<Events<TtsEvent>>().unwrap();
        events.send(msg);
    }
}

pub struct TtsPlugin;

impl Plugin for TtsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let tts = TTS::default().unwrap();
        let (tx, rx) = channel();
        let tx_begin = tx.clone();
        let tx_end = tx.clone();
        let tx_stop = tx;
        let Features {
            utterance_callbacks,
            ..
        } = tts.supported_features();
        if utterance_callbacks {
            tts.on_utterance_begin(Some(Box::new(move |utterance| {
                tx_begin.send(TtsEvent::UtteranceBegin(utterance)).unwrap();
            })))
            .unwrap();
            tts.on_utterance_end(Some(Box::new(move |utterance| {
                tx_end.send(TtsEvent::UtteranceEnd(utterance)).unwrap();
            })))
            .unwrap();
            tts.on_utterance_stop(Some(Box::new(move |utterance| {
                tx_stop.send(TtsEvent::UtteranceStop(utterance)).unwrap();
            })))
            .unwrap();
        }
        app.add_event::<TtsEvent>()
            .add_thread_local_resource(TtsChannel(rx))
            .add_resource(tts)
            .add_system(poll_callbacks.system());
    }
}
