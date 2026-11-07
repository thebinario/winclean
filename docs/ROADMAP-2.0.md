# Roadmap 2.0 — WinClean

**Planejamento de produto e sprints**  
**Plataforma:** Windows 10/11 · **Stack:** Tauri 2, Rust, React e TypeScript  
**Status:** proposta para refinamento do backlog

> Seu Windows. Seu espaço. Seu controle.

## 1. Visão

O WinClean quer ajudar as pessoas a entender, recuperar e gerenciar espaço no Windows com controle sobre cada ação. O produto deve ser gratuito, open source, leve e transparente, sem anúncios, assinaturas, telemetria obrigatória ou promessas artificiais de aceleração.

Princípios do produto:

- **Seguro:** mostrar o impacto antes de alterar dados e bloquear caminhos protegidos.
- **Reversível:** preferir quarentena e restauração quando tecnicamente possível.
- **Transparente:** explicar o que será alterado, o que foi ignorado e por quê.
- **Leve:** executar análises sem travar a interface e respeitar limites de recursos.
- **Privado:** manter nomes de arquivos e histórico localmente.
- **Auditável:** regras compreensíveis, código aberto e operações nativas documentadas.

## 2. Estado atual e escopo

A base 0.1 já oferece análise de arquivos TEMP do usuário e exclusão selecionada. O repositório também contém suporte inicial para localizar discos VDI dinâmicos do VirtualBox e solicitar compactação com a VM desligada, além de fixtures para cenários de disco elegível, bloqueado, fixo e associado a snapshot.

A exclusão TEMP atual é permanente. Proteções contra condições de corrida, quarentena, restauração, progresso cancelável e cobertura de testes Windows ainda são trabalho futuro. O suporte a VDI deve continuar limitado a discos base desbloqueados até que snapshots, cadeias e outros formatos tenham desenho e testes próprios.

As versões e sprints abaixo são propostas. O tamanho sugerido é de duas semanas por sprint, sem datas prometidas. Reestimar escopo e capacidade durante o planejamento de cada sprint.

## 3. Marcos de produto

| Marco | Versões | Resultado esperado |
| --- | --- | --- |
| A — Base confiável | 0.2–0.3 | Operações seguras, reversíveis, testadas e com interface compreensível |
| B — Produto utilizável | 0.4–0.6 | Categorias adicionais, análise de armazenamento e atalhos para ferramentas oficiais |
| C — Produto público | 0.7–1.0 | Regras e automação controladas, desempenho validado, beta e lançamento estável |

## 4. Plano inicial de sprints

### Sprint 1 — Testes e limites de segurança (v0.2)

- Criar testes unitários Rust para validação, regras e operações de arquivos.
- Criar testes de integração Windows com diretórios artificiais e arquivos temporários.
- Definir uma política explícita de caminhos autorizados e caminhos bloqueados.
- Cobrir symlinks, junctions, arquivos em uso, permissões negadas e mudanças entre análise e ação.

**Saída:** testes reproduzíveis demonstram que a limpeza fica dentro da raiz autorizada e recusa entradas inseguras.

### Sprint 2 — Quarentena e restauração (v0.2)

- Substituir a exclusão permanente por movimentação para uma quarentena local no mesmo volume, quando possível.
- Registrar metadados mínimos para restaurar cada item ao caminho original.
- Implementar restauração individual e em lote.
- Definir comportamento quando o destino original já existir ou a movimentação entre volumes não for atômica.

**Saída:** itens elegíveis podem ser restaurados; conflitos não sobrescrevem arquivos.

### Sprint 3 — Expiração, tarefas canceláveis e relatórios (v0.2)

- Definir expiração configurável da quarentena e uma ação explícita para remoção definitiva.
- Executar o scanner fora da thread da interface, com progresso e cancelamento.
- Exibir contagens de encontrados, ignorados, bloqueados, movidos, restaurados e com falha.
- Adicionar logs locais sem nomes completos de arquivos por padrão e sem conteúdo pessoal.

**Saída:** scanner cancelável, relatório por operação e política de retenção documentada.

### Sprint 4 — Dashboard e navegação (v0.3)

- Criar dashboard com uso por unidade e espaço potencialmente recuperável, deixando claras as estimativas.
- Organizar navegação lateral por categorias e páginas.
- Adicionar tema claro, escuro e automático.
- Exibir estados vazios, carregamento, erro e sucesso com linguagem clara.

**Saída:** navegação e dashboard utilizáveis com teclado e sem depender da execução de uma limpeza.

### Sprint 5 — Tabelas, filtros e confirmação acessível (v0.3)

- Adicionar pesquisa, filtros e ordenação às tabelas.
- Mostrar nível de risco e motivo de elegibilidade por categoria.
- Exibir resumo detalhado antes da confirmação, incluindo total, categorias e reversibilidade.
- Revisar responsividade, foco, contraste, rótulos e leitores de tela.

**Saída:** a pessoa entende e revisa a seleção antes de qualquer operação.

### Sprint 6 — Limpeza Windows conservadora (v0.4)

- Adicionar TEMP do Windows com regras independentes e caminhos validados.
- Avaliar miniaturas, relatórios de erro e logs descartáveis com políticas por tipo.
- Ignorar arquivos em uso e explicar a decisão.
- Integrar a Lixeira do Windows somente com confirmação explícita e comportamento documentado.

**Saída:** cada categoria tem regra, prévia, teste e opção de exclusão antes de ser habilitada.

### Sprint 7 — Caches de navegadores e aplicativos (v0.4)

- Suportar caches de navegadores compatíveis por regras versionadas.
- Adicionar caches de aplicativos conhecidos, com lista e versão de regra visíveis.
- Detectar aplicativos abertos e solicitar que sejam fechados; nunca forçar encerramento.
- Permitir excluir categorias da análise antes de iniciar a limpeza.

**Saída:** somente caches identificados por regras mantidas e cobertas por testes aparecem como elegíveis.

### Sprint 8 — Mapa de armazenamento (v0.5)

- Analisar unidades e pastas sob demanda, sem varreduras ocultas.
- Exibir uso por pasta e um treemap navegável.
- Listar os 100 maiores arquivos com caminho, tamanho e data.
- Detectar arquivos antigos e downloads grandes com filtros e exclusões.

**Saída:** a análise é cancelável, informa itens ignorados e não altera dados.

### Sprint 9 — Duplicados e destinos favoritos (v0.5)

- Detectar candidatos a duplicados por tamanho e confirmar por hash.
- Exibir grupos lado a lado e exigir seleção manual.
- Permitir filtros por extensão e período.
- Salvar pastas favoritas localmente e analisar discos externos apenas quando solicitados.

**Saída:** nenhum duplicado é removido automaticamente; a interface mostra o método usado para identificá-lo.

### Sprint 10 — Inventário de aplicativos e inicialização (v0.6)

- Exibir aplicativos instalados com dados obtidos de fontes suportadas do Windows.
- Oferecer atalhos para o desinstalador nativo, sem executar desinstalação silenciosa.
- Listar itens de inicialização e permitir abrir as configurações nativas correspondentes.
- Mostrar informações de espaço por unidade e saúde básica do disco quando disponível.

**Saída:** ferramentas informativas e ações delegadas a interfaces oficiais do Windows.

### Sprint 11 — Operações administrativas específicas (v0.6)

- Avaliar hibernação e hiberfil.sys, explicando o impacto antes de abrir a configuração oficial.
- Integrar limpeza de componentes do Windows por ferramentas oficiais, sem apagar componentes manualmente.
- Gerenciar pontos de restauração por interfaces ou APIs suportadas, com confirmação e explicação.
- Implementar elevação somente para uma operação específica, após revisão de segurança.

**Saída:** nenhum privilégio administrativo é solicitado na inicialização do aplicativo.

### Sprint 12 — Regras locais e perfis (v0.7)

- Definir formato versionado para regras e perfis de limpeza.
- Permitir inclusões e exclusões locais com validação de caminho.
- Oferecer perfis predefinidos e modo de simulação antes de aplicar uma regra.
- Registrar a versão das regras usada em cada operação.

**Saída:** regras inválidas ou fora das áreas autorizadas são recusadas; execução automática permanece desativada por padrão.

### Sprint 13 — Agendamento com controle visível (v0.7)

- Adicionar agendamento opcional para análises e notificações.
- Exigir consentimento separado para qualquer limpeza agendada.
- Exibir próxima execução, histórico, pausa e cancelamento.
- Garantir que tarefas ocorram na sessão do usuário e não elevem privilégios silenciosamente.

**Saída:** nenhuma limpeza silenciosa; toda tarefa agendada pode ser inspecionada e interrompida.

### Sprint 14 — Desempenho e recuperação (v0.8)

- Estabelecer medições de CPU, memória, tempo de análise e uso de disco em conjuntos de referência.
- Processar resultados em lotes e limitar filas, memória e quantidade de itens.
- Melhorar cancelamento, tratamento de falhas e recuperação após interrupção.
- Validar o comportamento em discos lentos, grandes, externos e com acesso negado.

**Saída:** limites e metas de desempenho publicados; falhas não deixam a quarentena em estado ambíguo.

### Sprint 15 — Beta e compatibilidade (v0.9)

- Executar matriz de testes em Windows 10/11 e nas arquiteturas anunciadas.
- Revisar acessibilidade e fluxos de instalação, atualização e remoção.
- Preparar beta público com limitações conhecidas e canal de relato de problemas.
- Coletar feedback voluntário sem telemetria obrigatória e sem nomes de arquivos.

**Saída:** bloqueadores de segurança e estabilidade triados; limitações e instruções beta publicadas.

### Sprint 16 — Correções beta e preparação de release (v0.9)

- Priorizar problemas reproduzíveis do beta e adicionar regressões aos testes.
- Verificar instaladores limpos, atualização, remoção e restauração de dados da quarentena.
- Revisar dependências, licenças, configuração de build e cadeia de publicação.
- Finalizar documentação de instalação, desenvolvimento, segurança e contribuição.

**Saída:** checklist de lançamento v1.0 pode ser avaliado com evidência para cada critério.

### Sprint 17 — Lançamento estável (v1.0)

- Fechar os critérios de lançamento abaixo ou registrar explicitamente os itens adiados.
- Publicar instaladores assinados e checksums das versões.
- Publicar notas de versão, instruções de atualização e política de suporte.
- Marcar a release no GitHub e manter processo de correção para vulnerabilidades e regressões.

**Saída:** release reproduzível, documentada e verificável; nenhuma funcionalidade de limpeza é ativada sem os gates de segurança.

## 5. Backlog por versão

### v0.1 — MVP

- Scanner de arquivos TEMP do usuário atual.
- Limite de idade e de quantidade de itens.
- Exclusão selecionada com revalidação básica.
- Suporte inicial para analisar e compactar VDI dinâmico base desbloqueado do VirtualBox.
- Fixtures de simulação para discos dinâmicos, bloqueados, fixos e com snapshot.

### v0.2 — Segurança e robustez

1. Testes unitários Rust e testes de integração Windows.
2. Limpeza reversível por quarentena.
3. Restauração e expiração da quarentena.
4. Proteção contra symlinks, junctions e condições de corrida.
5. Bloqueio de caminhos protegidos do sistema.
6. Logs locais sem dados sensíveis desnecessários.
7. Scanner assíncrono com progresso e cancelamento.
8. Relatório de arquivos ignorados, bloqueados e processados.

### v0.3 — Interface e experiência

1. Dashboard com espaço utilizado e recuperável.
2. Navegação lateral e categorias de limpeza.
3. Temas claro, escuro e automático.
4. Tabela com filtros, pesquisa e ordenação.
5. Indicadores de risco por categoria.
6. Confirmação com resumo detalhado.
7. Notificações e estados de erro compreensíveis.
8. Interface responsiva e acessível.

### v0.4 — Limpeza avançada

1. TEMP do Windows e do usuário com regras próprias.
2. Miniaturas e caches descartáveis do Windows.
3. Relatórios de erro e logs elegíveis.
4. Lixeira do Windows com confirmação.
5. Caches de navegadores compatíveis.
6. Caches de aplicativos conhecidos por regras versionadas.
7. Detecção de aplicativos abertos antes da limpeza.
8. Prévia de arquivos e seleção de categorias.

### v0.5 — Analisador de armazenamento

1. Uso por unidade e pasta.
2. Treemap de arquivos e diretórios.
3. Lista dos 100 maiores arquivos.
4. Arquivos antigos e downloads grandes.
5. Duplicados por tamanho e hash.
6. Filtros por extensão e período.
7. Exclusões e pastas favoritas.
8. Análise sob demanda de discos externos.

### v0.6 — Ferramentas do Windows

1. Gerenciamento de itens de inicialização por interfaces nativas.
2. Visualização de aplicativos instalados.
3. Atalhos para desinstalação nativa.
4. Hibernação e hiberfil.sys com explicação e confirmação.
5. Limpeza de componentes do Windows por ferramentas oficiais.
6. Pontos de restauração por recursos suportados.
7. Informações de armazenamento e saúde básica do disco.
8. Elevação administrativa somente em operações específicas.

### v0.7 — Automação e regras

Proposta a refinar durante o planejamento:

1. Formato versionado de regras locais.
2. Perfis predefinidos e personalizados.
3. Inclusões e exclusões de caminhos com validação.
4. Simulação obrigatória antes de habilitar uma regra nova.
5. Agendamento opcional de análises.
6. Consentimento separado para limpeza agendada.
7. Histórico local com pausa, cancelamento e próxima execução.
8. Importação/exportação explícita de regras sem dados pessoais.

### v0.8 — Performance e confiabilidade

Proposta a refinar durante o planejamento:

1. Benchmarks repetíveis de CPU, memória e duração.
2. Metas publicadas para conjuntos de referência.
3. Scanner em lotes com limites de memória.
4. Cancelamento responsivo em todas as operações longas.
5. Recuperação consistente após falha ou encerramento.
6. Testes em discos lentos, grandes, externos e com erros.
7. Limites configuráveis para tamanho e quantidade de itens.
8. Diagnóstico local exportável com dados pessoais removidos por padrão.

### v0.9 — Beta e qualidade

Proposta a refinar durante o planejamento:

1. Matriz de compatibilidade Windows 10/11 e arquiteturas anunciadas.
2. Testes de instalação, atualização, remoção e restauração.
3. Revisão de acessibilidade por teclado e leitor de tela.
4. Revisão de localização e mensagens de erro.
5. Beta público com limitações e canal de feedback.
6. Triagem de problemas com testes de regressão.
7. Revisão de dependências, licenças e configuração de build.
8. Evidências reproduzíveis para o checklist v1.0.

### v1.0 — Lançamento estável

Proposta a refinar durante o planejamento:

1. Nenhuma exclusão fora dos caminhos autorizados nos testes.
2. Operações reversíveis ou marcadas como irreversíveis antes da confirmação.
3. Scanner cancelável sem travar a interface.
4. Testes automatizados e builds de release passando.
5. Instaladores verificados nas arquiteturas anunciadas.
6. Documentação de instalação, desenvolvimento e contribuição.
7. Atualizações verificadas criptograficamente.
8. Sem telemetria obrigatória nem limpezas ocultas.

## 6. Arquitetura alvo

Manter Tauri 2. Organizar o código por domínio para que validação e operações de arquivo não fiquem concentradas em um único módulo.

```text
winclean/
├── src/
│   ├── components/
│   ├── pages/
│   ├── features/
│   ├── hooks/
│   └── services/
├── src-tauri/
│   └── src/
│       ├── commands/
│       ├── scanner/
│       ├── cleaner/
│       ├── rules/
│       ├── storage/
│       ├── windows/
│       └── safety/
├── tests/
├── docs/
└── .github/workflows/
```

A migração deve ser incremental, começando por testes e módulos de segurança da v0.2. Cada comando Tauri deve validar entradas no backend; a interface não é fronteira de segurança.

## 7. Gates de segurança

Aplicar estes gates a toda nova categoria de limpeza ou ferramenta que altere dados:

- **Dry-run:** apresentar candidatos e totais antes da ação.
- **Escopo:** permitir apenas raízes explicitamente autorizadas e validar caminhos no backend.
- **Links e identidade:** recusar links simbólicos, junctions não autorizados e substituições detectadas entre análise e ação.
- **Quarentena:** preferir movimentação reversível; quando não for possível, explicar a irreversibilidade antes da confirmação.
- **Arquivos em uso:** ignorar e explicar; não forçar encerramento ou remoção.
- **Permissões:** executar como usuário comum por padrão e elevar somente para uma operação específica.
- **Confirmação:** exigir consentimento explícito para operações destrutivas ou agendadas.
- **Privacidade:** manter caminhos e histórico localmente; diagnósticos exportados devem minimizar ou remover dados pessoais.
- **Testes:** cobrir cenário positivo, cenário bloqueado, erro, cancelamento e recuperação.
- **VirtualBox:** nunca alterar discos durante análise; compactar apenas após validação do estado, formato e cadeia. Informar que o trim do sistema convidado pode ser necessário para recuperar mais espaço.

## 8. Definição de pronto por sprint

Uma entrega só entra em release quando:

1. Critérios de aceite foram definidos antes da implementação.
2. Testes relevantes passam no Windows suportado.
3. Erros e cancelamento deixam o estado consistente.
4. A interface explica impacto, escopo e reversibilidade.
5. Nenhuma permissão é ampliada sem necessidade documentada.
6. Documentação e limitações foram atualizadas.
7. A alteração não introduz telemetria ou execução em segundo plano sem consentimento.
8. O build afetado foi executado e o resultado registrado.

## 9. Ritos de planejamento

- **Refinamento:** revisar objetivo, riscos, dependências e critérios de aceite antes de estimar.
- **Planejamento:** escolher entregas que caibam na capacidade real da sprint; não assumir que todos os itens listados cabem automaticamente.
- **Revisão:** demonstrar o fluxo com diretórios artificiais ou ambiente de teste, sem limpar dados reais.
- **Retrospectiva:** registrar uma melhoria concreta para a próxima sprint.
- **Release:** promover somente o que passou pelos gates de segurança, testes e documentação.

A próxima sprint recomendada é a Sprint 1 da v0.2: criar a base de testes Windows e definir os caminhos autorizados antes de implementar mais categorias de limpeza.
