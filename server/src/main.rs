use std::env;
use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::io::Read;
use std::io::Write;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::net::TcpStream;
use std::net::TcpListener;
use regex::Regex;

enum HashmapMessage {
    Insert {key: String, value: String},
    Query {key: String, tx: mpsc::Sender<String>},
}

fn main() {
    //obtém argumentos
    let (n, myid) = {
        let args: Vec<String> = env::args().collect();
        if args.len() != 3 {
            println!("uso: {} <n> <i>", args[0]);
            println!("  -n: log2(número de nós)");
            println!("  -i: número deste nó");
            return ();
        }
        let n: i32 = args[1].parse().unwrap();
        let i: i32 = args[2].parse().unwrap();
        (n,i)
    };
    let firstport:i32 = 7000;
    
    //cria e inicializa finger table
    //mapa que associa id dos nós vizinhos aos seus "ip:porta"
    let finger_table: BTreeMap<i32, String> = (0..n).map(|i| {
        let id = (myid + (1<<i))%(1<<n);
        let address = format!("127.0.0.1:{}",firstport +id);
        (id, address)
    }).collect();
    let finger_table = Arc::new(finger_table);
    
    //inicia thread de gerência do HashMap
    let (hashmap_message_tx, hashmap_message_rx) = mpsc::channel(); 
    thread::spawn( move || {
        let mut hashmap: HashMap<String, String> =  HashMap::new();
        let mut waiting_queue: HashMap<String, Vec<mpsc::Sender<String>>> = HashMap::new();
        for message in hashmap_message_rx {
            match message {
                HashmapMessage::Insert{key,value} => {
                    if let Some(to_answer) = waiting_queue.remove(&key) {
                        for tx in to_answer { tx.send(value.clone()).unwrap() };
                    }
                    hashmap.insert(key, value);
                },
                HashmapMessage::Query{key,tx} => {
                    match hashmap.get(&key) {
                        Some(value) => tx.send(value.clone()).unwrap(),
                        None => waiting_queue.entry(key).or_default().push(tx)
                    }
                }
            }
        }
    });
   
    

    { // inicia loop de recebimento de conexões e criação de threads atendentes 
        let myip = format!("127.0.0.1:{}",myid+firstport);
        let listener = TcpListener::bind(&myip).expect("Bind error");
        println!("*Nó {} - Esperando conexões em {}", myid, myip);
        let regex = Regex::new(r"^(.+);([0-9]+);(i|c);(.+);(.*)$").unwrap();
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            let regex = regex.clone();
            let hashmap_message_tx = hashmap_message_tx.clone();
            let finger_table = finger_table.clone();

            //lança thread atendente
            thread::spawn(move || {
                //obtém requisição do cliente
                let mut buffer = [0; 1024];
                let lidos = stream.read(&mut buffer).unwrap();
                let request = String::from_utf8_lossy(&buffer[0..lidos]);
                
                //parse da requisição
                let cap = match regex.captures(&request) {
                    Some(x) => x,
                    None => panic!("Não foi possível fazer parse da requisição")
                };
                let response_ip = cap.get(1).unwrap().as_str();
                let operation_id= cap.get(2).unwrap().as_str();
                let operation   = cap.get(3).unwrap().as_str();
                let key         = cap.get(4).unwrap().as_str();
                let value       = cap.get(5).unwrap().as_str();
                println!("*Nó {} - Requisição {} recebida response_ip: {}, operation: {}, key: {}, value: {}",
                    myid, operation_id, response_ip, operation, key, value);
                
                let hash = {
                    let mut sum: i32 = 0;
                    for byte in key.bytes() {
                        let byte: i32 = byte.into();
                        sum = (sum + byte) % (1<<n);
                    }
                    sum
                };
                
                if hash == myid {   // pede o valor para a thread gerente do hashmap
                    if operation.eq("i") { //operação de inserção
                        hashmap_message_tx.send(
                            HashmapMessage::Insert{key: key.to_string(), value: value.to_string()}
                        ).unwrap();
                        println!("*Nó {} - Inserido par ({},{})", myid, key, value);
                    }
                    else { //operação de consulta
                        let (response_tx, response_rx) = mpsc::channel(); 
                        hashmap_message_tx.send(
                            HashmapMessage::Query{key: key.to_string(), tx: response_tx}
                        ).unwrap();
                        let value = response_rx.recv().unwrap();
                        let message = format!("{};{}",value, myid);
                        let mut response_stream = TcpStream::connect(response_ip)
                            .expect("Não foi possível se conectar ao cliente para resposta");
                        response_stream.write(message.as_bytes()).unwrap();
                        println!("*Nó {} - Consulta {} respondida par ({},{})", myid, operation_id, key, value);
                    }
                }
                else {      // roteia o pedido para o nó mais proximo do destino
                    let node_qty = 1<<n;
                    let distance = (((hash-myid)%node_qty)+node_qty)%node_qty; //(hash-myid) mod 2^n
                    let closest_node_id = {
                        let closest_node_distance = (0..n).rev().map(|x| 1<<x).find(|x| x<=&distance).unwrap();
                        (closest_node_distance + myid) % node_qty 
                    };
                    let closest_node_ip = finger_table.get(&closest_node_id).unwrap();
                    let mut retransmission_stream = TcpStream::connect(closest_node_ip)
                            .expect("Não foi possível se conectar ao nó vizinho");
                    retransmission_stream.write(request[0..lidos].as_bytes()).unwrap();
                }

            });
        }
    }


}
