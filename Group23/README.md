# Back-up di emergenza - Gruppo 23


## Introduzione

Il progetto, scritto interamente in Rust, ha come obiettivo quello di permettere l'esecuzione di un back-up d'emergenza nel caso di schermo inagibile.


## Configurazione

Per poter effettuare il backup, bisogna anzitutto definire il percorso sorgente e il tipo di file che si vuole copiare.
### Percorso sorgente
Va specificato nella **prima riga** del file [conf.txt](./conf.txt).
### Tipi di file
Si possono specificare aggiungendo la loro estensione nella **seconda riga** del file [conf.txt](./conf.txt); se la riga risulta vuota, allora verranno considerate tutte le possibili estensioni.

## Attivazione e disattivazione
Il tool necessita di [Rust](https://www.rust-lang.org/tools/install) per essere installato ed eseguito. In particolare gli script di installazione usano 'Cargo'.
### Attivazione
Una volta compilato ed eseguito per la prima volta, il tool si avvierà in bootstrap con il computer.
#### Windows
- Per attivare il tool è sufficiente eseguire [activate.bat](./activate.bat).
  - Per eseguirlo: doppio click oppure `start "" activate.bat`. 
- In alternativa, da terminale: `cargo build --release`, `start "" target\release\progetto_rust.exe`.
#### Linux/MacOS
- Per attivare il tool è sufficiente eseguire [activate.sh](./activate.sh).
  - Per eseguirlo: `chmod +x activate.sh`, `bash activate.sh`. 
- In alternativa, da terminale: `cargo build --release`, `nohup ./target/release/progetto_rust &>/dev/null &`.
  - Nota: potrebbero essere necessari pacchetti aggiuntivi, quali `libx11-dev`, `pkg-config`, `libxi-dev`, `libxtst-dev` e linker cc (per esempio `build-essential` su Ubuntu o `xcode-select` su MacOS). Se si vuole procedere da terminale e non tramite activate.sh occorre installarli manualmente.
### Disattivazione 
Disattivazione del tool: il processo si arresterà e non si avvierà più in bootstrap.
- Windows: per disattivare il tool è sufficiente eseguire [deactivate.bat](./deactivate.bat).
  - Per eseguirlo: doppio click oppure `start "" deactivate.bat`.
- Linux/MacOS: per disattivare il tool è sufficiente eseguire [deactivate.sh](./deactivate.sh).
  - Per eseguirlo: `chmod +x deactivate.sh`, `bash deactivate.sh`.  
- In alternativa, per tutti i sistemi operativi: terminare manualmente il processo `progetto_rust` ed eliminare il file eseguibile contenuto nella cartella `target\release`.


## Esecuzione del back-up
Si compone di due comandi:
 - **primo comando**: rettangolo
 - **secondo comando**: linea orizzontale

### Primo comando
Si può tracciare partendo da un angolo qualsiasi dello schermo.
Al fine di catturare correttamente la gesture, è necessario tracciare **4 segmenti distinti e consecutivi** lungo i bordi dello schermo (con una tolleranza di 50 pixel).
Per tracciare un segmento bisogna tenere premuto il tasto sinistro del mouse e tirare una linea fino all'altro angolo; a quel punto, il tasto deve essere rilasciato. Ripetere per tutti gli altri segmenti
Se il rettangolo è stato tracciato correttamente, apparirà un pop-up che confermerà l'acquisizione e inviterà l'utente a tracciare il secondo comando entro alcuni secondi.

### Secondo comando
È sufficiente tracciare una linea orizzontale partendo da un punto qualsiasi (purchè copra almeno il 90% della lunghezza dello schermo).

### Flusso di esecuzione
1. Il programma aspetta che il rettangolo venga tracciato correttamente
2. Lettura del file di configurazione e verifica della presenza un chiavetta USB/disco.
   1. Se presenti, viene mostrato il pop-up di conferma acquisizione
   2. Altrimenti, viene mostrato un pop-up che invita a riavviare la procedura, quindi tornare al punto 1)
3. Il programma aspetta che il secondo comando venga tracciato correttamente entro la finestra temporale.
   1. Se ciò avviene, viene mostrato un pop-up che avvisa dell'inizio del back-up
   2. Altrimenti, viene mostrato un pop-up che invita a riavviare la procedura, quindi tornare al punto 1)
4. Viene mostrato un pop-up con alcuni dettagli relativi al back-up.
   Nella chiavetta sarà presente una cartella contenente i file copiati e un file di log.
   Il programma termina l'esecuzione.

## Autori

- Daniele Maragò    (s292445)
- Antonella Sarcuni (s334047)
- Alessandro Seller (s331495)
- Simone Tumminelli (s333017)
 
