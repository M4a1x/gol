mod naive;

pub fn run() {
    run_naive();
}

fn run_naive() {
    use naive::*;
    use std::thread;
    use std::time;
    let mut game = Game::new(
        5,
        5,
        Rules {
            survive: vec![2, 3],
            birth: vec![3],
        },
    );

    game[0][1] = Cell::Alive;
    game[1][2] = Cell::Alive;
    game[2][0] = Cell::Alive;
    game[2][1] = Cell::Alive;
    game[2][2] = Cell::Alive;

    loop {
        println!("{}", game);
        game.compute_next_gen();
        thread::sleep(time::Duration::from_millis(100));
    }
}
