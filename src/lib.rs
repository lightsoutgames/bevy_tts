use bevy::prelude::*;
use crossbeam_channel::{unbounded, Receiver};
pub use tts::{self, Backends, Error, Features, UtteranceId};

#[derive(Resource, Clone, Deref, DerefMut)]
pub struct Tts(tts::Tts);

#[derive(Debug)]
pub enum TtsEvent {
    UtteranceBegin(UtteranceId),
    UtteranceEnd(UtteranceId),
    UtteranceStop(UtteranceId),
}

#[derive(Resource)]
struct TtsChannel(Receiver<TtsEvent>);

fn poll_callbacks(channel: Res<TtsChannel>, mut events: EventWriter<TtsEvent>) {
    if let Ok(msg) = channel.0.try_recv() {
        events.send(msg);
    }
}

pub struct TtsPlugin;

impl Plugin for TtsPlugin {
    fn build(&self, app: &mut App) {
        let tts = tts::Tts::default().unwrap();
        let (tx, rx) = unbounded();
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
        let tts = Tts(tts);
        app.add_event::<TtsEvent>()
            .insert_resource(TtsChannel(rx))
            .insert_resource(tts)
            .add_system(poll_callbacks);
    }
}
