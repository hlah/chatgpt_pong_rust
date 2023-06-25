
# Experimento de geração de código usando ChatGPT.

## Introdução

Os recentes avanços nos modelos de linguagem como o GPT e suas aplicações como ChatGPT nos deixou mais proximos da geração de programas a partir de uma descricão em texto livre.
Porém ainda a grandes limitações na capacidade de geração de codigo.
Uma delas é que o codigo gerado não é inteiramente confiavel. 

Mesmo que a sintaxe do codigo gerado esteja correta é comum a geração de codigo semanticamente incorreto.
Em particular isso costuma acontecer no uso de bibilotecas ocorerndo erros como chamada de métodos inexistentes e uso incorreto de parâmetros.

A proposta desse estudo é reproduzir esse problemas e tentar entender as suas causas como também possiveis soluções.

## Objetivos

O objetivo e analisar a criação de codigo usando o ChatGPT. 

Em especial analizar os casos em que o modelo "alucina" métodos ou parâmetros que não existem.
Também será avaliado o quão próximo o código é da intenção original.

## Metodologia

Serão utilizadas várias formas de fazer um prompt no ChatGPT afim de ser criado um jogo de pong usando a linguagem de programação Rust:

- Sendo bem aberto na solicitação apenas exegindo a criação de um jogo "pong" e que seja em Rust.
- Específicando uma biblioteca a ser usada.
- Específicando a bilioteca e alguns critérios mais específicos em como o jogo deve ser.

Após a geração inicial do código será tentado rodá-lo.
Se houver algum erro de compilação ele será apresentado de volta para o ChatGPT e solicitado que seja corrigido.
Se o código compilar sem erros o jogo será rodado e se tiver algum problema será informado o ChatGPT para corrigí-lo.
Será apenas aplicado as correções sugeridas pelo chat sem ajuda humana, também não será fornecida nenhuma sugestão de como resolver os problemas.

Esse processo se repetirá até que:

- O jogo não apresente mais nenhum problema.
- 3 iterações consecutivas em que o programa não compila.
- Atingido o número máximo de 10 iterações

Cada variação será tentada pelo menos 2 vezes.

Foi escolhida a linguagem de programação Rust por ser uma linguagem em emergência que certamente foi vista pelo conjunto de treinamento, mas que seu ecosistema de bibilotecas ainda está em desenvolvimentos o que deve evidenciar alguma dificuldade do modelo no uso correto delas.

Foi escolhido a geração de um jogo pong pois deve exigir pelo menos o uso de alguma bibiloteca grafica. 
A criação de um jogo pong também não é um problema muito simples nem muito complexo devendo haver vairos exemplos de implementação no conjunto de treinamento do ChatGPT. 
A ideia e isolar para que os problemas gerado sejam sobre o uso especifico de alguma bibiloteca e não sobre resolver o probelma em si.

## Resultados

Foram um total de 6 chats, 2 com cada variação citada na metodologia.
Segue um breve resumo de cada um junto com um link para a chat:

- Variação 1, prompt "Crie um clone de Pong usando a linguagem de programação Rust.":
    - Tentativa 1 ([chat link](https://chat.openai.com/share/26e7ebd4-7587-4fdb-90eb-561a201e46c5)): não conseguiu resorver os erros de compilacão.
    - Tentativa 2 ([chat link](https://chat.openai.com/share/73568dc1-9e22-4866-937a-5f685ee0c0a8)): na segunda iteração conseguiu gerar um codio valido mas com algun bugs, ao solicitar a correção de um dos bugs voltau a apresentar erros de compilação e não conseguiu resolver.
- Variação 2, prompt "Crie um clone de Pong usando a linguagem de programação Rust com o biblioteca kiss3d versão 0.26":
    - Tentativa 1 ([chat link](https://chat.openai.com/share/5df9b40b-b3ec-42a6-883c-eac81dd8376a)): não conseguiu resorver os erros de compilacão.
    - Tentativa 2 ([chat link](https://chat.openai.com/share/cd7c6e91-83f2-4ffe-aa02-f5f82ed55fc2)): não conseguiu resorver os erros de compilacão.
- Variação 3, prompt "Crie um clone de Pong usando a linguagem de programação Rust com o biblioteca kiss3d versão 0.26. O jogo deve consistir em dois 'paddles' que podem ser movimentados verticalmente via teclado e uma bola que se movimenta na diagonal refletindo nas bordas superiores ou ao colidir com um dos 'paddles'. Quando a bola ultrapassa uma das bordas laterais a bola é reposicionada no meio da tela.":
    - Tentativa 1 ([chat link](https://chat.openai.com/share/b8eb9edc-b4cd-4eb7-b7b2-56a59c3c720c)): não conseguiu resorver os erros de compilacão.
    - Tentativa 2 ([chat link](https://chat.openai.com/share/bd3e8e03-0ebd-44df-bfd1-5a082bb5983d)): não conseguiu resorver os erros de compilacão.

Ao executar os codigos gerados foram encontrados vários erros de compilação, sendo os 4 primeiros mais frequentes:

- Por uso de métodos inexistentes;
- Uso de valores ou objetos inexistentes;
- Uso de argumentos inválidos
- Uso de dependências incompatíveis (o modelo estava explicitamente importando uma versão de uma dependência da bibiloteca gráfica usada)
- Não implementação de trait necessário;
- Auséncia do parâmetro de tipo.

Seguem alguns exemplos de erros que ocorreram.

Uso de método inexistente:

```
error[E0599]: no method named `draw_rectangle` found for struct `Window` in the current scope
  --> src/main.rs:83:21
   |
83 |         self.window.draw_rectangle(
   |                     ^^^^^^^^^^^^^^ help: there is a method with a similar name: `add_rectangle`
```

Uso de valor inexistente:

```
error[E0599]: no variant or associated item named `Hold` found for enum `Action` in the current scope
  --> src/main.rs:38:46
   |
38 |             WindowEvent::Key(Key::W, Action::Hold, _) => self.paddle1_pos -= 5.0,
   |                                              ^^^^ variant or associated item not found in `Action`
```

Argumentos invalidos:

```
error[E0308]: mismatched types
    --> src/main.rs:37:36
     |
37   |             .set_local_translation(Point3::new(self.position.x, self.position.y, 0.0));
     |              --------------------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Translation`, found struct `na::Point`
     |              |
     |              arguments to this function are incorrect
     |
     = note: expected struct `Translation<f32, U3>`
                found struct `na::Point<f32, U3>`
note: associated function defined here
    --> /home/mateus/.cargo/registry/src/github.com-1ecc6299db9ec823/kiss3d-0.26.0/src/scene/scene_node.rs:1118:12
     |
1118 |     pub fn set_local_translation(&mut self, t: Translation3<f32>) {
     |            ^^^^^^^^^^^^^^^^^^^^^
```

Nas duas tentantivas em que se deixou aberta a bibiloteca a ser usada foi o utilizada a bibiloteca **ggez**. 
Nos casos em que foi informado qual bibiloteca usar foi solicitado o uso da bibiloteca **kiss3d** verão 0.26, foi escolhida essa por ser uma veversão disponíves ainda antes da data de corte dos dados do ChatGPT (lançada ~1 ano antes).

Apenas em uma das tentativas com prompt mais aberto (com uso de **ggez**) foi gerado um código sem erro de compilações.
Foi gerado um jogo bem similar com o pong mas com algums bugs ou coisas faltantes.
Foi então solicitada a correção de um dos bugs (o jogo quebrava quando o ponteiro passava sobre a janela). O modelo não conseguiu resolver o problema e acabou gerando erros de compilação que não foi capaz de resolver.

As hipóteses para os problemas observados são os seguintes:

- Baixa exposição do modelo as bibilotecas usadas: ambas bibiliotecas **ggez** e **kiss3d** tem um uso razoável mas que não deve ser suficiente para ter um grande número de códigos de exemplo.
- Variação de versões das bibilotecas: essas bibilotecas, por estarem em ativo desenvolvimento ainda possuem várias versões que o modelo provavelmente misturou.
- Incapacidade do modelo em entender problemas de conflito de dependência: em quase todas as tentativas com a bibiloteco **kiss3d** o modelo exportou uma subdependêcia (**nalgebra**) manualmente em vez de usar o reexportação forneciada pela bibiloteca. Não foi capaz de resolver esse probelma. Em dois casos solicitou para o usuário resolver esses problemas mas sem explicitar como. 

Seria interessante estudar formas de resolver esses problemas.
Uma possibilidade seria fornecer no prompt a documentação das bibilotecas utilizadas mas seria inviavel fornecer toda a documentação de uma bibiloteca devido a limitação do tamanho de cotexto que esses modelos tem. 
Teria que encontrar formas de disponibilizar apenas as partes relevantes da documentação.
