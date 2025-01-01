use std::thread;
use device_query::{DeviceQuery, DeviceState}; //device_query: libreria per chiedere dello stato di mouse e tastiera SENZA bisogno di finestra attiva. funziona per Windows, Mac, Linux
use rdev::display_size;            //rdev: libreria per sentire/inviare eventi a tastiera/mouse su Windows, Mac, Linux
use std::sync::mpsc::{self};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

pub fn first_command(stop_flag: Arc<AtomicBool>) -> mpsc::Receiver<bool> {
    let (sender, receiver) = mpsc::channel();

    let device_state = DeviceState::new();
    let mut sides: Vec<char> = Vec::with_capacity(4); //vettore di char perchè devo verificare di avere caratteri (rappresentanti i lati) sempre alternati
                                                              //con un semplice contatore non avrei questo controllo
    let mut direction= false; //per verificare se il rettangolo è disegnato in senso orario o antiorario
    let mut next: (String, String) = (String::new(), String::new());

    thread::spawn(move || {
        let (width, height) = display_size().unwrap(); //dimensione in pixel dello schermo principale

        let mut is_drawing = false;
        let mut start: (i32, i32) = (0, 0);
        let mut end: (i32, i32);

        loop {
            if stop_flag.load(Ordering::Relaxed){
                println!("interruzione del thread");
                break;
            }

            let mouse = device_state.get_mouse(); //per ottenere le coordinate del mouse e lo stato dei bottoni del mouse
            let coordinates = mouse.coords;
            if mouse.button_pressed[1]{     //pulsante premuto
                if !is_drawing{              //se sta disegnando
                    is_drawing = true;
                    start = coordinates;
                }
            } else{             //pulsante non premuto
                if is_drawing{   //ma lo stato dice che sta disegnando
                    is_drawing = false; //aggiorno lo stato al valore corretto
                    end = coordinates;
                    
                    if sides.len() == 0 {   //inserimento primo segmento
                        if vertical_check(start, end, height, width){
                            sides.push('V');
                            println!("{}", sides.len());
                            direction = is_clockwise(start, end, 'V', width as i32); //senso orario o antiorario
                        }else{
                            if horizontal_check(start, end, width, height){
                                sides.push('H');
                                direction = is_clockwise(start, end, 'H', height as i32); //senso orario o antiorario
                                println!("{}", sides.len());
                            }
                        }
                        next = next_side(end, direction);
                        println!("next: {:?}", next);
                    } else{ //ora vanno fatti i casi in cui un segmento è già stato inserito
                        if sides.len() < 4 {
                            if sides[sides.len() - 1] == 'V' && horizontal_check(start, end, width, height) {
                                if is_correct(start, end, 'H', &next){
                                    sides.push('H');
                                    next = next_side(end, direction);
                                    println!("{}", sides.len());
                                    println!("next: {:?}", next);
                                } else {
                                    sides.clear();
                                    println!("{}", sides.len());
                                }
                            } else{
                                if sides[sides.len() - 1] == 'H' && vertical_check(start, end, height, width){
                                    if is_correct(start, end, 'V', &next){
                                        sides.push('V');
                                        next = next_side(end, direction);
                                        println!("{}", sides.len());
                                        println!("next: {:?}", next);
                                    } else {
                                        sides.clear();
                                        println!("{}", sides.len());
                                    }
                                } else{
                                    sides.clear(); //se ho V e V oppure H e H oppure un segmento non valido, resetto tutto
                                    println!("{}", sides.len());
                                }
                            }
                            if sides.len() == 4 { //rettangolo fatto
                                println!("ho TEORICAMENTE finito");
                                sides.clear();
                                sender.send(true).unwrap(); //NOTA: non c'è break, in modo da poter ricominciare in caso di errore
                                                              //il thread terminerà grazie a un avviso da parte del main
                            }
                        }
                    } 
                }
            }
        }
    });
    receiver
}

pub fn second_command() -> mpsc::Receiver<bool> {
    let (sender, receiver) = mpsc::channel();
    let device_state = DeviceState::new();
    thread::spawn(move || {
        let (width, _height) = display_size().unwrap();
        let w = width as f64;

        let mut is_drawing = false;
        let mut start: (i32, i32) = (0, 0);
        let mut end: (i32, i32);
        loop {
            let mouse = device_state.get_mouse();
            let coordinates = mouse.coords;
            if mouse.button_pressed[1]{
                if !is_drawing{
                    is_drawing = true;
                    start = coordinates;
                }
            }else{
                if is_drawing{
                    is_drawing = false;
                    end = coordinates;
                    if horizontal_check2(start, end, w){
                        if sender.send(true).is_err(){
                            println!("Second command disconnesso");
                            break;
                        }
                    }
                }
            }
        }
    });
    receiver
}

/*
verifico che i punti start ed end siano grossomodo allineati verticalmente,
che il segmento tracciato sia almeno il 90% dell'altezza dello schermo
e che il segmento sia tracciato lungo il bordo destro o sinistro dello schermo
*/
fn vertical_check(start: (i32, i32), end: (i32, i32), height: u64, width:u64) -> bool {
    //println!("sono in VERTICAL CHECK: end.0: {}", end.0);
    let tolerance = 50;
    let check = (start.0 >=  end.0 - tolerance && start.0 <= end.0 + tolerance) && (end.1 - start.1).abs() as f64 > 0.9 * height as f64;
    //println!("{}", check && ((end.0 - tolerance <= 0) || (end.0 + tolerance >= width as i32)));
    check && ((end.0 - tolerance <= 0) || (end.0 + tolerance >= width as i32))
}

/*
verifico che i punti start ed end siano grossomodo allineati orizzontalmente,
che il segmento tracciato sia almeno il 90% della lunghezza dello schermo
e che il segmento sia tracciato lungo il bordo superiore o inferiore dello schermo
 */
fn horizontal_check(start: (i32, i32), end: (i32, i32), width: u64, height: u64) -> bool {
    //println!("sono in HORIZONTAL CHECK: start.1: {}, end.1: {}", start.1, end.1);
    //println!("height: {}", height);
    let tolerance = 50;
    let check = (start.1 >=  end.1 - tolerance && start.1 <= end.1 + tolerance) && (start.0 - end.0).abs() as f64 > 0.90 * width as f64;
    //println!("{}, {}, {}", check, end.1 - tolerance <= 0, end.1 + tolerance >= height as i32);
    //println!("{}", check && (end.1 - tolerance <= 0) || (end.1 + tolerance >= height as i32));
    check && ((end.1 - tolerance <= 0) || (end.1 + tolerance >= height as i32))
}

fn horizontal_check2(start: (i32, i32), end: (i32, i32), width: f64) -> bool {
    let tolerance = 50;
    (start.1 >=  end.1 - tolerance && start.1 <= end.1 + tolerance) && (start.0 - end.0).abs() as f64 > 0.90 * width
}

/*
calcola l'orientamento (senso orario o antiorario) del rettangolo sulla base del primo segmento
false --> antiorario
true --> orario
 */
fn is_clockwise(start: (i32, i32), end: (i32, i32), side: char, length: i32) -> bool {
    let tolerance = 50;
    let mut clocwise: bool = true;
    if side == 'V' {
        if ((start.1 < end.1) && (end.0 <= tolerance)) || ((start.1 > end.1) && (end.0 + tolerance >= length)){
            clocwise = false;
        }
        else if ((start.1 > end.1) && (end.0 <= tolerance)) || ((start.1 < end.1) && (end.0 + tolerance >= length)){
            clocwise = true;
        }
    } else if side == 'H' {
        if (start.0 > end.0 && end.1 <= tolerance) || (start.0 < end.0 && end.1 + tolerance >= length){ //alto <-- OPPURE basso -->
            clocwise = false;
        }
        else if ((start.0 < end.0) && (end.1 <= tolerance)) || ((start.0 > end.0) && (end.1 + tolerance >= length)){ //alto --> OPPURE basso <--
            clocwise = true;
        }
    }
    println!("clocwise: {}", clocwise);
    clocwise
}

/*
calcola quale DEVE essere il prossimo segmento da tracciare, in modo che venga riconosciuta
la giusta sequenza di segmenti durante tutto il tracciamento del primo comando
 */
fn next_side(end: (i32, i32), direction: bool) -> (String, String) {
    let tolerance= 50;
    if end.1 >= tolerance && end.0 <= tolerance { //angolo in basso a sx
        if direction {
            ("sinistra".to_string(), "basso".to_string())
        } else {
            ("basso".to_string(), "sx".to_string())
        }
    } else if end.1 >= tolerance && end.0 > tolerance { //angolo in basso a dx
        if direction {
            ("basso".to_string(), "dx".to_string())
        } else {
            ("destra".to_string(), "basso".to_string())
        }
    } else if end.1 < tolerance && end.0 <= tolerance { //angolo in alto a sx
        if direction {
            ("alto".to_string(), "sx".to_string())
        } else {
            ("sinistra".to_string(), "alto".to_string())
        }
    } else { //angolo in alto a dx
        if direction {
            ("destra".to_string(), "alto".to_string())
        } else {
            ("alto".to_string(), "dx".to_string())
        }
    }
}

/*
verifica che il segmento effettivamente tracciato col mouse corrisponda al segmento calcolato con next_side
 */
fn is_correct(start: (i32, i32), end: (i32, i32), side: char, next: &(String, String)) -> bool {
    let tolerance = 50;
    let mut tmp = (String::new(), String::new());
    println!("start: ({}, {}), end: ({}, {})", start.0, start.1, end.0, end.1);
    println!("side: '{}'", side);
    println!("next: {}, {}", next.0, next.1);
    if side == 'V'{
        if end.0 <= tolerance {
            tmp.0 = "sinistra".to_string();
        } else {
            tmp.0 = "destra".to_string();
        }

        if start.1 < end.1 {
            tmp.1 = "alto".to_string();
        } else {
            tmp.1 = "basso".to_string();
        }
    } else if side == 'H'{
        if end.1 <= tolerance {
            tmp.0 = "alto".to_string();
        } else {
            tmp.0 = "basso".to_string();
        }

        if start.0 < end.0 {
            tmp.1 = "sx".to_string();
        } else {
            tmp.1 = "dx".to_string();
        }
    }
    next.0 == tmp.0 && next.1 == tmp.1
}