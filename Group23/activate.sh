#!/bin/bash
# TOOL DI BACKUP: Attivazione
echo "-- TOOL DI BACKUP: Attivazione --"
echo ""
# Chiusura di altre eventuali istanze del programma per evitare conflitti
echo "Verifica processi di backup attivi..."
if pgrep -f "progetto_rust" > /dev/null; then
    echo "\"progetto_rust\" è in esecuzione. Terminazione del processo..."
    pkill -f "progetto_rust"
    if [ $? -ne 0 ]; then
        echo "Impossibile terminare il processo. Assicurati di avere i permessi necessari."
        sleep 6
        exit 1
    fi
    echo "Processi terminati con successo."
else
    echo "Nessun processo attivo trovato."
fi
echo
# Compilazione
echo "Compilazione del tool di backup..."
cargo build --release
if [ $? -ne 0 ]; then
    echo "Errore durante l'installazione del tool di backup."
    sleep 6
    exit 1
fi
echo "Tool di backup compilato correttamente."
echo
# Avvio in background
echo "Attivazione del tool di backup..."
nohup ./target/release/progetto_rust &>/dev/null &
echo "Tool di backup attivato correttamente."
echo
# Lettura del file di configurazione
if [ ! -f "conf.txt" ]; then
    echo "Il file \"conf.txt\" non esiste. Assicurati che sia presente nella directory corrente."
    exit 1
fi
# Estrazione dei dati dal file di configurazione
Sorgente=$(head -n 1 conf.txt)
Estensioni=$(sed -n '2p' conf.txt)
# Stampa delle informazioni di configurazione
echo "Sorgente di backup: $Sorgente."
if [ -z "$Estensioni" ]; then
    echo "Estensioni dei file da copiare: tutti i file."
else
    echo "Estensioni dei file da copiare: $Estensioni."
fi
echo

echo "Per cambiare sorgente ed estensioni modifica il file 'conf.txt'."
echo "Modalità di backup:"
echo "  1. Per impostare il backup per tutti i tipi di file lasciare vuota"
echo "     la seconda riga di 'conf.txt'."
echo "  2. Per copiare solo determinati tipi di file scrivere le estensioni"
echo "     desiderate nella seconda riga di 'conf.txt'."
echo
# Messaggio finale e chiusura
echo "Il tool è attualmente in esecuzione."
echo "Dal prossimo avvio del PC il programma sarà avviato in background."
echo
echo "Chiusura della finestra in 50 secondi."
sleep 50
exit
