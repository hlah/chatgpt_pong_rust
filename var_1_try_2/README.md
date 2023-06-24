
Experimento número 2 usando prompt inicial aberto: "Crie um clone de Pong usando a linguagem de programação Rust."

## Resultados

    - Iteração 1: criado código usando bibiloteco **ggez** versão 0.5. Apresentado erros de compilação por não implementação de __trait__.
    - Iteração 2: sem erros de compilação. 
        - Program roda gerando janela com dois "paddles", que podem ser movidas com teclas UP e DOWN ou W e S. A bola se movimenta e reflete nos limites superiores e inferiores da janela como também nos paddles.
        - Problemas encontrados, em ordem de gravidade:
            - Programa fecha ao colocar o mouse sobre janela.
            - Paddles não são limitados pelas paredes inferiores o superiores da janela.
            - Ao colidir "padddles" pela suas laterais o bola não reflete mas fica travada na mesma posição x que o "paddle".
            - Nada acontece quando a bola sai por uma das laterais, deveria ao menos ser resetada.
            - Não há uma forma de sair do jogo.
    - Iteração 3: solicitado para corrigir o primeiro erro da lista acima, não corrigiu o erro, apenas modificou para um dos "paddles" ser controlado pelo mouse.
    - Iteração 4: solicitei novamente para corrigir o erro, apenas fez uma alteração irrelevante no código.
    - Iteração 5: reenfatizei o problema que está ocorrendo e madei o erro que ocorre junto com o backtrace. Rescreveu várias partes do código, mas está ocorrendo erro de compilação por usar um módulo não declarado.

