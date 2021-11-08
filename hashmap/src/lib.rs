use std::net::TcpStream;
use std::io::Write;
use std::io::Read;
use std::net::TcpListener;
use regex::Regex;

pub struct Hashmap {
    node_ip: String,
}

pub struct Response {
    pub value: String,
    pub node_id: i32,
}

pub fn inicia(node_id:i32) -> Hashmap {
    Hashmap{node_ip: format!("127.0.0.1:{}", node_id+7000)}
}

pub fn insere(hashmap: Hashmap, id:i32, key: &String, value: &String) -> Result<(), &'static str> {
    let mut stream = match TcpStream::connect(hashmap.node_ip.as_str()) {
        Ok(s) => s,
        Err(_) => return Result::Err("Erro ao conectar com o nó servidor"),
    };

    let message = format!("0;{};i;{};{}", id, key, value);
    match stream.write(message.as_bytes()) {
        Ok(n) => {
            if n == message.len() {
                return Result::Ok(());
            }
            else{
                return Result::Err("Erro ao enviar requisição ao nó servido");
            }
        },
        Err(_) => return Result::Err("Erro ao enviar requisição ao nó servido"),
    }
}

pub fn consulta(hashmap: Hashmap, id: i32, key: &String) -> Result<Response, &'static str> {
    let mut stream = match TcpStream::connect(hashmap.node_ip.as_str()) {
        Ok(s) => s,
        Err(_) => return Result::Err("Erro ao conectar com o nó servidor"),
    };
    
    let listener = match TcpListener::bind(&format!("127.0.0.1:{}", 6000+id).as_str()) {
        Ok(l) => l,
        Err(_) => return Result::Err(&"Erro ao escutar conexões"),
    };

    let message = format!("127.0.0.1:{};{};c;{};0", 6000+id, id, key);
    match stream.write(message.as_bytes()) {
        Ok(_) => (),
        Err(_) => return Result::Err("Erro ao enviar requisição ao nó servido"),
    }
    
    let regex = Regex::new(r"^(.+);([0-9]+)$").unwrap();
    for received in listener.incoming() {
        let mut received = received.unwrap();
        let mut buffer = [0;1024];
        let lidos = received.read(&mut buffer).unwrap();
        let message = String::from_utf8_lossy(&buffer[0..lidos]);
        let cap = match regex.captures(&message) {
            Some(x) => x,
            None => continue,
        };
        let value = cap.get(1).unwrap().as_str().to_string();
        let id:i32 = cap.get(2).unwrap().as_str().parse().unwrap();
        return Result::Ok(Response{value: value, node_id: id});
    }
    Result::Err("Não foi recebida conexão do nó servidor de destino")
}
