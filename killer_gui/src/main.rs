use eframe::{egui, egui::{Color32, Key, Pos2, Rect, Stroke}};
#[cfg(target_arch = "wasm32")]
// wasm import pouze pro atribut start
use eframe::wasm_bindgen::prelude::*;
use std::collections::HashSet;
use std::time::Instant;

const WIDTH: i32 = 80; // logická šířka
const HEIGHT: i32 = 50; // logická výška
const CELL: f32 = 10.0; // velikost buňky v pixelech
const MAX_TIME_SECS: u64 = 120; // 2 minuty

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Pos(i32, i32);

struct GameApp {
    // Setup screen
    entering: bool,
    name: String,
    speed: u32,
    // Stav hry
    start: Instant,
    player: Pos,
    goal: Pos,
    enemies: Vec<Pos>,
    obstacles: HashSet<Pos>,
    finished_text: Option<String>,
    last_enemy_step: Instant,
}

impl Default for GameApp {
    fn default() -> Self {
        let mut obstacles = HashSet::new();
        fill_static_obstacles(&mut obstacles);
        // Okraje (zjednodušeno)
        for x in 2..=80 { obstacles.insert(Pos(x, 2)); obstacles.insert(Pos(x, 48)); }
        for y in 3..=47 { obstacles.insert(Pos(3, y)); obstacles.insert(Pos(79, y)); obstacles.insert(Pos(2, y)); obstacles.insert(Pos(80, y)); }
        Self {
            entering: true,
            name: String::new(),
            speed: 3,
            start: Instant::now(),
            player: Pos(8, 7),
            goal: Pos(8, 37),
            enemies: vec![Pos(40, 20), Pos(40, 40), Pos(60, 47), Pos(10, 47)],
            obstacles,
            finished_text: None,
            last_enemy_step: Instant::now(),
        }
    }
}

impl eframe::App for GameApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.entering {
            egui::Window::new("Start hry")
                .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0,0.0))
                .resizable(false)
                .show(ctx, |ui| {
                    ui.heading("Zabijak (GUI)");
                    ui.separator();
                    ui.label("Zadej jméno hráče:");
                    ui.text_edit_singleline(&mut self.name);
                    ui.label("Rychlost (1 pomalé – 5 rychlé):");
                    ui.add(egui::Slider::new(&mut self.speed, 1..=5));
                    if ui.button("Start hry").clicked() {
                        if self.name.trim().is_empty() { self.name = "Hrac".into(); }
                        self.entering = false;
                        self.start = Instant::now();
                        self.finished_text = None;
                        self.last_enemy_step = Instant::now();
                    }
                });
        }

        // Zpracování vstupu (šipky/WASD + ESC)
        ctx.input(|i| {
            if i.key_pressed(Key::Escape) {
                self.finished_text = Some("Ukonceno (ESC)".into());
            }
            let mut moved = false;
            if i.key_pressed(Key::ArrowUp) || i.key_pressed(Key::W) { moved |= self.try_move(0, -2); }
            if i.key_pressed(Key::ArrowDown) || i.key_pressed(Key::S) { moved |= self.try_move(0, 2); }
            if i.key_pressed(Key::ArrowLeft) || i.key_pressed(Key::A) { moved |= self.try_move(-2, 0); }
            if i.key_pressed(Key::ArrowRight) || i.key_pressed(Key::D) { moved |= self.try_move(2, 0); }
            if moved { /* případně něco */ }
        });

        // Frame delay / interval pro nepřátele (ne během startovací obrazovky)
        let elapsed_secs = self.start.elapsed().as_secs();
        if self.finished_text.is_none() && !self.entering {
            // Interval podle zvolené rychlosti (vyšší rychlost = kratší čekání, ale stále pomalejší než dříve)
            let interval_ms = match self.speed { 1 => 1000, 2 => 800, 3 => 650, 4 => 500, 5 => 380, _ => 700 };
            if self.last_enemy_step.elapsed().as_millis() >= interval_ms as u128 {
                self.move_enemies();
                self.last_enemy_step = Instant::now();
            }
        }

        // Podmínky ukončení
        if self.finished_text.is_none() {
            if self.player == self.goal { self.finished_text = Some("!!! Gratuluji, prosel jsi 1. kolo !!!".into()); }
            if elapsed_secs >= MAX_TIME_SECS { self.finished_text = Some("!!! Herni cas vyprsel (120s) !!!".into()); }
            if self.enemies.iter().any(|&e| e == self.player) {
                self.finished_text = Some(format!("!!! Hrac {} porazen !!!", self.name));
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(format!("Jméno: {}", self.name));
                ui.label(format!("Rychlost: {}", self.speed));
                ui.label(format!("Čas: {} / {}s", elapsed_secs, MAX_TIME_SECS));
                ui.label(format!("X:{} Y:{}", self.player.0, self.player.1));
                if ui.button("Restart").clicked() {
                    *self = GameApp::default();
                    self.entering = false;
                    self.name = "Restart".into();
                    self.last_enemy_step = Instant::now();
                }
            });
            ui.separator();

            let desired = egui::Vec2::new(WIDTH as f32 * CELL, HEIGHT as f32 * CELL);
            let (resp, painter) = ui.allocate_painter(desired, egui::Sense::hover());
            let to_screen = |p: Pos| -> Pos2 { resp.rect.min + egui::vec2(p.0 as f32 * CELL, p.1 as f32 * CELL) };
            // Background
            painter.rect_filled(resp.rect, 0.0, Color32::from_rgb(10, 10, 10));

            // Obstacles
            for o in &self.obstacles {
                painter.rect_filled(Rect::from_min_size(to_screen(*o), egui::vec2(CELL, CELL)), 0.0, Color32::from_rgb(150, 120, 40));
            }
            // Goal
            painter.rect_filled(Rect::from_min_size(to_screen(self.goal), egui::vec2(CELL * 2.0, CELL * 1.2)), 0.0, Color32::LIGHT_GREEN);
            painter.text(to_screen(self.goal) + egui::vec2(0.0, 0.0), egui::Align2::LEFT_TOP, "Cil", egui::FontId::monospace(10.0), Color32::WHITE);
            // Player
            painter.rect_filled(Rect::from_min_size(to_screen(self.player), egui::vec2(CELL, CELL)), 2.0, Color32::from_rgb(30, 170, 255));
            // Enemies
            for e in &self.enemies {
                painter.rect_filled(Rect::from_min_size(to_screen(*e), egui::vec2(CELL, CELL)), 0.0, Color32::RED);
            }
            // Grid overlay (optional lighten)
            for x in 0..=WIDTH { painter.line_segment([resp.rect.min + egui::vec2(x as f32 * CELL, 0.0), resp.rect.min + egui::vec2(x as f32 * CELL, HEIGHT as f32 * CELL)], Stroke { width: 0.5, color: Color32::DARK_GRAY }); }
            for y in 0..=HEIGHT { painter.line_segment([resp.rect.min + egui::vec2(0.0, y as f32 * CELL), resp.rect.min + egui::vec2(WIDTH as f32 * CELL, y as f32 * CELL)], Stroke { width: 0.5, color: Color32::DARK_GRAY }); }

            if let Some(msg) = &self.finished_text {
                ui.separator();
                ui.colored_label(Color32::YELLOW, msg);
            }
        });
        ctx.request_repaint();
    }
}

impl GameApp {
    fn try_move(&mut self, dx: i32, dy: i32) -> bool {
        let steps = dx.abs().max(dy.abs());
        if steps == 0 { return false; }
        let step_dx = dx.signum();
        let step_dy = dy.signum();
        let mut moved = false;
        for _ in 0..steps {
            let nx = self.player.0 + step_dx;
            let ny = self.player.1 + step_dy;
            if nx < 0 || nx > WIDTH || ny < 0 || ny > HEIGHT || self.obstacles.contains(&Pos(nx, ny)) {
                break; // narazil na zeď během dílčího kroku
            }
            self.player.0 = nx;
            self.player.1 = ny;
            moved = true;
        }
        moved
    }
    fn move_enemies(&mut self) {
        let dist_map = compute_distance_map(&self.obstacles, self.player);
        let w = (WIDTH + 1) as usize;
        let idx = |p: Pos| (p.0 as usize) + (p.1 as usize) * w;
        let dirs = [(1,0),(-1,0),(0,1),(0,-1)];
        for e in self.enemies.iter_mut() {
            if e.0 == self.player.0 && e.1 == self.player.1 { continue; }
            let mut best: Option<(u16, Pos)> = None;
            for (dx,dy) in dirs {
                let np = Pos(e.0 + dx, e.1 + dy);
                if np.0 < 0 || np.0 > WIDTH || np.1 < 0 || np.1 > HEIGHT { continue; }
                if self.obstacles.contains(&np) { continue; }
                if let Some(d) = dist_map[idx(np)] {
                    match best { Some((bd,_)) if d >= bd => {}, _ => best = Some((d,np)) }
                }
            }
            if let Some((_d, np)) = best { *e = np; }
        }
        avoid_enemy_collisions(&mut self.enemies, &self.obstacles);
    }
}

// BFS mapa vzdáleností od hráče
fn compute_distance_map(obstacles: &HashSet<Pos>, player: Pos) -> Vec<Option<u16>> {
    let w = (WIDTH + 1) as usize;
    let h = (HEIGHT + 1) as usize;
    let mut dist: Vec<Option<u16>> = vec![None; w * h];
    let mut queue: std::collections::VecDeque<Pos> = std::collections::VecDeque::new();
    let idx = |p: Pos| -> usize { (p.0 as usize) + (p.1 as usize) * w };
    if player.0 >= 0 && player.0 <= WIDTH && player.1 >= 0 && player.1 <= HEIGHT {
        dist[idx(player)] = Some(0);
        queue.push_back(player);
    }
    let dirs = [(1,0),(-1,0),(0,1),(0,-1)];
    while let Some(Pos(x,y)) = queue.pop_front() {
        let base_d = dist[idx(Pos(x,y))].unwrap();
        for (dx,dy) in dirs {
            let nx = x + dx; let ny = y + dy;
            if nx < 0 || nx > WIDTH || ny < 0 || ny > HEIGHT { continue; }
            let np = Pos(nx,ny);
            if obstacles.contains(&np) { continue; }
            let id = idx(np);
            if dist[id].is_none() { dist[id] = Some(base_d + 1); queue.push_back(np); }
        }
    }
    dist
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

fn fill_static_obstacles(o: &mut HashSet<Pos>) {
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

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Killer GUI",
        native_options,
        Box::new(|_cc| Box::new(GameApp::default()))
    )
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub async fn start() -> Result<(), eframe::wasm_bindgen::JsValue> {
    let web_options = eframe::WebOptions::default();
    let runner = eframe::WebRunner::new();
    runner.start(
        "the_canvas_id",
        web_options,
        Box::new(|_cc| Box::new(GameApp::default()))
    ).await
}

// Dummy main pro wasm bin target (vyžadováno linkováním, skutečný start řeší wasm_bindgen(start))
#[cfg(target_arch = "wasm32")]
fn main() {}