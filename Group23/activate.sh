#!/bin/bash
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

# Verifica e installazione componenti di X11 e build-essential
install_components() {
    MISSING_PACKAGES=()

    # Verifica dei pacchetti su sistemi Debian/Ubuntu
    if command -v apt &>/dev/null; then
        for pkg in build-essential libx11-dev pkg-config libxi-dev libxtst-dev; do
            if ! dpkg -s "$pkg" &>/dev/null; then
                MISSING_PACKAGES+=("$pkg")
            fi
        done

        if [ ${#MISSING_PACKAGES[@]} -gt 0 ]; then
            echo "E' necessaria l'installazione di pacchetti aggiuntivi."
            echo "I seguenti pacchetti sono mancanti: ${MISSING_PACKAGES[*]}."
            echo "Installazione pacchetti mancanti..."
            sudo apt update -y
            sudo apt install -y "${MISSING_PACKAGES[@]}"
            echo "Pacchetti installati correttamente."
            echo ""
        fi

    # Verifica dei pacchetti su sistemi Fedora/RHEL
    elif command -v dnf &>/dev/null; then
        for pkg in "@Development Tools" libX11-devel pkg-config libXi-devel libXtst-devel; do
            if [[ "$pkg" == "@Development Tools" ]]; then
                dnf group list installed | grep -q "Development Tools" || MISSING_PACKAGES+=("$pkg")
            else
                if ! rpm -q "$pkg" &>/dev/null; then
                    MISSING_PACKAGES+=("$pkg")
                fi
            fi
        done

        if [ ${#MISSING_PACKAGES[@]} -gt 0 ]; then
            echo "E' necessaria l'installazione di pacchetti aggiuntivi."
            echo "I seguenti pacchetti sono mancanti: ${MISSING_PACKAGES[*]}"
            echo "Installazione pacchetti mancanti..."
            sudo dnf install -y "${MISSING_PACKAGES[@]}"
            echo "Pacchetti installati correttamente."
            echo ""
        fi

    # Verifica dei pacchetti su sistemi Arch
    elif command -v pacman &>/dev/null; then
        for pkg in base-devel libx11 pkg-config libxi libxtst; do
            if ! pacman -Qi "$pkg" &>/dev/null; then
                MISSING_PACKAGES+=("$pkg")
            fi
        done

        if [ ${#MISSING_PACKAGES[@]} -gt 0 ]; then
            echo "E' necessaria l'installazione di pacchetti aggiuntivi."
            echo "I seguenti pacchetti sono mancanti: ${MISSING_PACKAGES[*]}"
            echo "Installazione pacchetti mancanti..."
            sudo pacman -S --noconfirm "${MISSING_PACKAGES[@]}"
            echo "Pacchetti installati correttamente."
            echo ""
        fi

    # MacOS con Homebrew
    elif [[ "$OSTYPE" == "darwin"* ]] && command -v brew &>/dev/null; then

        if ! command -v brew &>/dev/null; then
            echo "Homebrew non è installato. Procedo con l'installazione..."
            /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
            echo "Homebrew installato correttamente."
            echo "Assicurati di aggiungere Homebrew al PATH se richiesto durante l'installazione."
        fi

        for pkg in libx11 pkg-config libxi libxtst; do
            if ! brew list "$pkg" &>/dev/null; then
                MISSING_PACKAGES+=("$pkg")
            fi
        done

        if [ ! -x "$(command -v xcode-select)" ] || ! xcode-select --print-path &>/dev/null; then
            echo "Installazione degli strumenti di sviluppo Xcode necessaria."
            xcode-select --install || echo "Xcode installato o già configurato."
        fi

        if [ ${#MISSING_PACKAGES[@]} -gt 0 ]; then
            echo "E' necessaria l'installazione di pacchetti aggiuntivi."
            echo "I seguenti pacchetti sono mancanti: ${MISSING_PACKAGES[*]}"
            echo "Installazione pacchetti mancanti..."
            brew install "${MISSING_PACKAGES[@]}"
            echo "Pacchetti installati correttamente."
            echo ""
        fi

    else
        echo "Sistema operativo non supportato o gestore di pacchetti non trovato."
    fi
}
install_components
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