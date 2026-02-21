# Zenite

Aplicação desktop multiplataforma construída com Tauri 2.0.

## Tecnologias

- **Frontend**: TypeScript + Vite
- **Backend**: Rust (Tauri)
- **UI**: Vanilla HTML/CSS

## Documentação

Acesse a [documentação completa](./docs/README.md) para entender:
- Decisões arquiteturais (ADRs)
- Contrato da API entre Frontend e Rust
- Como adicionar novos comandos Tauri

## Desenvolvimento

### Pré-requisitos
- [Node.js](https://nodejs.org/)
- [Rust](https://rustup.rs/)

### Comandos

```bash
# Instalar dependências
npm install

# Iniciar em modo desenvolvimento
npm run tauri dev

# Compilar para produção
npm run tauri build

# Gerar documentação Rust
cd src-tauri && cargo doc --no-deps --open
```

### Estrutura do Projeto

```
zenite/
├── src/              # Frontend TypeScript
├── src-tauri/        # Backend Rust
├── docs/             # Documentação
└── README.md
```

## Engenharia de Conhecimento

Este projeto segue os princípios de **Engenharia de Conhecimento Cumulativo**:
- Todo código documenta a si mesmo e suas decisões
- ADRs registram porque fazemos o que fazemos
- Contratos API mantêm sincronia entre Frontend e Rust

Leia mais em: [docs/README.md](./docs/README.md)

## Fluxo de Desenvolvimento

Fluxo Git Flow com branches develop para desenvolvimento de features e main para releases estáveis, onde todas as alterações são primeiro integradas no develop via merge request e apenas versões validadas são promovidas ao main.

---

**Versão**: 0.1.0
