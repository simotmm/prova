@echo off

echo -- TOOL DI BACKUP: Attivazione --
echo:

rem chiusura di altre eventuali istanze del programma per evitare conflitti
echo Verifica processi di backup attivi...
tasklist | findstr /i "progetto_rust.exe" >nul
if %errorlevel% equ 0 (
    echo "progetto_rust.exe" e' in esecuzione. Terminazione del processo...
    taskkill /im progetto_rust.exe /f >nul
    if %errorlevel% neq 0 (
        echo Impossibile terminare il processo. Assicurati di avere i permessi necessari.
        timeout /t 6 >nul
        exit /b 1
    )
    echo Processi terminati con successo.
) else (
    echo Nessun processo attivo trovato.
)
echo:

rem compilazione
echo Compilazione del tool di backup...
cargo build --release
if %errorlevel% neq 0 (
    echo Errore durante l'installazione del tool di backup.
    timeout /t 6 >nul
    exit /b 1
)
echo Tool di backup compilato correttamente.
echo:

rem avvio in background
echo Attivazione del tool di backup...
start "" target\release\progetto_rust.exe
echo Tool di backup attivato correttamente.
echo: 

@echo off

rem lettura del file di configurazione
if not exist conf.txt (
    echo Il file "conf.txt" non esiste. Assicurati che sia presente nella directory corrente.
    exit /b 1
)
setlocal enabledelayedexpansion
set /p Sorgente=<conf.txt
for /f "skip=1 delims=" %%A in (conf.txt) do (
    set Estensioni=%%A
    goto :print
)

:print
echo Sorgente di backup: !Sorgente!.
if "!Estensioni!"=="" (
    echo Estensioni dei file da copiare: tutti i file.
) else (
    echo Estensioni dei file da copiare: !Estensioni!.
)
echo:
endlocal
echo Per cambiare sorgente ed estensioni modificare il file 'conf.txt'.
echo Modalita' di backup:
echo  1. Per impostare il backup per tutti i tipi di file lasciare vuota
echo     la seconda riga di 'conf.txt'.
echo  2. Per copiare solo determinati tipi di file scrivere le estensioni 
echo     desiderate nella seconda riga di 'conf.txt'.
echo:

rem messaggio finale e chiusura
echo Il tool e' attualmente in esecuzione.
echo Dal prossimo avvio del pc il programma sara' avviato in backgound.
echo:
echo Chiusura della finestra in 50 secondi.
timeout /t 50 >nul
exit
