# Changelog

Todas as mudanças notáveis deste projeto serão documentadas neste arquivo.

O formato é baseado no [Keep a Changelog](https://keepachangelog.com/pt-BR/1.0.0/) e este projeto segue [Semantic Versioning (SemVer)](https://semver.org/lang/pt-BR/).

## [v0.1.0] - 2026-11-02
### Added
- **Overlay Dinâmico e Sem Bordas:** Janela flutuante no topo da tela com transparência real, sem bordas, `alwaysOnTop` e sem ícone intrusivo no Alt+Tab.
- **Física de Mola do macOS (Spring Physics):** Transições suaves e elásticas entre os estados Compacto e Expandido utilizando Framer Motion.
- **Integração Nativa com GSMTC (Windows Media):** Detecção automática de músicas em reprodução (Spotify, Chrome, Edge, etc.) com controles de Play, Pause, Avançar e Voltar.
- **Capas de Álbuns Reais (Artwork):** Extração de streams de imagem da sessão do Windows e renderização em base64 nos modos compacto e expandido.
- **Click-through Inteligente:** Desativação automática de captura de cursor fora da pílula para permitir cliques nas janelas de fundo.
- **Status do Sistema e Bateria:** Monitoramento de nível de bateria e indicador de carregamento via APIs Win32 Power Management.
- **Pipeline de Release:** Workflow do GitHub Actions para compilação e publicação automática de instaladores `.msi`/`.exe`.
