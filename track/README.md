Základní funkce (MVP - Minimum Viable Product)
Start/Stop: Příkaz track start "Název úkolu" začne měřit čas a track stop ho ukončí.

Persistence: Data se ukládají do souboru (např. JSON, SQLite nebo jednoduchý textový soubor), aby nezmizela po vypnutí terminálu.

Status: Příkaz track status, který ukáže, na čem právě pracuješ a jak dlouho už to trvá.

Log: Výpis historie (např. dnes odpracováno: 6h 20m).

Jak na to v Rustu? (Doporučené knihovny)
chrono: Naprostý standard pro práci s datem a časem v Rustu. Pomůže ti počítat rozdíly mezi startem a koncem.

serde + serde_json: Pokud se rozhodneš ukládat data do JSONu. Je to nejjednodušší cesta pro začátek.

clap: Pro parsování argumentů (aby tvoje CLI vypadalo profesionálně a mělo nápovědu přes --help).

indicatif: Pokud chceš v terminálu zobrazit běžící stopky nebo progress bar.

Pokročilejší nápady (Vylepšení pro webového vývojáře)
Export do JSONu/CSV: abys mohl data naimportovat do fakturačního systému nebo Excelu.

Integrace s prohlížečem: Rust aplikace by mohla běžet na pozadí a nabízet jednoduché lokální API, na které by se mohl napojit tvůj frontend (např. lišta v Reactu, co ukazuje aktuální úkol).

Detekce nečinnosti (Idle detection): Pokud se na počítači nic neděje 5 minut, Rust se tě po návratu zeptá, jestli máš ten čas započítat.

TUI (Terminal User Interface): Místo jen řádkových příkazů udělat grafické prostředí přímo v terminálu pomocí knihovny ratatui. Viděl by jsi tam grafy produktivity nebo seznam úkolů v tabulce.

Příklad, jak by mohl vypadat datový model:
Rust
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug)]
struct TimeEntry {
task_name: String,
start_time: DateTime<Utc>,
end_time: Option<DateTime<Utc>>, // None znamená, že úkol stále běží
}
Proč je to super na učení Rustu?
Vlastnictví (Ownership): Narazíš na to, jak předávat názvy úkolů mezi funkcemi.

Chybové stavy: Co když uživatel napíše stop, ale žádný úkol neběží? Rust tě donutí tyto stavy ošetřit (Option a Result).

Souborový systém: Naučíš se, kam v počítači korektně ukládat konfigurační data (např. do složky ~/.config/my_tracker).

když to v jedné konzoli pustím a v jiné vypnu tak to funguje ?

To je skvělá technická otázka! Krátká odpověď zní: Ano, ale musíš to správně navrhnout přes sdílený soubor.

Protože Rust (stejně jako ostatní jazyky) standardně nepouští programy tak, aby "vzájemně viděly do své paměti", musíš použít princip persistence (trvalého uložení).

Jak to funguje v praxi:
V první konzoli napíšeš tracker start "Refaktorování":

Program vytvoří soubor (např. current_task.json).

Uloží do něj název úkolu a aktuální čas (timestamp).

Program skončí (neběží dál v pozadí, jen zanechal "stopu" na disku).

Ve druhé konzoli napíšeš tracker stop:

Program se spustí, podívá se, jestli existuje current_task.json.

Načte startovní čas, spočítá rozdíl oproti aktuálnímu času.

Výsledek zapíše do historie (např. history.db) a smaže current_task.json.

Na co si dát v Rustu pozor (The Rust Way):
Pokud bys chtěl mít v první konzoli živý odpočet (sekundy, co naskakují), a ve druhé to chtěl stopnout, je to o úroveň složitější, ale zajímavější:

1. File Locking (Zamykání souborů)
   Co když omylem napíšeš stop ve dvou oknech naráz? Rust má skvělé knihovny jako fs2, které ti dovolí soubor "zamknout". Druhý proces pak musí počkat, až ten první skončí, což zabrání poškození dat.

2. Signal Handling
   Pokud chceš, aby program v první konzoli běžel a "poslouchal", až ho někdo vypne (třeba přes Ctrl+C), můžeš v Rustu odchytávat signály operačního systému (pomocí crate ctrlc). Při vypnutí pak stihneš korektně uložit data na disk.

Jak by vypadala struktura JSONu:
JSON
{
"task_name": "Programování v Rustu",
"started_at": "2023-10-27T14:30:00Z",
"is_active": true
}
Tip pro tvůj vývoj:
Pokud to chceš brát vážně, doporučuji ukládat data do standardních složek pro aplikace. V Rustu na to existuje skvělý crate directories, který ti automaticky najde správnou cestu:

Linux: ~/.local/share/tvuj-tracker

Windows: AppData/Roaming/tvuj-tracker

macOS: Library/Application Support/tvuj-tracker

k tomu by šlo udělat i vizuální rozhraní jednoduše ne?

Rozhodně! V terminálu máš v podstatě dvě cesty, jak to udělat. Buď zůstaneš u klasického textového výstupu, nebo vytvoříš tzv. TUI (Terminal User Interface), což vypadá jako aplikace v okně, ale pořád běží v konzoli (podobně jako htop nebo vim).

Tady jsou dvě úrovně, jak to vizuálně posunout:

1. Úroveň: "Hezké CLI" (Knihovna indicatif a tabled)
   Pokud nechceš stavět celé rozhraní, ale chceš, aby to vypadalo profesionálně:

Progress bary: Když běží čas, můžeš tam mít lištu, která se hýbe.

Tabulky: Historii úkolů vypíšeš do krásně zarovnané tabulky s ohraničením.

Barvy: Start bude zelený, stop červený, názvy projektů třeba tučně modré.

2. Úroveň: "Plnohodnotné TUI" (Knihovna ratatui)
   Tohle je teď v Rust komunitě naprostý hit. ratatui (nástupce staršího tui-rs) ti dovolí rozdělit terminál na sekce (layouty).

Vlevo uvidíš seznam projektů.

Vpravo nahoře poběží velké digitální hodiny s aktuálním úkolem.

Dole bude graf (třeba sloupcový), kolik jsi toho odpracoval v posledních 7 dnech.

Jak by to fungovalo technicky?
Aby to fungovalo napříč konzolemi (jak ses ptal předtím), architektura by vypadala takto:

Storage Layer (Soubor/DB): Základem je stále ten soubor na disku (např. data.json). Ten slouží jako "jediný zdroj pravdy".

TUI App: Tahle aplikace se po spuštění "přisaje" na ten soubor. Každou sekundu (nebo při změně) soubor přečte a překreslí rozhraní.

CLI příkazy: Pořád můžeš mít možnost napsat track stop v jiném okně. TUI aplikace si všimne, že se soubor změnil, a okamžitě se aktualizuje (přestane počítat).

Proč zkusit ratatui?
Okamžitá odezva: Naučíš se pracovat s "event loop" (smyčka, která čeká na stisknutí klávesy nebo tiknutí hodin).

Layouty: Pochopíš, jak Rust řeší vykreslování do terminálu pomocí "bufferů".

Wow efekt: Vypadá to neuvěřitelně cool a pro webového vývojáře je to zajímavá změna oproti klasickému DOMu v prohlížeči.

S čím začít?
Doporučuji nejdřív zprovoznit tu logiku ukládání do souboru (příkazy start/stop) a pak na to zkusit "nasadit" to vizuální rozhraní.

Chceš, abych ti ukázal, jak by vypadal úplně základní "Ahoj světe" v ratatui, abys viděl, jak se v tom kreslí?

myslel jsem že by to bylo v terminálu ale i jako běžný program

To je skvělá úvaha! V Rustu máš tu výhodu, že si můžeš vybrat, jak moc „systémově“ nebo „vizuálně“ na to půjdeš. Pokud chceš, aby to byl běžný program (okno s tlačítky), ale zároveň to mohl ovládat z terminálu, existují tři hlavní cesty:

1. Hybridní model (CLI + GUI)
   Vytvoříš jednu binárku, která se chová podle toho, jak ji spustíš.

Když napíšeš v terminálu tracker start, provede se kód pro zápis do souboru.

Když na ikonu klikneš dvakrát (nebo napíšeš tracker gui), otevře se klasické okno.

Knihovna pro Rust: Tauri. Tauri je pro webového vývojáře jako stvořené. Je to alternativa k Electronu, ale mnohem lehčí a bezpečnější.

Backend: Rust (řeší zápis do souborů, logiku, systémové hodiny).

Frontend: HTML/CSS/JS (React, Vue, nebo klidně čisté JS), které běží v nativním okně OS.

2. Rust-Native GUI
   Pokud se chceš úplně zbavit HTML/CSS a zkusit něco nového, můžeš použít knihovny, které kreslí okna přímo pomocí grafické karty.

Iced: Inspirováno jazykem Elm. Je to velmi populární a „rustové“.

egui: Extrémně jednoduché na rozjetí, skvělé pro utility a nástroje.

Výhoda je, že výsledkem je jeden malý .exe (nebo binárka pro Mac/Linux), který nemá žádné závislosti.

3. Jak zajistit, aby to „všichni viděli“?
   Aby tvoje CLI, TUI i GUI ukazovaly stejná data, musíš mít dobře navrženou Data Layer.

Shared JSON/SQLite: Všechny verze programu (CLI i GUI) čtou ze stejného souboru v AppData nebo ~/.config.

Local Server (volitelné): Tvůj program může běžet na pozadí jako „démon“ a CLI i GUI se k němu připojují přes lokální port (např. přes HTTP nebo WebSockety). Pak by se čas aktualizoval v reálném čase všude naráz.

Která cesta tě láká víc?
Cesta Technologie Proč do toho jít?
Tauri (Webview) Rust + React/Vue Využiješ to, co už umíš z webu, a přidáš k tomu výkon Rustu.
Iced/egui (Native) Čistý Rust Naučíš se, jak se staví opravdové desktopové aplikace bez prohlížeče.
Ratatui (TUI) Terminál Zůstaneš v „hacker“ prostředí, je to nejrychlejší na ovládání.
Mám ti zkusit ukázat, jak by vypadala struktura projektu pro Tauri, abys viděl, jak se tam propojuje tvůj webový frontend s Rustovým kódem pro měření času?

kdyz mam nejaky batch - terminál pod dockerem kde mam projekt a klasicky tam pouštím třeba yarn start:projekt.

jak tam pridam ten rust program?

Pokud pracuješ v Dockeru, kde ti běží frontend (přes Yarn), máš v podstatě tři hlavní způsoby, jak tam ten svůj Rust program „dostat“. Záleží na tom, jestli chceš, aby Rust běžel uvnitř toho stejného kontejneru, nebo vedle něj.

Tady jsou možnosti seřazené od nejjednodušší po nejčistší:

1. "Vše v jednom" (Nejrychlejší pro vývoj)
   Upravíš svůj stávající Dockerfile tak, aby se do něj nainstaloval Rust a tvoje binárka se zkompilovala přímo tam.

Postup:

Do Dockerfile přidáš instalaci Rustu (nebo použiješ multi-stage build).

Spustíš kontejner.

Otevřeš si další terminál do běžícího kontejneru: docker exec -it <id_kontejneru> /bin/bash

Uvnitř kontejneru pak normálně spouštíš svůj Rust program (např. cargo run).

Nevýhoda: Docker image bude obrovský, protože obsahuje Node.js i Rust tooling.

2. Docker Compose (Nejprofesionálnější)
   Pokud tvůj Rust program funguje jako Fake Data API nebo nějaká služba, nejlepší je dát ho do vlastního kontejneru a propojit je pomocí docker-compose.yml.

Příklad docker-compose.yml:

YAML
services:
frontend:
build: .
command: yarn start:projekt
ports: - "3000:3000"
volumes: - .:/app # Sdílení souborů pro persistence dat

rust-tracker:
image: rust:latest
volumes: - ./rust-src:/app # Tvůj Rust kód - .:/shared-data # Tady si Rust i Yarn mohou sahat na stejný soubor (např. JSON)
working_dir: /app
command: cargo run --release
Výhoda: Každý program má své izolované prostředí, ale díky Volumes mohou oba sahat na stejný soubor (např. ten data.json pro tvůj time tracker).

3. "Sidecar" Binárka
   Pokud tvůj Rust program nepotřebuje v Dockeru běžet neustále (je to jen utilita jako formatter nebo generator), můžeš ho zkompilovat lokálně (na svém stroji) a pak ho do Dockeru jen "přihodit" jako hotový soubor.

Zkompiluješ v Rustu: cargo build --release

V docker-compose.yml nebo při spuštění namapuješ složku target/release do kontejneru.

Pak uvnitř Dockeru prostě voláš ./my-rust-tool.

Jak to udělat, aby se ty dva programy "viděly"?
Pokud chceš, aby tvůj yarn start (frontend) mohl komunikovat s tím Rust programem (např. pro zobrazení času), máš dvě cesty:

Sdílený soubor (Volume): Oba kontejnery mapují stejnou složku na tvém disku. Rust do ní zapisuje time.json, Frontend ho čte.

Síť (Internal Network): Docker Compose vytvoří mezi kontejnery síť. Tvůj frontend v JS bude volat fetch('http://rust-tracker:4000/api'). rust-tracker je v tomto případě hostname, který Docker automaticky vyřeší.

Co zkusit jako první?
Pokud s Rustem v Dockeru začínáš, doporučuji Option 1 (Docker exec). Je to nejmíň práce s nastavováním sítě.

Přidej do Dockerfilu: RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

Restartuj kontejner.

Připoj se dovnitř a hraj si.
