TiTr | Time Tracker

- titr je napsaný v Rust a je dostupný jako binárka pro Linux, MacOS i Windows
- umí trackovat čas strávený na různých úkolech / vytvářet minutku v CLI (napříč různámi terminály)
- pokud se spustí jak minutka nebo tracking úkolu objeví v systémové liště (tray) ikona s časem
- data jsou ukládána lokálně v JSON formátu (žádné cloudové služby)
- příkazy programu:

```bash
    titr start "Název úkolu"
    titr stop | pause // pozastaví aktuální úkol
    titr status // zobrazí stav aktuálního úkolu
    titr tasks // zobrazí seznam všech úkolů
    titr report // zobrazí report o stráveném čase
      titr report --today // report za poslední den
      titr report -week // report za poslední týden
      titr report --month // report za poslední měsíc
    titr min --set 10 // uloží výchozí čas minutky na 10 minut
    titr min 5 // spustí minutku na 5 minut
    titr min // spustí výchozí minutku
    titr day // zobrazí report za dnešní den ukoly a minutky
    titr clear // smaže všechna data (potvrzení před smazáním)
    titr help // zobrazí nápovědu
```

- příklad souboru:

```json
  {
  "tasks": [
  {
  "name": "Název úkolu",
  "sessions": [
  {
  "start": "2024-06-01T10:00:00Z",
  "end": "2024-06-01T11:00:00Z"
  },
  {
  "start": "2024-06-01T12:00:00Z",
  "end": "2024-06-01T13:30:00Z"
  }
  ]
  },
  {
  "minutes": {
  "default": 10,
  "session": [
  {
  "start": "2024-06-01T14:00:00Z",
  "end": "2024-06-01T14:10:00Z"
  }
  }
  ]
  }
```

- nápady do budoucna
  - možnost kategorizace úkolů (projekt A, projekt B)
  - možnost exportu reportů do CSV nebo PDF
  - integrace s kalendářem (Google Calendar, Outlook)
  - možnost nastavení připomínek pro pauzy nebo konce minutek
