// Jack client integration

use jack::prelude::{AsyncClient, AudioOutPort, AudioOutSpec, Client, ClosureProcessHandler,
                    JackControl, ProcessScope, client_options};

use std::f64;
use std::io;
use std::str::FromStr;
use std::sync::mpsc::channel;

use super::oscillators::{Oscillator, Osc};
use super::waveforms::Sine;


fn read_freq() -> Option<f64> {
    let mut user_input = String::new();
    match io::stdin().read_line(&mut user_input) {
        Ok(_) => u16::from_str(&user_input.trim()).ok().map(|n| n as f64),
        Err(_) => None,
    }
}

pub fn run_client() {
    // 1. Open a client
    let (client, _status) = Client::new("rust_jack_sine", client_options::NO_START_SERVER).unwrap();

    // 2. register port
    let mut out_port = client.register_port("aether_out", AudioOutSpec::default())
        .unwrap();

    // 3. define process callback handler
    let mut frequency = 220.0;
    let sample_rate = client.sample_rate();
    let frame_t = 1.0 / sample_rate as f64;
    let mut time = 0.0;
    let (tx, rx) = channel();

    // Define Oscillators, etc.
    let mut osc = Osc::new(frequency, sample_rate, Box::new(Sine));

    let process = ClosureProcessHandler::new(move |_: &Client, ps: &ProcessScope| -> JackControl {
        // Get output buffer
        let mut out_p = AudioOutPort::new(&mut out_port, ps);
        let out: &mut [f32] = &mut out_p;

        // Check frequency requests
        while let Ok(f) = rx.try_recv() {
            time = 0.0;
            frequency = f;
        }

        // Write output
        for v in out.iter_mut() {
            // let x = frequency * time * 2.0 * f64::consts::PI;
            // let y = x.sin();
            let y = osc.get_amplitude();
            *v = y as f32;
            time += frame_t;
        }

        // Continue as normal
        JackControl::Continue
    });

    // 4. activate the client
    let active_client = AsyncClient::new(client, (), process).unwrap();
    // processing starts here

    // 5. wait or do some processing while your handler is running in real time.
    println!("Enter an integer value to change the frequency of the sine wave.");
    while let Some(f) = read_freq() {
        tx.send(f).unwrap();
    }

    // 6. Optional deactivate. not required since activate_client will deactivate on
    // drop, though explicit deactivate may help you identify errors in deactivate.
    active_client.deactivate().unwrap();
}
