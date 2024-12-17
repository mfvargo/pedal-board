/// Example program to run two channels of audio from an alsa device into two pedal boards
/// Alsa input and output device names can be passed in on the command line.
use alsa_device::AlsaDevice;
use clap::{Parser, command};
use log::{error, info};
use pedal_board::PedalBoard;

mod box_error;
mod alsa_device;
use alsa_device::Callback;

#[derive(Parser)]
#[command(version, about, long_about = None, disable_version_flag = true)]
struct Args {
    /// input alsa device
    #[arg(short, long, default_value = "hw:CODEC")]
    in_dev: String,

    /// output alsa device
    #[arg(short, long, default_value = "hw:CODEC")]
    out_dev: String,
}

// The two pedalboards to be used on the audio streams
struct BoardSet {
    right_board: PedalBoard,
    left_board: PedalBoard,
}

impl BoardSet {
    fn new() -> BoardSet {
        let mut right_board = PedalBoard::new(0_);
        right_board.insert_pedal("Champ", 0);
        right_board.insert_pedal("Sigma Reverb", 0);
        right_board.insert_pedal("Delay", 0);
    
        let mut left_board = PedalBoard::new(0_);
        left_board.insert_pedal("Sigma Reverb", 0);
        left_board.insert_pedal("Noise Gate", 0);
        BoardSet {
            left_board: left_board,
            right_board: right_board,
        }
    }
}

// By implementing the Callback trait (defined in alsa_device) this structure can
// be passed into the process_a_frame function on the alsa device.  The alsa device will
// call the function named "call" with a frame of audio samples.
impl Callback for BoardSet {
    fn call(&mut self, 
            in_a: &[f32], 
            in_b: &[f32], 
            out_a: &mut [f32], 
            out_b: &mut [f32]
    ) -> () {
        self.left_board.process(in_a, out_a);
        self.right_board.process(in_b, out_b);    
        // make it stereo
        for (i, _v) in in_a.iter().enumerate() {
            let left = out_a[i];
            out_a[i] += out_b[i];
            out_b[i] += left;
        }
    }
}

fn main() -> Result<(), box_error::BoxError> {
    // Turn on the logger
    env_logger::init();
    let args = Args::parse();

    // Create a board set that implements the callback trait.  The alsa
    // thread will call that function with input/output buffers to process
    let mut bset = BoardSet::new();

    info!("running alsa test example");
    let res = AlsaDevice::new(
        &args.in_dev, 
        &args.out_dev);
    match res {
        Ok(mut alsa) => {
            info!("alsa device: {}", alsa);
            loop {
                alsa.process_a_frame(&mut bset)?;
            }
        }
        Err(e) => {
            error!("device open error: {}", dbg!(e));
        }
    }
    Ok(())
}