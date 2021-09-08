use promptly::prompt;

struct Animal {
    sound: String,
    count: usize,
}

impl ToString for Animal {
    fn to_string(&self) -> String {
        let food = "🐠".repeat(self.count);
        format!("Enjoy {}! 🍽 {}", self.sound, food)
    }
}

fn dolphin_say(message: impl ToString) {
    let m = message.to_string();
    let length = m.len() + 7;
    println!("{}", "-".repeat(length));
    println!("<  {}  >", m);
    println!("{}", "-".repeat(length));
    println!("    \\                                      _");
    println!("     \\                                _.-~~.)");
    println!("      \\         _.--~~~~~---....__  .' . .,'");
    println!("       \\      ,'. . . . . . . . . .~- ._ (");
    println!("        \\    ( .. .g. . . . . . . . . . .~-._");
    println!("          .~__.-~    ~`. . . . . . . . . . . -.");
    println!("          `----..._      ~-=~~-. . . . . . . . ~-.");
    println!("                    ~-._   `-._ ~=_~~--. . . . . .~.");
    println!("                     | .~-.._  ~--._-.    ~-. . . . ~-.");
    println!("                      \\ .(   ~~--.._~'       `. . . . .~-.                ,");
    println!("                       `._\\         ~~--.._    `. . . . . ~-.    .- .   ,'/");
    println!("       _  . _ . -~\\        _ ..  _          ~~--.`_. . . . . ~-_     ,-','`  .");
    println!("                    ` ._           ~                ~--. . . . .~=.-'. /. `");
    println!("              - . -~            -. _ . - ~ - _   - ~     ~--..__~ _,. /   \\  - ~");
    println!("                     . __ ..                   ~-               ~~_. (  `");
    println!("       )`. _ _               `-       ..  - .    . - ~ ~ .    \\    ~-` ` `  `. _");
    println!("             _ Seal _");
}

fn main() {
    let name = prompt("What's your name").unwrap();
    let num: usize = prompt("How many fishes you want to eat today").unwrap();
    let animal_sound = Animal {
        sound: name,
        count: num,
    };
    animal_sound.to_string();
    dolphin_say(animal_sound);
}
