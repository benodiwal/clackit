use flume::{Receiver, Sender};
use once_cell::sync::Lazy;
use rodio_wav_fix::{source::Buffered, Decoder, OutputStream, Sink, Source};
use std::{collections::HashMap, fs::File, io::BufReader, sync::Mutex, thread, time::Duration};

type SoundSource = Buffered<Decoder<BufReader<File>>>;

static GLOBAL_DATA: Lazy<Mutex<HashMap<String, SoundSource>>> = Lazy::new(|| Mutex::new(HashMap::new()));
static WORKER_CHANNEL: Lazy<Mutex<Sender<String>>> = Lazy::new(|| Mutex::new(new_worker()));

fn new_worker() -> Sender<String> {
    let (sender, receiver) = flume::unbounded();
    thread::spawn(move || worker(receiver));
    sender
}

pub fn play_sound(soundpack: String, vol: u16) {
    let mut sender = WORKER_CHANNEL.lock().unwrap();
    if sender.is_disconnected() {
        *sender = new_worker();
    }
    sender.send(format!("{}|{}", soundpack, vol)).expect("Couldn't send name to threadpool");
}

pub fn worker(receiver: Receiver<String>) {
    let (_, stream_handle) = OutputStream::try_default().unwrap();
    while let Ok(raw) = receiver.recv_timeout(Duration::from_secs(20)) {
        let data: Vec<&str> = raw.split("|").collect();
        let name = data[0].to_string();
        let volume = data[1].parse::<u16>().expect("Cannot parse volume");
        let file_name = name.clone();
        let source = {
            let mut sound_map = GLOBAL_DATA.lock().unwrap();
            sound_map
                .entry(name.clone())
                .or_insert_with(|| {
                    let file = BufReader::new(File::open(&file_name[..]).unwrap());
                    Decoder::new(file).unwrap().buffered()
                })
                .clone()
        };
        let sink = Sink::try_new(&stream_handle).unwrap();
        let vol = volume as f32 / 100.0;
        sink.set_volume(vol);
        sink.append(source);
        sink.detach();
    }
}
