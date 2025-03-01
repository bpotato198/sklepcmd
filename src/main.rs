use std::io::stdin;
fn main() {
    let version = "0.0122025";
    let mut cmd = String::new();
    println!("Generator komend do MC podaj komende napisany w Rust");
    println!("Wersja {}", version);

    println!("===summon===tp===msg===random");
    stdin().read_line(&mut cmd).expect("failed to readline!");
    let cmd = cmd.trim();

    if cmd == "msg" {
        let mut prefixmsg = String::new();
        let mut to = String::new();
        let mut text = String::new();
        println!("Podaj do kogo");
        println!("=@p - Do najbliższego gracza, @a - Do wszystkich graczy   @r - do randomowego gracza albo do niku=");
        stdin().read_line(&mut to).expect("failed to readline");
        stdin()
            .read_line(&mut prefixmsg)
            .expect("failed to readline!");
        println!("podaj wiadomosć");
        stdin().read_line(&mut text).expect("failed to readline!");
        println!("twoja komenda to:");
        println!("/msg {} {} ", to, text);
    }

    if cmd == "summon" {
        let mut summon = String::new();
        println!("podaj nazwe mobu jakiego chcesz zrespawnowac (np. blaze albo bee)");
        stdin().read_line(&mut summon).expect("failed to readline");
        println!("twoja komenda");
        println!("/summon minecraft:{}", summon);
        exit();
    }

    if cmd == "tp" {
        tp();
    }
    fn exit() {
        let mut wyl = String::new();
        stdin().read_line(&mut wyl).expect("failed to readline");
    }
    if cmd == "debug" {
        error();
    }
    fn error() {
        println!("Nic nie podano albo komenda nie wlasciwa!");
    }

    fn tp() {
        let mut cords = String::new();
        let mut who = String::new();
        println!("Podaj kogo tp");
        stdin().read_line(&mut who).expect("failed to readline");
        println!("podaj do kogo albo kordy");
        stdin().read_line(&mut cords).expect("failed to readline!");
        println!("komenda to: ");
        println!("/tp {} {}", who, cords);
        exit();
    }
}
