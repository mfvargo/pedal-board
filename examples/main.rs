use pedal_board::PedalBoard;


fn main() -> () {

    // Printout list of pedal types we know about
    let pedal_types = PedalBoard::get_pedal_types();

    println!("Types of pedals");
    println!("{}", serde_json::to_string_pretty(&pedal_types).unwrap());

    // lets build a pedal board
    let mut board = PedalBoard::new(3);

    // Insert pedals in reverse order (it puts each on at the front)
    for pedal in ["Champ", "Sigma Reverb", "Delay", "Compressor"] {
        board.insert_pedal(pedal, 0);
    }

    let board_json = board.as_json(0);
    println!("Here is the json for that board");
    println!("{}", serde_json::to_string_pretty(&board_json).unwrap());

    let board_str = board_json.to_string();
    // Here is where we can construct a board from json
    let mut board_two = PedalBoard::new(3);
    board_two.load_from_json(&board_str);


    board = PedalBoard::new(0);
    board.insert_pedal("Compressor", 0);

    // Here is how you would change a setting on a pedal
    // So we know the Compressor is the first pedal in the chain.  Let's change the attack
    board.change_value(0, &serde_json::json!({
        "name": "attack",
        "value": 125.0
    }));
    // Now lets click on the bypass switch
    board.change_value(0, &serde_json::json!({
        "name": "bypass",
        "value": true,
    }));
    println!("modified board with bypass on and compressor attack set to 125");
    println!("{}", serde_json::to_string_pretty(&board.as_json(0)).unwrap());

    const FRAME_SIZE: usize = 128;

    // Lastly  let's run some data through the boards
    let in_a: [f32; FRAME_SIZE] = [0.0; FRAME_SIZE];
    let in_b: [f32; FRAME_SIZE] = [0.0; FRAME_SIZE];
    let mut out_a: [f32; FRAME_SIZE] = [0.0; FRAME_SIZE];
    let mut out_b: [f32; FRAME_SIZE] = [0.0; FRAME_SIZE];

    // Imagine you jack callback or a function after an alsa read or whatever that is chunking audio
    // Call process and the board will apply the stacked pedal boards to the audio.
    board.process(&in_a, &mut out_a);
    board_two.process(&in_b, &mut out_b);


    println!("Voila!");

}