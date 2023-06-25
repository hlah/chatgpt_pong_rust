
Experimento de geração de código usando ChatGPT.

# Objetivos

O objetivo e analisar a criação de codigo usando o ChatGPT. 

Em especial analizar os casos em que o modelo "alucina" métodos ou parâmetros que não existem e descobrir formas de minimzar isso.
Também será analizado o quão próximo o código é da intenção original.

# Metodologia

Serão utilizadas várias formas de fazer um prompt no ChatGPT afim de ser criado um jogo de pong usando a linguagem de programação Rust:

    - Sendo bem aberto na solicitação apenas exegindo a criação de um jogo "pong" e que seja em Rust.
    - Específicando uma biblioteca a ser usada.
    - Específicando a bilioteca e alguns critérios mais específicos em como o jogo deve ser.

Após a geração inicial do código será tentado rodá-lo.
Se houver algum erro de compilação ele será apresentado para de volta para o ChatGPT e solicitado que seja corrigido.
Se o código compilar sem erros o jogo será rodado e se tiver algum problema será informado o ChatGPT para corrigí-lo.
Será apenas aplicado as correções sugeridas pelo chat sem ajuda humana, também não será fornecida nenhuma sugestão de como resolver os problemas.

Esse processo se repetirá até que:

    - O jogo não apresente mais nenhum problema.
    - 3 iterações consecutivas em que o programa não compila.
    - Atingido o número máximo de 10 iterações

Cada variação será tentada pelo menos 2 vezes.

Foi escolhida a linguagem de programação Rust por ser uma linguagem em emergência que certamente foi vista pelo conjunto de treinamento.
Mas que seu ecosistema de bibilotecas ainda está em desenvolvimentos o que deve evidenciar alguma dificuldade do modelo no uso correto delas.

# Resultados

Ao executar os codigos gerados foram encontrados vários erros de compilação, sendo os 4 primeiros mais frequentes:

    - Por usa de métodos inexistentes;
    - Usa do valores ou objectos inexistentes;
    - Uso de argumentos inválidos
    - Uso de dependências incompatíveis (o modelo estava explicitamente importando uma versão de uma dependência da bibiloteca gráfica usada)
    - Não implementação de trait necessário;
    - Auséncia do parâmetro de tipo.

Nas duas tentantivas em que se deixou aberta a bibiloteca a ser usada foi o utilizada a bibiloteca **ggez**. 
Nos casos em que foi informado qual bibiloteca usar foi solicitado o uso da bibiloteca **kiss3d** verão 0.26, foi escolhida essa por ser uma veversão disponíves ainda antes da data de corte dos dados do ChatGPT (lançada ~1 ano antes).

Apenas em uma das tentativas com prompt mais aberto (com uso de **ggez**) foi gerado um código sem erro de compilações.
Foi gerado um jogo bem similar com o pong mas com algums bugs ou coisas faltantes.
Foi então solicitada a correção de um dos bugs (o jogo quebrava quando o ponteiro passava sobre a janela). O model não conseguiu resolver o problema e acabou gerando erros de compilação que não foi capaz de resolver.

As hipóteses para os problemas observados são os seguintes:

    - Baixa exposição do modelo as bibilotecas usadas: ambas bibiliotecas **ggez** e **kiss3d** tem um uso razoável mas que não deve ser suficiente para ter um grande número de códigos de exemplo.
    - Variação de versões das bibilotecas: essas bibilotecas, por estarem em ativo desenvolvimento ainda possuem várias versões que o modelo provavelmente misturou.
    - Incapacidade do modelo em entender problemas de conflito de dependência: em quase todas as tentativas com a bibiloteco **kiss3d** o modelo exportou uma subdependêcia (**nalgebra**) manualmente em vez de usar o reexportação forneciada pela bibiloteca. Não foi capaz de resolver esse probelma. Em dois casos solicitou para o usuário resolver esses problemas mas sem explicitar como. 
