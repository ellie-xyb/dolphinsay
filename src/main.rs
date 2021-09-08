struct Animal {
    sound: String,
    count: usize,
}

impl ToString for Animal {
    fn to_string(&self) -> String {
        let greeting = "🐠".repeat(self.count);
        format!("{} {}", greeting, self.sound)
    }
}

fn dolphin_say(message: impl ToString) {
    let m = message.to_string();
    let length = m.len() + 7;
    println!("{}", "-".repeat(length));
    println!("<  {}!  >", m);
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
    let animal_sound = Animal {
        sound: "eeeeeeee".to_string(),
        count: 5,
    };
    animal_sound.to_string();
    dolphin_say(animal_sound);
} 
