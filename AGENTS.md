# AGENTS.md

## Privacidade para desenvolvimento e release

Este repositório é público. Nunca use dados reais desta máquina, de outras máquinas locais ou de usuários como código, documentação, fixture, screenshot, log, exemplo ou artefato de release.

- Use nomes, caminhos, UUIDs, endereços e conteúdo sintéticos em testes e exemplos. Identifique valores de exemplo como fictícios quando isso não for óbvio.
- Não copie para o repositório nomes de usuário, caminhos de perfil, nomes de máquina, e-mails privados, hostnames, endereços IP, MACs, UUIDs de discos ou VMs, números de série, inventário de aplicativos, configurações locais, conteúdo de arquivos, capturas de tela ou logs reais.
- Não inclua discos de VM, snapshots, estados salvos, configurações exportadas, dumps, caches, arquivos temporários, arquivos de configuração local ou credenciais.
- Nunca grave tokens, senhas, chaves, cookies, dados de sessão ou conteúdo de arquivos .env. Mantenha arquivos e artefatos locais fora do Git.
- Não use caminhos absolutos desta máquina em código, documentação, testes ou mensagens. Para testes, crie diretórios temporários próprios e remova-os ao terminar.
- A aplicação pode consultar variáveis padrão do Windows em tempo de execução quando necessário para funcionar. Não registre nem salve seus valores em logs, relatórios, telemetria ou fixtures.
- Logs e relatórios devem minimizar dados pessoais. Remova nomes de usuário e caminhos pessoais antes de compartilhar ou versionar qualquer diagnóstico.
- Não envie dados locais para serviços externos. A análise e os testes devem ser locais, salvo solicitação explícita do usuário.

## Verificação antes de commit ou release

- Revise todos os arquivos novos e modificados, inclusive fixtures, documentação, screenshots e metadados.
- Procure caminhos de usuário, UUIDs/MACs, nomes de máquina, e-mails privados, segredos e conteúdo copiado de arquivos locais.
- Confirme que os arquivos de build, diretórios temporários, logs, caches e dados de VM estão ignorados pelo Git e ausentes do commit.
- Fixtures devem ser pequenos, reproduzíveis e totalmente sintéticos.
- Preserve atribuições de autoria e licença já escolhidas deliberadamente para o projeto; não acrescente informações pessoais obtidas do ambiente local.

## Código e testes

- Mantenha identificadores, APIs e código em inglês, sem comentários no código.
- Prefira a menor implementação segura que satisfaça o requisito.
- Teste comportamento de sucesso, falha e limites usando dados sintéticos; nunca execute testes de limpeza contra arquivos reais do usuário.
