use std::net::{TcpListener, TcpStream};
use std::io::Read;
use std::io::Write;
use std::process;

fn tratacon (mut s: TcpStream) {

  let mut buffer = [0; 128];
  let res = s.read(&mut buffer);
  let lidos = match res {
    Ok(num) => num,
    Err(_) => {println!("erro"); process::exit(0x01)},
  };

  println!("recebi {} bytes", lidos);

  let st = String::from_utf8_lossy(&buffer);

  println!("recebeu: {}", st);

  let res = s.write(&buffer[0..lidos]);
  match res {
    Ok(num) => println!("escreveu {}", num),
    Err(_) => {println!("erro"); process::exit(0x01)},
  }

}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println! ("vai esperar conexoes!");
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("nova conexão!");
        tratacon(stream);

    }
}
