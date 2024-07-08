use std::io;
use rand::{thread_rng, Rng};

fn main() {
  println!("Escolha quantos dados você quer e quantos lados tem cada dado:\n\n\
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
  for num in 1..=num_dices
  {
    println!("Informe o número de lados para o {}º dado:\n\
    ", num);

    let mut sides_dices = String::new();
    
    io::stdin()
    .read_line(&mut sides_dices)
    .expect(&format!("Erro ao pegar o número de lado do dado {}º", num));

    println!(" ");

    let sides_dice: u8 = sides_dices
    .trim()
    .parse()
    .expect(&format!("Erro ao transformas os lados do dado {}º em número!", num));

    if sides_dice < 4
    {
        vect_all_dices.push(4);
    }
    else
    {
        vect_all_dices.push(sides_dice);
    }   
  }

  println!("O resultado de cada dado foi:");
  
  for i in 0..num_dices
  {
    // Jogando um dado por vez.
    println!("O resultado do {}º dado é: {}",i + 1,match_result(vect_all_dices[i as usize]));
  }
  
}

// Fazendo as jogadas dos dados.

fn match_result(num_sides: u8) -> u8
{
  let result_move = thread_rng().gen_range(1..num_sides);

  result_move
}