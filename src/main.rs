mod formant;
use crate::formant::Formant;

use cpal::traits::{HostTrait, DeviceTrait, StreamTrait};
use cpal::StreamConfig;
use cpal::BufferSize;
use std::thread;

fn main() {
    let host = cpal::default_host();
    let input_device = host.default_input_device().expect("no output device available");
    let default_input_configs = input_device.supported_input_configs().unwrap();

    //for config in &default_input_configs {
    //    println!("config: {:?}", config);
    //}
    let config = default_input_configs.into_iter().next()
                                        .expect("no supported config?!")
                                        .with_max_sample_rate();
    println!("config: {:?}", config);
    let config = StreamConfig{
        channels: config.channels(),
        sample_rate: config.sample_rate(),
        buffer_size: BufferSize::Default
    };

    let input_stream = input_device.build_input_stream(&config,
                                                move |data: &[f32], _: &cpal::InputCallbackInfo| {
                                                // Process the audio data (data contains audio samples)
                                                println!("Audio data: {:?}", data);
                                                },
                                                move |err| {
                                                    // Handle errors if they occur
                                                    eprintln!("Error in input stream: {:?}", err);
                                                }, None).unwrap();
    input_stream.play().unwrap();

    println!("Capturing audio. Press Enter to exit...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let mut a = Formant{name: String::from("one"), value: 1000};
    let mut b = Formant{name: String::from("two"), value: 5000};
    println!("formant {} - value {}", a.name, a.value);
    println!("formant {} - value {}", b.name, b.value);
    a.value = 2200;
    b.value = 6000;
    println!("formant {} - value {}", a.name, a.value);
    println!("formant {} - value {}", b.name, b.value);
}
