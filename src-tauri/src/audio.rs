use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};
use std::io::Cursor;
use std::sync::mpsc;
use tauri::AppHandle;

use crate::utils::load_asset_bytes;
pub(crate) enum AudioCommand {
    PlayClick,
    PlayBgm,
    StopBgm,
    PlayGacha(f32),
    StopGacha,
}

#[derive(Clone)]
pub(crate) struct AudioController {
    tx: mpsc::Sender<AudioCommand>,
}

impl AudioController {
    pub(crate) fn new(app: &AppHandle) -> Self {
        let click_bytes = load_asset_bytes(app, "sound/button_click.wav");
        let bgm_bytes = load_asset_bytes(app, "sound/bgm.mp3");
        let gacha_bytes = load_asset_bytes(app, "sound/gacha_loading.ogg");
        let (tx, rx) = mpsc::channel();
        std::thread::spawn(move || run_audio_thread(rx, click_bytes, bgm_bytes, gacha_bytes));
        Self { tx }
    }

    pub(crate) fn send(&self, command: AudioCommand) -> Result<(), String> {
        self.tx.send(command).map_err(|error| error.to_string())
    }
}

fn run_audio_thread(
    rx: mpsc::Receiver<AudioCommand>,
    click_bytes: Vec<u8>,
    bgm_bytes: Vec<u8>,
    gacha_bytes: Vec<u8>,
) {
    let Ok((_stream, handle)) = OutputStream::try_default() else {
        return;
    };
    let mut bgm_sink: Option<Sink> = None;
    let mut gacha_sink: Option<Sink> = None;

    while let Ok(command) = rx.recv() {
        match command {
            AudioCommand::PlayClick => {
                play_audio_once(&handle, &click_bytes, 1.0);
            }
            AudioCommand::PlayBgm => {
                if let Some(sink) = bgm_sink.take() {
                    sink.stop();
                }
                bgm_sink = play_audio_loop(&handle, &bgm_bytes, 0.3);
            }
            AudioCommand::StopBgm => {
                if let Some(sink) = bgm_sink.take() {
                    sink.stop();
                }
            }
            AudioCommand::PlayGacha(volume) => {
                if let Some(sink) = gacha_sink.take() {
                    sink.stop();
                }
                gacha_sink = play_audio_sink(&handle, &gacha_bytes, volume);
            }
            AudioCommand::StopGacha => {
                if let Some(sink) = gacha_sink.take() {
                    sink.stop();
                }
            }
        }
    }
}

fn decoder_from_bytes(bytes: &[u8]) -> Option<Decoder<Cursor<Vec<u8>>>> {
    if bytes.is_empty() {
        return None;
    }
    Decoder::new(Cursor::new(bytes.to_vec())).ok()
}

fn play_audio_once(handle: &OutputStreamHandle, bytes: &[u8], volume: f32) {
    if let Some(sink) = play_audio_sink(handle, bytes, volume) {
        sink.detach();
    }
}

fn play_audio_sink(handle: &OutputStreamHandle, bytes: &[u8], volume: f32) -> Option<Sink> {
    let sink = Sink::try_new(handle).ok()?;
    let source = decoder_from_bytes(bytes)?;
    sink.set_volume(volume.clamp(0.0, 1.0));
    sink.append(source);
    Some(sink)
}

fn play_audio_loop(handle: &OutputStreamHandle, bytes: &[u8], volume: f32) -> Option<Sink> {
    let sink = Sink::try_new(handle).ok()?;
    let source = decoder_from_bytes(bytes)?.repeat_infinite();
    sink.set_volume(volume.clamp(0.0, 1.0));
    sink.append(source);
    Some(sink)
}
