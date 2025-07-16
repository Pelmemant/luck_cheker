use std::time::{SystemTime, UNIX_EPOCH};
use rand::Rng;
use std::io;

fn main() {
    println!("Введите 0 или 1:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    let value: i32 = input.trim().parse().unwrap_or_else(|_| {
        eprintln!("Ошибка! Использую значение по умолчанию: 1");
        1
    });
    let mut rng = rand::rng();
    let mut answer = 0;
    let since_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap() 
        .as_secs(); 
    for _ in 0..10000{
        let num = rng.random_range(1..since_epoch);
        match (num % 2, value) {
            (0, 1) => answer += 1,
            (1, 0) => answer += 1, 
            _ => ()  
        }
    };
match answer {
    0..=4850 => println!("⚡ Аномально низкий результат ({} совпадений, {:.1}σ)", answer, (answer as f64 - 5000.0) / 50.0),
    4851..=4900 => println!("🔴 Сильный негативный перекос ({} совпадений, {:.1}σ)", answer, (answer as f64 - 5000.0) / 50.0),
    4901..=4950 => println!("🔶 Умеренный негативный перекос ({} совпадений, {:.1}σ)", answer, (answer as f64 - 5000.0) / 50.0),
    4951..=5000 => println!("✅ Баланс неудачи ({} совпадений, ±1σ)", answer),
    5001..=5050 => println!("✅ Баланс удачи ({} совпадений, ±1σ)", answer),
    5051..=5100 => println!("🔷 Умеренный позитивный перекос ({} совпадений, +{:.1}σ)", answer, (answer as f64 - 5000.0) / 50.0),
    5101..=5150 => println!("🔵 Сильный позитивный перекос ({} совпадений, +{:.1}σ)", answer, (answer as f64 - 5000.0) / 50.0),
    5151..=10000 => println!("⚡ Аномально высокий результат ({} совпадений, +{:.1}σ)", answer, (answer as f64 - 5000.0) / 50.0),
    _ => println!("❓ Невозможное значение ({})", answer)
}

}

