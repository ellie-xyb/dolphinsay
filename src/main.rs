fn dolphin_say(message: &str) {
    let length = message.len() + 7;
    println!("{}", "-".repeat(length));
    println!("<  {}!  >", message);
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
    let sound = "eeeee";
    // println!("{}!", sound);
    dolphin_say(sound);
}
