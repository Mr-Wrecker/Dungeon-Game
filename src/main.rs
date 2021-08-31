use crossterm::{
    terminal::{Clear, ClearType},
    QueueableCommand,
};
use rand::{prelude::SliceRandom, thread_rng};
use std::{
    io::{stdin, stdout},
    thread,
    time::Duration,
};

const ROW_NUMBER: i32 = 5;
const COL_NUMBER: i32 = 5;

fn main() {
    clear();
    show_help();
    thread::sleep(Duration::from_secs(5));
    clear();

    let mut cells = vec![];
    for x in 0..ROW_NUMBER {
        for y in 0..COL_NUMBER {
            cells.push((y, x));
        }
    }

    let (mut player, door, monster) = get_location(&cells);

    loop {
        clear();
        draw_map(&player, &cells);
        let valid_moves = get_move(&player);
        println!("Your Current Location {:?}", player);
        println!("You Can Move To {}\n:", valid_moves.join(" And "));

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        input = input.trim().to_uppercase();

        if input == "QUIT" {
            break;
        } else if valid_moves.iter().any(|&item| item == input) {
            player = player_move(&player, input);

            if player == monster {
                println!("\n ** OOH NO, Monster Find You **\n");
                break;
            } else if player == door {
                println!("\n ** Perfect, You Find Exit Door **\n");
                break;
            }
        } else {
            println!("\n ** You Can't Exit of This Map, Stay in Game. **\n");
            thread::sleep(Duration::from_secs(1));
            continue;
        }
    }

    // Play Again
    println!("Are You want Play Again? [Y/n]");
    let mut paly_again = String::new();
    stdin().read_line(&mut paly_again).unwrap();
    paly_again = paly_again.trim().to_lowercase();
    if paly_again != "n" {
        main();
    } else {
        println!("Bye.")
    }
}

fn clear() {
    stdout().queue(Clear(ClearType::All)).unwrap();
}

fn show_help() {
    println!(
        r#"
    Welcome to Dungeon Game
    For Exit The Game Write `QUIT`
    In this map, a door and a monster are hidden.
    To win, you must find the tile of the exit door before you reach the monster.
    "#
    );
}

fn get_location(map: &Vec<(i32, i32)>) -> ((i32, i32), (i32, i32), (i32, i32)) {
    let mut random = thread_rng();
    (
        *map.choose(&mut random).unwrap(),
        *map.choose(&mut random).unwrap(),
        *map.choose(&mut random).unwrap(),
    )
}

fn draw_map(player: &(i32, i32), map: &Vec<(i32, i32)>) {
    println!("{}", " _".repeat(ROW_NUMBER as usize));

    for cell in map.iter() {
        let output: &str;
        let end_line: &str;
        let (x, _) = cell;
        if *x < (ROW_NUMBER - 1) {
            end_line = "";
            if cell == player {
                output = "|X";
            } else {
                output = "|_";
            }
        } else {
            end_line = "\n";
            if cell == player {
                output = "|X|";
            } else {
                output = "|_|";
            }
        }
        print!("{}{}", output, end_line);
    }
    println!();
}

fn get_move(player: &(i32, i32)) -> Vec<&str> {
    let mut movement = vec!["LEFT", "RIGHT", "UP", "DOWN"];
    let (x, y) = player;

    if *x == 0 {
        movement.remove(movement.iter().position(|&i| i == "LEFT").unwrap());
    } else if *x == (ROW_NUMBER - 1) {
        movement.remove(movement.iter().position(|&i| i == "RIGHT").unwrap());
    }

    if *y == 0 {
        movement.remove(movement.iter().position(|&i| i == "UP").unwrap());
    } else if *y == (COL_NUMBER - 1) {
        movement.remove(movement.iter().position(|&i| i == "DOWN").unwrap());
    }

    movement
}

fn player_move(player: &(i32, i32), movement: String) -> (i32, i32) {
    let (mut x, mut y) = *player;

    match movement.as_str() {
        "UP" => y -= 1,
        "DOWN" => y += 1,
        "LEFT" => x -= 1,
        "RIGHT" => x += 1,
        _ => {}
    }
    (x, y)
}
