
Experimento número 1 usando prompt inicial: "Crie um clone de Pong usando a linguagem de programação Rust com o biblioteca kiss3d versão 0.26. O jogo deve consistir em dois 'paddles' que podem ser movimentados verticalmente via teclado e uma bola que se movimenta na diagonal refletindo nas bordas superiores ou ao colidir com um dos 'paddles'. Quando a bola ultrapassa uma das bordas laterais a bola é reposicionada no meio da tela.".

## Resultados

    - Iteração 1: varios erros de compilação por uso de argumentos inválidos e métodos inexistentes.
    - Iteração 2: continuou apresentando os mesmos erros de compilação ou similares.
    - Iteração 3: não ofereceu um novo código só pediu para trocar a versão do 'nalgebra' para uma compatível. tomei a liberdade de remover o dependência do nalgebra e usar o reexportação do próprio kiss3d, isse resoveu os problemas de argumentos incopatíveis mas manteve os erros de métodos inexistentes.
