# Guia de Contribuição e Versionamento

## 📌 Padrão de Commits (Conventional Commits)

Cada commit deve seguir o formato:
`<tipo>(<escopo>): <descrição no imperativo>`

Exemplos:
- `feat(overlay): implementar física elástica de expansão`
- `fix(win32): corrigir click-through em áreas transparentes`
- `docs(readme): atualizar instruções de build com Rust`

## 🏷️ Fluxo de Branches e Releases

1. Desenvolvimento contínuo na branch `main` ou feature branches (`feature/nome-da-feature`).
2. Pull Requests devem ser validados pelo CI de lint/build.
3. Releases são marcadas exclusivamente por **Git Tags** (`vX.Y.Z`).
