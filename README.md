# WinClean 🧹

Aplicativo **gratuito e open source**, feito com **Tauri 2 + React + TypeScript + Rust**, para inspecionar e remover arquivos temporários antigos do usuário no Windows 11.

> ⚠️ MVP experimental, não auditado. Teste primeiro em máquina virtual e mantenha backup dos arquivos importantes. A limpeza é permanente e não passa pela Lixeira.

## Pré-requisitos

- Windows 11, Node.js LTS, npm
- Rust (via rustup), ferramentas de compilação C++ MSVC do Visual Studio Build Tools
- Microsoft Edge WebView2 (normalmente presente no Windows 11)

Consulte também: https://v2.tauri.app/start/prerequisites/

## Executar

```powershell
npm install
npm run tauri dev
```

## Compilar instalador Windows

```powershell
npm run tauri build
```

Os instaladores gerados ficam sob `src-tauri/target/release/bundle/`, conforme o formato de bundle habilitado.

## Primeira versão (v0.1)

- Varre **somente** a pasta temporária retornada pelo Windows para o usuário atual (`std::env::temp_dir()`).
- Considera arquivos com **mais de 24 horas** desde a última modificação.
- Ignora links simbólicos, diretórios e arquivos inacessíveis.
- Limita o conjunto encontrado a 100 mil itens.
- Exibe tamanho, idade e caminho relativo de cada arquivo.
- Permite selecionar e confirmar exclusão permanente.
- Antes de remover, verifica novamente identidade básica do item (caminho canônico, tamanho e data de modificação).
- Não necessita de privilégios administrativos para executar.

## Limitações de segurança

- Arquivos antigos podem ser necessários a aplicações em execução. A idade não garante que podem ser removidos.
- Verificações de caminho/tamanho/data não eliminam completamente condições de corrida do sistema de arquivos (TOCTOU); um processo local concorrente ainda pode alterar arquivos entre verificações e exclusão.
- Não foi realizado teste de integração no Windows nem auditoria externa.
- O frontend lista os 500 maiores arquivos para não travar a renderização; o seletor geral contempla todos os itens digitalizados.
- A interface carrega a fonte Inter pela rede quando disponível. Isso pode ser removido para ter interface 100% offline.

## Próximas etapas

1. Testes Windows e testes de integração com pastas TEMP artificiais.
2. Adicionar limpeza por quarentena/Lixeira como alternativa segura à remoção permanente.
3. Evitar TOCTOU com abertura por handle e verificação de identidade nativa Windows.
4. Progresso e cancelamento de scanner em thread dedicada, sem bloquear a interface.
5. Perfis opcionais: cache de navegador com navegador fechado; Windows Update via APIs oficiais; hibernação com consentimento e elevação limitada.
6. CI com lint, cargo test, npm build e release assinado.

## Licença

MIT. Contribuições são bem-vindas.
