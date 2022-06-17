extern crate termion;
extern crate rand;

use termion::{color, style};
use std::{thread, time::Duration};
use rand::Rng;
use std::io::{self, Write};
use std::mem;

#[derive(Copy, Clone)]
struct Char {
    x: u16,
    y: u16,
    life: u16,
    maxlife: u16,
    intensity: f64,
    glitched: bool,
    character: char,
}

pub fn print_matrix() {
    let mut rng = rand::thread_rng();

    let width = 60;
    let height = 20;
    let mut chars = Vec::<Char>::new();
    let process_time = 50;

    print!("{}", termion::clear::All);
    loop {

        /*for old in &chars {
            print_char_old(*old);
        }*/

        let c = Char {
            x: rng.gen_range(1..width),
            y: 1,
            life: 0,
            maxlife: rng.gen_range(10..20),
            glitched: rng.gen::<f64>() > 0.95,
            intensity: rng.gen::<f64>(),
            character: get_char()
        };
        let mut new_chars = Vec::<Char>::new();
        for (i, c) in chars.iter_mut().enumerate() {
            c.life = c.life + 1;

            if (c.glitched) {
                c.character = get_char()
            }

            if (c.y < height && c.life == 1) {
                new_chars.push(Char {
                    x: c.x,
                    y: c.y + 1,
                    life: 0,
                    maxlife: c.maxlife,
                    glitched: rng.gen::<f64>() > 0.95,
                    intensity: rng.gen::<f64>(),
                    character: get_char()
                });
            }
        }

        for c in &new_chars {
            chars.push(*c);
        }
        // Removes those that are past the height limit
        
        chars.push(c);

        for c in &chars {
            if (c.life == 0) {
                print_char_current(*c);
            } else if (c.life < c.maxlife / 2) {
                print_char_semiold(*c);
            } else if (c.life < c.maxlife) {
                print_char_old(*c);
            } else {
                clear(*c);
            }
        }

        chars.retain(|&c| c.life < c.maxlife);

        //print_char_current(c);

        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(process_time));
    }
}

fn get_char() -> char {
    let CHARS = &['モ','エ','ヤ','キ','オ','カ','7','ケ','サ','ス','z','1','5','2','ヨ','タ','ワ','4','ネ','ヌ','ナ','9','8','ヒ','0','ホ','ア','3','ウ',' ','セ','¦',':','"','꞊','ミ','ラ','リ','╌','ツ','テ','ニ','ハ','ソ','▪','—','<','>','0','|','+','*','コ','シ','マ','ム','メ'];
    let mut rng = rand::thread_rng();
    let end = CHARS.len();
    return CHARS[rng.gen_range(0..end)];
}

fn print_char_current(c: Char) {
    print!("{}{}{}", color::Fg(color::White), termion::cursor::Goto(c.x, c.y), c.character);
}

fn print_char_semiold(c: Char) {
    let g = c.intensity * 60.0 + 180.0;
    print!("{}{}{}{}{}", color::Fg(color::Rgb(0, g as u8, 0)), style::Bold, termion::cursor::Goto(c.x, c.y), c.character, style::Reset);
}

fn print_char_old(c: Char) {
    let g = c.intensity * 60.0 + 100.0;
    print!("{}{}{}", color::Fg(color::Rgb(0, g as u8, 0)), termion::cursor::Goto(c.x, c.y), c.character);
}

fn clear(c: Char) {
    print!("{} ", termion::cursor::Goto(c.x, c.y));
}
