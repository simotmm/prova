@echo off

echo -- TOOL DI BACKUP: Disattivazione --
echo:

rem terminazione dei processi
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

rem eliminazione degli eseguibili
if exist target\release\progetto_rust.exe (
    del /f /q target\release\progetto_rust.exe
)
if exist target\debug\progetto_rust.exe (
    del /f /q target\debug\progetto_rust.exe
)

rem messaggio finale e chiusura
echo Il tool non e' piu' in esecuzione.
echo Il programma non sara' piu' avviato in backgound all'avvio del pc.
echo:
echo Chiusura della finestra in 30 secondi.
timeout /t 30 >nul
exit
