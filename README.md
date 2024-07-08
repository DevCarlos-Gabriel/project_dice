# Projeto de Rolagem de Dados 🎲

Este projeto é um programa simples em Rust que permite ao usuário rolar um conjunto de dados com um número personalizado de lados. O usuário pode especificar quantos dados deseja rolar e quantos lados cada dado deve ter. Se o número de lados de um dado for menor que 4, o programa automaticamente define esse dado como tendo 4 lados.

## Funcionalidades ✨

- Permite ao usuário especificar quantos dados deseja rolar.
- Permite ao usuário especificar quantos lados cada dado deve ter.
- Garante que todos os dados tenham pelo menos 4 lados.
- Exibe o resultado de cada rolagem de dado.

## Como Usar 🚀

1. **Clone o repositório:**
    ```sh
    git clone https://github.com/DevCarlos-Gabriel/project_dice.git project_dice
    cd project_dice
    ```

2. **Compile o programa:**
    ```sh
    cargo build
    ```

3. **Execute o programa:**
    ```sh
    cargo run
    ```

## Exemplo de Uso 📋

Ao executar o programa, você verá as seguintes mensagens no terminal:

```sh
Escolha quantos dados você quer e quantos lados tem cada dado:

Informe quantos dados você quer no seu jogo:

3

Informe quantos lados vai ter cada dado:

Informe o número de lados para o 1º dado:

6

Informe o número de lados para o 2º dado:

8

Informe o número de lados para o 3º dado:

10

O resultado de cada dado foi:
O resultado do 1º dado é: 3
O resultado do 2º dado é: 5
O resultado do 3º dado é: 7
```

## Estrutura do Código 📂

### Função Principal

A função `main` é responsável por interagir com o usuário para obter a quantidade de dados e o número de lados de cada dado. Em seguida, ela gera e exibe os resultados das rolagens.

### Função de Rolagem de Dados

A função `match_result` recebe o número de lados de um dado e retorna um número aleatório entre 1 e o número de lados especificado.

## Considerações Finais 📝

Este projeto é uma introdução simples à programação em Rust, ao uso de vetores, entrada e saída padrão e geração de números aleatórios. Sinta-se à vontade para modificar e expandir o projeto conforme sua necessidade.

## Licença 📄

Este projeto está licenciado sob a Licença MIT - veja o arquivo [LICENSE](https://github.com/DevCarlos-Gabriel/project_dice/blob/main/LICENSE) para mais detalhes.
