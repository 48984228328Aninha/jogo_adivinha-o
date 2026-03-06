<div align="center">

## Descrição:
Um jogo simples de adivinhação de números desenvolvido em Rust, onde o jogador tenta adivinhar um número secreto entre 1 e 100.

</div>

---
<!--Start Experiences-->
<h1 align="center">
  <img src="https://i.imgur.com/dwyUWDH.gif" width="30"/>
  Features
</h1>

- **Geração Aleatória**: Número secreto gerado aleatoriamente entre 1 e 100
- **Feedback Interativo**: Dicas se o número é maior ou menor
- **Validação de Entrada**: Tratamento de erros para entradas inválidas
- **Loop Contínuo**: Aposte até acertar o número

---

<!--Start Stack-->     
<h1 align="center"><img src="https://i.imgur.com/eu3StDB.gif" width="28"/> 𝗧𝗲𝗰𝗵 𝗦𝘁𝗮𝗰𝗸</h1> 
<p align="center">
  <img src="https://skillicons.dev/icons?i=rust&size=72&perline=8" />
</p>

---

<h1 align="center"><img src="https://i.imgur.com/4ohPnKW.gif" width="28"/> Requisitos</h1> 

### - [Rust instalado](https://rustup.rs/) / ou na sua IDE

<h1 align="center"><img src="https://i.imgur.com/m41XwMi.gif" width="28"/> Testando</h1> 

```bash
cd jogo_adivinhacao
cargo run
```

## Como Jogar:

1. Execute o jogo com `cargo run`
2. Digite um número entre 1 e 100
3. Receba feedback se o número é maior ou menor
4. Continue tentando até acertar!
5. Quando acertar, você vence!

## Exemplo de Uso:

```
Digite um número
The secret number is 42
50
Você digitou 50
Muito grande
30
Você digitou 30
Muito pequeno
42
Você digitou 42
You win
```

## Dependências no Código:

- `rand`: Geração de números aleatórios (óbvio)
- `std::io`: Entrada/saída padrão (in and out)
- `std::cmp`: Comparação de valores (cmp)
