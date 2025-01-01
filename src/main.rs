#![windows_subsystem = "windows"] //per non mostrare il terminale (windows)

use std::{env};
use auto_launch::{AutoLaunchBuilder};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::time::Duration;

mod backup_tool;
use backup_tool::{generate_backup_name, get_extensions, get_usb_path, copy_dir, get_src_path, log_cpu_usage};

mod backup_command;
use backup_command::{first_command, second_command};

mod notification_window;
use notification_window::{start_notify};

fn main(){
    log_cpu_usage();

    let exe = env::current_exe().unwrap();
    let exe_path = exe.to_string_lossy().to_string();
    let project_root = exe.parent().unwrap().parent().unwrap().parent();

    let conf_name = "conf.txt";
    let file_di_configurazione = project_root.unwrap().join(conf_name).to_string_lossy().to_string();

    #[cfg(not(target_os = "macos"))] 
    {
        let auto = AutoLaunchBuilder::new()
            .set_app_name("Group23")
            .set_app_path(&exe_path)  //Imposta il percorso dell'applicazione che deve essere avviata automaticamente
            .build()
            .unwrap();

        auto.enable().unwrap();
        println!("Autostart enabled: {}", auto.is_enabled().unwrap());
    }

    #[cfg(target_os = "macos")] 
    {
        let _ = AutoLaunchBuilder::new()
            .set_app_name("Group23")
            .set_app_path(&exe_path)
            .set_use_launch_agent(false) 
            .build()
            .unwrap().enable();

        Command::new("osascript") //per non mostrare il terminale (macOS)
            .arg("-e")
            .arg("tell application \"Terminal\" to set visible of front window to false")
            .output()
            .expect("Failed to hide terminal");
    }

    let mut origine;
    let mut usb_path;
    let mut estensioni;
    let mut destinazione;

    loop {
        println!("Traccia un rettangolo con il mouse per iniziare il backup...");
        let stop_flag = Arc::new(AtomicBool::new(false));
        let stop_flag_clone = Arc::clone(&stop_flag);
        let receiver = first_command(stop_flag_clone);
        match receiver.recv() { // .recv() blocca finché non riceve un valore
            Ok(success) => {
                if success {
                    origine = get_src_path(&file_di_configurazione);
                    usb_path = get_usb_path();

                    if origine.is_some() && usb_path.is_some() {
                        estensioni = get_extensions(&file_di_configurazione).unwrap();
                        destinazione = generate_backup_name(&origine.clone().unwrap().to_string(), &usb_path.clone().unwrap().to_string());
                    }
                    else {
                        println!("Il backup non è andato a buon fine, riavvio della procedura.");
                        stop_flag.store(true, Ordering::Relaxed);
                        continue; //ricomincia il loop
                    }
                    stop_flag.store(true, Ordering::Relaxed); //imposta il flag su `true`, farà interrompere first_command
                    println!("Flag di stop impostato!");
                    start_notify("Gesture riconosciuta", "Eseguire la seconda gesture entro 10 secondi per proseguire con il backup.");
                    let second_receiver = second_command();
                    match second_receiver.recv_timeout(Duration::from_secs(10)) {
                        Ok(success) => {
                            if success {
                                start_notify("Gesture di conferma riconosciuta", "Backup in corso...");
                                println!("Rettangolo rilevato! Inizio backup...");
                                let result = copy_dir(&origine.unwrap().to_string(), &destinazione, estensioni);
                                if result.unwrap()==-1 {
                                    continue; //valore speciale "-1" per riavviare la procedura (se avviene un errore tra una gesture e l'altra)
                                }
                                break; //esce dal loop dopo aver terminato il backup, il programma termina.
                            }
                        }
                        Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                            start_notify("Tempo scaduto", "Tempo scaduto per il secondo comando. Riavvio della procedura.");
                            continue; // ricomincia il loop.
                        }
                        Err(e) => {
                            println!("Errore nella comunicazione con il thread (2° comando): {}", e);
                        }
                    }
                } else {
                    println!("Operazione fallita.");
                }
            }
            Err(e) => {
                println!("Errore nella comunicazione con il thread: {}", e);
            }
        }
    }
}
