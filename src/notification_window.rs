use notify_rust::Notification;

pub fn start_notify(summary: &str, body: &str) {
    // Crea e mostra la notifica
    Notification::new()
        .summary(summary)
        .body(body)
        .show()
        .unwrap();

    // Attendi 5 secondi
    //thread::sleep(Duration::from_secs(5));

    // Notifica l'utente che la notifica è sparita
    println!("La notifica è sparita."); //inserire qui il ritorno alla fase iniziale di attesa prima gesture
}