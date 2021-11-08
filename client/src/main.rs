use std::env;

use hashmap;

fn main () {
    let args: Vec<String> = env::args().collect();
    if args.len() < 5 || args.len() > 6 {
        println!("uso: {} <id_consulta> <id_nó> <operação> <chave> [<valor>]", args[0]);
        println!("  -id_consulta: id da consulta");
        println!("  -id_nó: id do nó servidor que receberá a requisição");
        println!("  -operação: 'c' ou 'i' para consulta ou inserção");
        println!("  -chave: chave para busca ou inserção");
        println!("  -valor: valor para inserção");
        return ();
    }

    let consulta_id:i32= args[1].parse().unwrap();
    let node_id:i32= args[2].parse().unwrap();
    let op:char= args[3].parse().unwrap();
    let chave: String= args[4].parse().unwrap();
    let valor: String;

    let hm = hashmap::inicia(node_id);

    if op == 'i' {
        if args.len() != 6 {
            println!("Valor não especificado");
            return ();
        }
        valor = args[5].parse().unwrap();
        match hashmap::insere(hm, consulta_id, &chave, &valor) {
            Ok(_) => (),
            Err(e) => panic!("{}", e),
        }
    }
    else if op == 'c' {
        match hashmap::consulta(hm, consulta_id, &chave) {
            Ok(response) => {
                println!("Cliente - Consulta {} enviada a nó {}: valor para \"{}\" armazenado no nó {}: \"{}\"",
                    consulta_id, node_id, chave, response.node_id, response.value); 
            },
            Err(e) => panic!("{}", e),
        }
    }
    else {
        println!("Operação desconhecida");
    }
}
