/// Example program to run two channels of audio from an alsa device into two pedal boards
/// Channel one will be sent through to the alsa left output and channel 2 into the alsa
/// right output.  Alsa input and output device names can be passed in on the command line.
use alsa_device::AlsaDevice;
use clap::{Parser, command};
use log::{error, info};
use pedal_board::PedalBoard;

mod box_error;
mod alsa_device;

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


fn main() -> Result<(), box_error::BoxError> {
    // Turn on the logger
    env_logger::init();
    let args = Args::parse();

    let mut right_board = PedalBoard::new(0_);
    right_board.insert_pedal("Champ", 0);
    right_board.insert_pedal("Sigma Reverb", 0);
    right_board.insert_pedal("Delay", 0);

    let mut left_board = PedalBoard::new(0_);
    left_board.insert_pedal("Sigma Reverb", 0);
    left_board.insert_pedal("Noise Gate", 0);



    info!("running alsa test example");
    let res = AlsaDevice::new(
        [left_board, right_board], 
        &args.in_dev, 
        &args.out_dev);
    match res {
        Ok(mut alsa) => {
            info!("alsa device: {}", alsa);
            loop {
                alsa.process_a_frame()?;
            }
        }
        Err(e) => {
            error!("device open error: {}", dbg!(e));
        }
    }
    Ok(())
}