Por o projeto para rodar:
1. Baixar [rustup](https://forge.rust-lang.org/infra/other-installation-methods.html)
2. Abrir o prompt de comando como administrador
3. Navegar até o diretório do projeto
4. Executar o comando `cargo run`

Manuseando o projeto:
* Utilizar `PageDown` para descer uma página.
* Utilizar `PageUp` para subir uma página.
* Apertando o botão `CARREGAR`, abrirá uma caixa de texto em que pode ser informado o disco a ser analisar (C:, D:, etc)
* Apertando o botão `NAVEGAR`, abrirá uma caixa de texto em que pode ser informado a posição no disco que desejasse navegar
  * Atenção: a posição informada deve estar no formato `hexadecimal` e ser `multiplo de 512`
* Apertando o botão `BUSCAR`, abrirá uma caixa de texto em que pode ser informado um termo a ser buscado no disco
  * Está busca retornará a `página` em que se encontra o termo informado.