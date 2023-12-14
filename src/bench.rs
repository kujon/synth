mod voices;

use voices::voices::Voices;
use voices::voices2::Voices2;
use rdev::Key;
use std::time::{Duration, SystemTime};

fn main() {
    let keys: Vec<Key> = vec![
        Key::F1,
        Key::F2,
        Key::F3,
        Key::F4,
        Key::F5,
        Key::F6,
        Key::F7,
        Key::F8,
        Key::F9,
        Key::F10,
        Key::Num0,
        Key::Num1,
        Key::Num2,
        Key::Num3,
        Key::Num4,
        Key::Num5,
        Key::Num6,
        Key::Num7,
        Key::Num8,
        Key::Num9,
    ];

    let mut test_data: Vec<Key> = vec![];

    for _ in 0..100 {
        for n in 0..10 {
            for m in 0..4 {
                for q in 0..3 {
                    test_data.push(keys[n + m + q]);
                }
            }
        }
    }

    let start = SystemTime::now();

    for n in 0..10000 {
        let mut voices:Voices<usize> = Voices::new(4);
        let mut time = SystemTime::UNIX_EPOCH;
    
        for k in &test_data {
            voices.play_with_time(*k, time, 0);

            if n % 5 == 0 {
                voices.stop(*k);
            }

            time = time.checked_add(Duration::new(1, 0)).unwrap();
        }
    }

    let end = SystemTime::now();

    println!("{:?}", end.duration_since(start));

    let start = SystemTime::now();

    for n in 0..10000 {
        let mut voices2:Voices2<usize> = Voices2::new(4);
    
        for k in &test_data {
            voices2.play(*k, 0);

            if n % 5 == 0 {
                voices2.stop(*k);
            }
        }
    }

    let end = SystemTime::now();

    println!("{:?}", end.duration_since(start));

}