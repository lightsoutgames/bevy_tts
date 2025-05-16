use bevy::{
    ecs::{component::HookContext, world::DeferredWorld},
    prelude::*,
};
use crossbeam_channel::{unbounded, Receiver};
pub use tts::{self, Backends, Error, Features, UtteranceId};

#[derive(Component, Resource, Clone, Deref, DerefMut)]
#[component(on_add = on_tts_added, on_remove=on_tts_removed)]
pub struct Tts(pub tts::Tts);

impl Default for Tts {
    fn default() -> Self {
        Self(tts::Tts::default().unwrap())
    }
}

impl Tts {
    pub fn screen_reader_available() -> bool {
        tts::Tts::screen_reader_available()
    }
}

#[derive(Event, Debug)]
pub enum TtsEvent {
    UtteranceBegin(UtteranceId),
    UtteranceEnd(UtteranceId),
    UtteranceStop(UtteranceId),
}

#[derive(Component, Resource)]
struct TtsChannel(Receiver<TtsEvent>);

fn on_tts_added(mut world: DeferredWorld, HookContext { entity, .. }: HookContext) {
    let tts = &world.get::<Tts>(entity).unwrap();
    let channel = setup_tts(tts);
    world.commands().entity(entity).insert(channel);
}

fn on_tts_removed(mut world: DeferredWorld, HookContext { entity, .. }: HookContext) {
    world.commands().entity(entity).remove::<TtsChannel>();
}

fn poll_callbacks(
    mut commands: Commands,
    channel: Res<TtsChannel>,
    mut events: EventWriter<TtsEvent>,
    speakers: Query<(Entity, &TtsChannel), With<Tts>>,
) {
    if let Ok(msg) = channel.0.try_recv() {
        events.write(msg);
    }
    for (entity, channel) in &speakers {
        if let Ok(msg) = channel.0.try_recv() {
            commands.entity(entity).trigger(msg);
        }
    }
}

fn setup_tts(tts: &Tts) -> TtsChannel {
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
    TtsChannel(rx)
}
pub struct TtsPlugin;

impl Plugin for TtsPlugin {
    fn build(&self, app: &mut App) {
        let tts = Tts::default();
        let channel = setup_tts(&tts);
        app.add_event::<TtsEvent>()
            .insert_resource(channel)
            .insert_resource(tts)
            .add_systems(Update, poll_callbacks);
    }
}
