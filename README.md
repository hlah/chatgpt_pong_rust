
Experimento de geração de código usando ChatGPT.

# Metodologia

Serão utilizadas várias formas de fazer um prompt no ChatGPT afim de ser criado um jogo de pong usando a linguagem de programação Rust:

    - Sendo bem aberto na solicitação apenas exegindo a criação de um jogo "pong" e que seja em Rust.
    - Específicando uma biblioteca a ser usada.
    - Específicando a bilioteca e alguns critérios mais específicos em como o jogo deve ser.

Após a geração inicial do código será tentado rodá-lo.
Se houver algum erro de compilação ele será apresentado para de volta para o ChatGPT e solicitado que seja corrigido.
Se o código compilar sem erros o jogo será rodado e se tiver algum problema será informado o ChatGPT para corrigí-lo.

Esse processo se repetirá até que:

    - O jogo não apresente mais nenhum problema.
    - 3 iterações consecutivas em que o programa não compila.
    - Atingido o número máximo de 10 iterações

Cada variação será tentada pelo menos 2 vezes.
