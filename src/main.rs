use std::io;
use rand::{thread_rng, Rng};

fn main() {
  println!("Esse é um jogo de dados, escolha quantos dados você quer e quantos lados tem cada dado:\n\n\
  Informe quantos dados você quer no seu jogo:\n\
  ");

  let mut num_dices = String::new();

  io::stdin()
  .read_line(&mut num_dices)
  .expect("Erro ao pegar a quantidade de dados do usuário!");

  println!("Informe quantos lados vai ter cada dado:\n\
  ");

  let num_dices:u8 = num_dices
  .trim()
  .parse()
  .expect("Erro: Informe um número!");

  println!(" ");

  let mut vect_all_dices:Vec<u8> = Vec::new();

  // Pegando os lados de cada dado.
  for num in 0..num_dices
  {
    println!("Informe o número de lados para o {}º dado:\n\
    ", num + 1);

    let mut sides_dices = String::new();
    
    io::stdin()
    .read_line(&mut sides_dices)
    .expect(&format!("Erro ao pegar o número de lado do dado {}º", num + 1));

    println!(" ");

    let sides_dices: u8 = sides_dices
    .trim()
    .parse()
    .expect(&format!("Erro ao transformas os lados do dado {}º em número!", num + 1));

    if sides_dices < 4
    {
        vect_all_dices.push(4);
    }
    else
    {
        vect_all_dices.push(sides_dices);
    }   
  }

  println!("O resultado de cada dado foi:");
  
  for i in 0..num_dices
  {
    println!("O resultado do {}º dado é: {:?}",i+1,match_result(num_dices, vect_all_dices.clone())[i as usize]);
  }
  
}

// Fazendo as jogadas dos dados.

fn match_result(dices: u8, num_sides: Vec<u8>) -> Vec<u8>
{
  let mut results: Vec<u8> = Vec::new();
    
    // Gerando um número aleatório.

    for num in 0..dices
    {
      let result_move = thread_rng().gen_range(0..num_sides[num as usize]);

      results.push(result_move);
    }

    results
}