use crossterm::{cursor, execute, style::Color, style::SetForegroundColor, terminal};
use crossterm::event::{self, Event, KeyCode};
use std::collections::HashSet;
use std::io::{stdout, Write};
use std::time::{Duration, Instant};

const WIDTH: i32 = 80;
const HEIGHT: i32 = 50;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Pos(i32, i32);

fn main() {
    let mut stdout = stdout();

    // Nejprve klasický vstup (mimo raw mód)
    let name = read_line("Zadej tve jmeno: ");
    let speed: u32 = loop {
        let s = read_line("Zadej rychlost (1 - 5): ");
        match s.trim().parse::<u32>() {
            Ok(v) if (1..=5).contains(&v) => break v,
            _ => println!("Neplatné číslo, zkus znovu."),
        }
    };

    // Inicializace
    let mut player = Pos(8, 7);
    let goal = Pos(8, 37); // "Cil" v původním kódu (XM2,YM2)
    let mut enemies = vec![Pos(40, 20), Pos(40, 40), Pos(60, 47), Pos(10, 47)];
    let mut obstacles: HashSet<Pos> = HashSet::new();

    // Naplnění překážek (analogicky k Pascalu)
    fill_static_obstacles(&mut obstacles);

    // Okraje a status sloupec
    for x in 2..=80 { obstacles.insert(Pos(x, 2)); obstacles.insert(Pos(x, 48)); }
    for y in 3..=47 { obstacles.insert(Pos(3, y)); obstacles.insert(Pos(79, y)); obstacles.insert(Pos(2, y)); obstacles.insert(Pos(80, y)); }

    // Nastavení terminálu
    execute!(stdout, terminal::EnterAlternateScreen).unwrap();
    terminal::enable_raw_mode().unwrap();
    execute!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();

    let start = Instant::now();

    loop {
        // Vykreslení statického layoutu (reálný čas od startu)
        let display_secs = start.elapsed().as_secs();
        draw_static(&mut stdout, &name, speed, display_secs, player);
        draw_goal(&mut stdout, goal);
        draw_obstacles(&mut stdout, &obstacles);
        draw_player(&mut stdout, player);
        draw_enemies(&mut stdout, &enemies);
        stdout.flush().unwrap();

        let elapsed_secs = start.elapsed().as_secs();
        // Podmínky ukončení
        if player == goal { end_message(&mut stdout, "!!! Gratuluji, prosel jsi 1. kolo !!!"); break; }
        if elapsed_secs >= 120 { end_message(&mut stdout, "!!! Herni cas vyprsel (120s) !!!"); break; }
        if enemies.iter().any(|&e| e == player) {
            end_message(&mut stdout, &format!("!!! Hrac {name} porazen !!!")); break;
        }

        // Delay podle rychlosti (čas se bere reálně přes Instant)
        let frame_delay = match speed { 1 => 100, 2 => 80, 3 => 60, 4 => 40, 5 => 20, _ => 50 }; // ms

        // Vstup (poll s timeoutem frame_delay)
        if event::poll(Duration::from_millis(frame_delay as u64)).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                match key.code {
                    KeyCode::Esc => { end_message(&mut stdout, "Ukonceno (ESC)"); break; },
                    KeyCode::Up => try_move(&mut player, 0, -2, &obstacles),
                    KeyCode::Down => try_move(&mut player, 0, 2, &obstacles),
                    KeyCode::Left => try_move(&mut player, -2, 0, &obstacles),
                    KeyCode::Right => try_move(&mut player, 2, 0, &obstacles),
                    _ => {}
                }
            }
        }

        // Pohyb nepřátel směrem k hráči (stejná strategie: horizontálně, pak vertikálně)
        for e in enemies.iter_mut() {
            step_towards(e, player, &obstacles);
        }

        // Prevence srážky nepřátel (posun o +1 na X jako v Pascalu)
        avoid_enemy_collisions(&mut enemies, &obstacles);

        // Vymazání pro další frame (zde jen vyčistíme, protože vše překreslujeme)
        execute!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();
    }

    // Závěrečný text
    final_screen(&mut stdout);

    // Návrat k normálnímu režimu
    terminal::disable_raw_mode().unwrap();
    execute!(stdout, terminal::LeaveAlternateScreen).unwrap();
    println!("Hotovo za {:?}", start.elapsed());
}

fn read_line(prompt: &str) -> String {
    use std::io::{stdin, stdout};
    print!("{prompt}");
    let _ = stdout().flush();
    let mut s = String::new();
    stdin().read_line(&mut s).ok();
    s.trim_end().to_string()
}

fn fill_static_obstacles(o: &mut HashSet<Pos>) {
    // Přepis bloků "Line 1..31" z Pascalu
    for x in 3..=55 { o.insert(Pos(x, 26)); }
    for y in 15..=26 { o.insert(Pos(35, y)); }
    for x in 3..=15 { o.insert(Pos(x, 20)); }
    for y in 3..=15 { o.insert(Pos(15, y)); }
    for x in 21..=30 { o.insert(Pos(x, 16)); }
    for y in 6..=20 { o.insert(Pos(21, y)); }
    for y in 24..=25 { o.insert(Pos(11, y)); }
    for x in 10..=15 { o.insert(Pos(x, 12)); }
    for y in 4..=10 { o.insert(Pos(41, y)); }
    for y in 12..=20 { o.insert(Pos(41, y)); }
    for x in 45..=55 { o.insert(Pos(x, 12)); }
    for x in 59..=74 { o.insert(Pos(x, 18)); }
    for y in 4..=15 { o.insert(Pos(65, y)); }
    for y in 10..=17 { o.insert(Pos(73, y)); }
    for x in 62..=70 { o.insert(Pos(x, 24)); }
    for y in 30..=42 { o.insert(Pos(15, y)); }
    for x in 3..=6 { o.insert(Pos(x, 32)); }
    for x in 12..=15 { o.insert(Pos(x, 32)); }
    for y in 26..=36 { o.insert(Pos(29, y)); }
    for y in 30..=47 { o.insert(Pos(35, y)); }
    for y in 33..=45 { o.insert(Pos(45, y)); }
    for y in 27..=45 { o.insert(Pos(55, y)); }
    for x in 55..=73 { o.insert(Pos(x, 28)); }
    for x in 60..=79 { o.insert(Pos(x, 32)); }
    for x in 55..=72 { o.insert(Pos(x, 36)); }
    for x in 40..=45 { o.insert(Pos(x, 36)); }
    for x in 45..=48 { o.insert(Pos(x, 40)); }
    for x in 23..=33 { o.insert(Pos(x, 40)); }
    for x in 27..=35 { o.insert(Pos(x, 10)); }
    for x in 3..=10 { o.insert(Pos(x, 40)); }
    for y in 40..=47 { o.insert(Pos(19, y)); }
}

fn draw_static(stdout: &mut std::io::Stdout, name: &str, speed: u32, elapsed_secs: u64, player: Pos) {
    execute!(stdout, cursor::MoveTo(10, 0), SetForegroundColor(Color::White)).ok();
    print!("Jmeno: {name}");
    execute!(stdout, cursor::MoveTo(26, 0)).ok();
    print!("Rychlost: ");
    execute!(stdout, SetForegroundColor(Color::Cyan)).ok();
    print!("{speed}");
    execute!(stdout, SetForegroundColor(Color::White), cursor::MoveTo(40, 0)).ok();
    execute!(stdout, SetForegroundColor(Color::White)).ok();
    print!("Cas: ");
    execute!(stdout, SetForegroundColor(Color::Cyan)).ok();
    print!("{} / 120s", elapsed_secs);
    execute!(stdout, SetForegroundColor(Color::White), cursor::MoveTo(55, 0)).ok();
    print!("Souradnice X:{} Y:{} ", player.0, player.1);
}

fn draw_goal(stdout: &mut std::io::Stdout, goal: Pos) {
    execute!(stdout, cursor::MoveTo(goal.0 as u16, goal.1 as u16), SetForegroundColor(Color::White)).ok();
    print!("Cil");
}

fn draw_player(stdout: &mut std::io::Stdout, p: Pos) {
    execute!(stdout, cursor::MoveTo(p.0 as u16, p.1 as u16), SetForegroundColor(Color::Cyan)).ok();
    print!("☺");
}

fn draw_enemies(stdout: &mut std::io::Stdout, enemies: &[Pos]) {
    for &e in enemies {
        execute!(stdout, cursor::MoveTo(e.0 as u16, e.1 as u16), SetForegroundColor(Color::Red)).ok();
        print!("@");
    }
}

fn draw_obstacles(stdout: &mut std::io::Stdout, obs: &HashSet<Pos>) {
    for pos in obs.iter() {
        execute!(stdout, cursor::MoveTo(pos.0 as u16, pos.1 as u16), SetForegroundColor(Color::DarkYellow)).ok();
        print!("█");
    }
}

fn try_move(p: &mut Pos, dx: i32, dy: i32, obstacles: &HashSet<Pos>) {
    let nx = p.0 + dx;
    let ny = p.1 + dy;
    if nx >= 0 && nx <= WIDTH && ny >= 0 && ny <= HEIGHT && !obstacles.contains(&Pos(nx, ny)) {
        p.0 = nx; p.1 = ny;
    }
}

fn step_towards(e: &mut Pos, target: Pos, obstacles: &HashSet<Pos>) {
    if e.0 != target.0 {
        let dir = if e.0 < target.0 { 1 } else { -1 };
        let nx = e.0 + dir;
        if !obstacles.contains(&Pos(nx, e.1)) { e.0 = nx; }
    } else if e.1 != target.1 {
        let dir = if e.1 < target.1 { 1 } else { -1 };
        let ny = e.1 + dir;
        if !obstacles.contains(&Pos(e.0, ny)) { e.1 = ny; }
    }
}

fn avoid_enemy_collisions(enemies: &mut Vec<Pos>, obstacles: &HashSet<Pos>) {
    for i in 0..enemies.len() {
        for j in (i + 1)..enemies.len() {
            if enemies[i] == enemies[j] {
                let nx = enemies[j].0 + 1;
                if !obstacles.contains(&Pos(nx, enemies[j].1)) { enemies[j].0 = nx; }
            }
        }
    }
}

fn end_message(stdout: &mut std::io::Stdout, msg: &str) {
    execute!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();
    execute!(stdout, cursor::MoveTo(10, 20), SetForegroundColor(Color::White)).unwrap();
    println!("{msg}");
}

fn final_screen(stdout: &mut std::io::Stdout) {
    execute!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();
    execute!(stdout, cursor::MoveTo(5, 20), SetForegroundColor(Color::White)).unwrap();
    println!("Doufam, ze jste se dobre pobavili...");
    execute!(stdout, cursor::MoveTo(5, 22)).unwrap();
    println!("Pripadne dotazy ci jine informace: rapmaker@seznam.cz");
    execute!(stdout, cursor::MoveTo(5, 24)).unwrap();
    println!("Naprogramoval (Rust port): Jan Pikl + Copilot");
}
