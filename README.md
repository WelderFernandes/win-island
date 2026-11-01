# win-island 🏝️

Dynamic Island fluida, moderna e reativa no estilo macOS/iOS, construída nativamente para o Windows 11 utilizando **Tauri v2**, **Rust** e **React + TypeScript + Framer Motion**.

---

## 🚀 Arquitetura e Tecnologias

- **Frontend / Motion Engine:** React 19, TypeScript, Framer Motion (para dinâmicas de mola/spring physics ultra-fluidas).
- **Core / Backend:** Rust (Tauri v2) para controle de janelas sem borda (*borderless overlay*), transparência real e integração com APIs Win32/WinRT.
- **Integrações de Sistema:**
  - GSMTC (*Global System Media Transport Controls*) para Spotify, navegadores e players de mídia.
  - Controle de volume, bateria e status de rede.
  - Click-through inteligente fora da ilha para não bloquear cliques na tela.

---

## 📋 Pré-requisitos

1. **Node.js** (v18+) e **npm**
2. **Rust & Cargo** (`rustup default stable-x86_64-pc-windows-msvc`)
3. **Microsoft C++ Build Tools** (Visual Studio Installer com a carga de trabalho de desenvolvimento para Desktop com C++)

---

## 🛠️ Como Executar em Desenvolvimento

```powershell
# Instalar dependências
npm install

# Iniciar em modo de desenvolvimento com hot-reload
npm run tauri dev
```

---

## 📦 Padrão de Controle de Versão & Git

Este repositório adota o padrão **[Conventional Commits](https://www.conventionalcommits.org/)**:

- `feat:` Novas funcionalidades (ex.: `feat: adicionar suporte a player do Spotify`)
- `fix:` Correções de bugs (ex.: `fix: corrigir posicionamento em múltiplos monitores`)
- `docs:` Alterações em documentações
- `style:` Formatação e estilo visual sem alteração de lógica
- `refactor:` Refatoração de código
- `chore:` Tarefas de build, dependências ou automações CI/CD

---

## 🏷️ Tags, Releases e CI/CD

O projeto conta com automação via **GitHub Actions** (`.github/workflows/release.yml`).

Para gerar uma nova versão oficial:
1. Atualize a versão no `package.json` e no `src-tauri/tauri.conf.json`.
2. Gere um commit de versão:
   ```powershell
   git commit -m "chore(release): bump v0.1.0"
   ```
3. Crie e envie a tag semântica:
   ```powershell
   git tag v0.1.0
   git push origin main --tags
   ```
4. O GitHub Actions iniciará automaticamente o pipeline que compila o instalador (`.msi` / `.exe`) e publica a **GitHub Release** com o changelog gerado automaticamente.
