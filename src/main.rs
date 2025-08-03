use std::{time::{SystemTime, UNIX_EPOCH}};
use rand::Rng;
use rand::seq::IndexedRandom;
use std::io;

fn main() {
    loop {
        start();
        println!("Ещё раз? Y/N");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().to_uppercase().as_str() {
            "Y" => continue,
            "N" => {
            println!("Спасибо что воспользовались данной программой!");
            break;
            },
        _ => println!("Неверный ввод, попробуйте ещё раз"),
        }
    }

}


fn start(){
    println!("Чем хотите воспользоваться сегодня:
    1: Испытание удачи,
    2: Магический шар девятка.");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    let value: i32 = input.trim().parse().unwrap_or_else(|_| {
        eprintln!("Ошибка! Использую значение по умолчанию: 1");
        1
    });
    match value{
        1 => luck(),
        2 => magic_ball(),
        _ => println!("Неизвестный вариант"),
    }
}



fn magic_ball() {
    println!("Задай вопрос вселенной!");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    let scroll = [
        "Бесспорно", "Предрешено", "Никаких сомнений", "Определённо да", "Можешь быть уверен в этом",
        "Мне кажется — «да»", "Вероятнее всего", "Хорошие перспективы", "Знаки говорят — «да»", "Да",
        "Пока не ясно, попробуй снова", "Спроси позже", "Лучше не рассказывать", "Сейчас нельзя предсказать", "Сконцентрируйся и спроси опять",
        "Даже не думай", "Мой ответ — «нет»", "По моим данным — «нет»", "Перспективы не очень хорошие", "Весьма сомнительно"
    ];
    
    let mut rng = rand::rng();
    let answer = scroll.choose(&mut rng).unwrap();
    
    println!("Твой вопрос вселенной: {}", input.trim());
    println!("И вселенная ответила: {}", answer);
}

fn luck(){
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
    0..=4850 => println!("⚡ Аномальная неудача ({} совпадений, {:.1}σ)", answer, (answer as f64 - 5000.0) / 50.0),
    4851..=4900 => println!("🔴 Большая неудача ({} совпадений, {:.1}σ)", answer, (answer as f64 - 5000.0) / 50.0),
    4901..=4950 => println!("🔶 Умеренная неудача ({} совпадений, {:.1}σ)", answer, (answer as f64 - 5000.0) / 50.0),
    4951..=4999 => println!("✅ Малая неудача ({} совпадений, ±1σ)", answer),
    5000 => println!("☯️ Идеальный баланс, совершенство, {}", answer),
    5001..=5050 => println!("✅ Малая удачи ({} совпадений, ±1σ)", answer),
    5051..=5100 => println!("🔷 Умеренная удача({} совпадений, +{:.1}σ)", answer, (answer as f64 - 5000.0) / 50.0),
    5101..=5150 => println!("🔵 Большая удача ({} совпадений, +{:.1}σ)", answer, (answer as f64 - 5000.0) / 50.0),
    5151..=10000 => println!("⚡ Аномальная удача ({} совпадений, +{:.1}σ)", answer, (answer as f64 - 5000.0) / 50.0),
    _ => println!("❓ Невозможное значение ({})", answer)
}}