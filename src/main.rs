use std::io::{self, Write};
use rand::Rng;
struct GameState {
    rows: Vec<usize>,
    current_player: usize,
    game_mode: GameMode,
}
#[derive(Clone, Copy)]
enum GameMode {
    TwoPlayer,
    VsRandomBot,
}


fn main() {
    loop {
        println!("Nim Oyununa Hoş Geldiniz!");
        println!("1- İki Oyunculu Oyna");
        println!("2- Tek Oyunculu Oyna");
        println!("3- Çıkış");


        match get_input("Seçiminizi girin: ") {
            Ok(choice) => match choice.as_str() {
                "1" => play_game(GameMode::TwoPlayer),
                "2" => play_game(GameMode::VsRandomBot),
                "3" => break,
                _ => println!("Geçersiz seçim. Lütfen tekrar deneyin."),
            },
            Err(e) => println!("Hata: {}", e),
        }
    }

    println!("Oynadığınız için teşekkürler!");
}

// Ana oyun döngüsü
fn play_game(mode: GameMode) {
    let row_count = match get_input("Sıra sayısını girin: ") {
        Ok(s) => match s.parse::<usize>() {
            Ok(n) if n > 0 => n,
            _ => {
                println!("Geçersiz sıra sayısı. Varsayılan olarak 4 sıra kullanılacak.");
                4
            }
        },
        Err(e) => {
            println!("Hata: {}", e);
            println!("Varsayılan olarak 4 sıra kullanılacak.");
            4
        }
    };

    let mut game = GameState {
        rows: (1..=row_count).map(|i| i * 2 - 1).collect(),
        current_player: 1,
        game_mode: mode,
    };

    while !game.rows.is_empty() {
        display_board(&game.rows);

        match game.game_mode {
            GameMode::TwoPlayer => make_human_move(&mut game),
            GameMode::VsRandomBot => {
                if game.current_player == 1 {
                    make_human_move(&mut game);
                } else {
                    make_random_bot_move(&mut game);
                }
            }
        }

        game.current_player = 3 - game.current_player; // 1 ve 2 arasında geçiş yapmak için
    }

    match game.game_mode {
        GameMode::TwoPlayer => println!("Oyuncu {} kazandı!", 3 - game.current_player),
        GameMode::VsRandomBot => {
            if game.current_player == 1 {
                println!("Bot kazandı!");
            } else {
                println!("Tebrikler, botu yendiniz!");
            }
        },
    }
}

// Kullanıcıdan girdi alma fonksiyonu
fn get_input(prompt: &str) -> Result<String, String> {
    print!("{}", prompt);
    io::stdout().flush().map_err(|e| e.to_string())?;
    let mut input = String::new();
    io::stdin().read_line(&mut input).map_err(|e| e.to_string())?;
    Ok(input.trim().to_string())
}

// Oyun tahtasını görüntüleme fonksiyonu
fn display_board(rows: &[usize]) {
    let max_width = *rows.iter().max().unwrap_or(&0);

    for (i, &count) in rows.iter().enumerate() {
        let padding = (max_width - count) / 2;
        println!("{:2}. {}{}{}", i + 1, " ".repeat(padding), "|".repeat(count), " ".repeat(padding));
    }
    println!();
}

// İnsan oyuncunun hamle yapma fonksiyonu
fn make_human_move(game: &mut GameState) {
    loop {

        let row = match get_input(&format!("Oyuncu {}, bir sıra seçin: ", game.current_player)) {
            Ok(input) => match input.parse::<usize>() {
                Ok(n) if n > 0 && n <= game.rows.len() => n - 1,
                _ => {
                    println!("Geçersiz sıra. Lütfen 1 ile {} arasında bir sayı girin.", game.rows.len());
                    continue;
                }
            },
            Err(e) => {
                println!("Hata: {}", e);
                continue;
            }
        };


        let count = match get_input("Kaç kibrit çıkarmak istiyorsunuz? ") {
            Ok(input) => match input.parse::<usize>() {
                Ok(n) if n > 0 && n <= game.rows[row] => n,
                _ => {
                    println!("Geçersiz kibrit sayısı. Lütfen 1 ile {} arasında bir sayı girin.", game.rows[row]);
                    continue;
                }
            },
            Err(e) => {
                println!("Hata: {}", e);
                continue;
            }
        };

        game.rows[row] -= count;
        if game.rows[row] == 0 {
            game.rows.remove(row);
        }
        break;
    }
}

// Rastgele bot hamle yapma fonksiyonu
fn make_random_bot_move(game: &mut GameState) {

    let mut rng = rand::thread_rng();
    let row = rng.gen_range(0..game.rows.len());

    let count = rng.gen_range(1..=game.rows[row]);

    println!("Bot {} sırasından {} kibrit çıkardı.", row + 1, count);
    game.rows[row] -= count;
    if game.rows[row] == 0 {
        game.rows.remove(row);
    }
}