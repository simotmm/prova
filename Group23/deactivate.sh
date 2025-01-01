#!/bin/bash

# TOOL DI BACKUP: Disattivazione
echo "-- TOOL DI BACKUP: Disattivazione --"
echo

# Terminazione dei processi
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

# Eliminazione degli eseguibili
if [ -f "target/release/progetto_rust" ]; then
    rm -f target/release/progetto_rust
fi
if [ -f "target/debug/progetto_rust" ]; then
    rm -f target/debug/progetto_rust
fi

# Messaggio finale e chiusura
echo "Il tool non è più in esecuzione."
echo "Il programma non sarà più avviato in background all'avvio del PC."
echo
echo "Chiusura della finestra in 30 secondi."
sleep 30
exit
