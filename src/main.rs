use reqwest::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct JsonBody {
    aviso: String,
    data: String,
    liturgia: String,
    cor: String,
    dia: String,
    oferendas: String,
    comunhao: String,
    primeiraLeitura: Leitura,
    segundaLeitura: Leitura,
    salmo: Leitura,
    evangelho: Leitura
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
enum Leitura {
    Leitura(DadosLeitura),
    Salmos(Salmo),
    Vazio(String),
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
struct DadosLeitura {
    referencia: String,
    titulo: String,
    texto: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
struct Salmo {
    referencia: String,
    refrao: String,
    texto: String
}

impl JsonBody {
    async fn request() ->  Result<JsonBody> {
        let url = "https://liturgiadiaria.site/";
        let client = reqwest::Client::new();
        let response: JsonBody = client
            .get(url)
            .header("content-type", "application/json")
            .header("Accept", "application/json")
            .send()
            .await?
            .json()
            .await?; 
        Ok(response)
    }
}

fn print_liturgia_diaria(liturgia_diaria: JsonBody) {
    println!("Data: {}", liturgia_diaria.data);
    println!("Liturgia: {}", liturgia_diaria.liturgia);
    println!("Cor: {}", liturgia_diaria.cor);
    print_leitura("Primeira Leitura", &liturgia_diaria.primeiraLeitura);
    print_leitura("Salmo", &liturgia_diaria.salmo);
    print_leitura("Segunda Leitura", &liturgia_diaria.segundaLeitura);
    print_leitura("Evangelho", &liturgia_diaria.evangelho);
}

fn print_leitura(titulo: &str, leitura: &Leitura) {
    match leitura {
        Leitura::Leitura(dados) => {
            println!("{}", titulo);
            println!("Referência: {}", dados.referencia);
            println!("Título: {}", dados.titulo);
            println!("Texto: {}", dados.texto);
        }
        Leitura::Salmos(dados) => {
            println!("{}", titulo);
            println!("Referência: {}", dados.referencia);
            println!("Refão: {}", dados.refrao);
            println!("Texto: {}", dados.texto);
        }
        Leitura::Vazio(_texto) => {}
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    match JsonBody::request().await {
        Ok(liturgia_diaria) => print_liturgia_diaria(liturgia_diaria),
        Err(e) => eprintln!("Erro ao realizar a requisição: {}", e),
    }
    Ok(())
}
