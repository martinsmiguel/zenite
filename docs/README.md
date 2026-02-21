# Índice de Documentação - Zenite

Bem-vindo à documentação do projeto Zenite. Esta é a central de conhecimento cumulativo do projeto.

## Specs

Visão de alto nível do planejamento e desenvolvimento:

- [Sistema de Gestão Financeira](./specs/sistema-gestao-financeira.md) - Detalhamento do sistema, funcionalidades e requisitos
- [Roadmap e Tracker](./specs/roadmap-tracker.md) - 5 fases, 19 issues, progresso atual

Documentação técnica completa focada no essencial.

## Documentação de Decisões (ADRs)

Registro de decisões arquiteturais importantes:

- [ADR-000: Estratégia de Documentação como Código](./adrs/000-estrategia-documentacao.md) - Como e por que documentamos
- [ADR-001: Escolha do Framework Tauri 2.0](./adrs/001-escolha-do-framework.md) - Por que escolhemos Tauri
- [ADR-002: File-based com Acesso Serializado](./adrs/002-file-based-serializado.md) - Arquitetura offline-first
- [ADR-003: Armazenamento em Centavos como Inteiros](./adrs/003-armazenamento-centavos.md) - Precisão monetária absoluta
- [ADR-004: Linha Independente para Parcelas](./adrs/004-linha-independente-parcelas.md) - Modelagem de parcelamentos
- [ADR-005: Estratégia de Planejamento MVP](./adrs/005-planejamento-mvp.md) - Por que as fases estão nessa ordem
- [Template para novos ADRs](./templates/template-adr.md)

## Arquitetura

Documentação técnica da estrutura do projeto:

- [Contrato da API Tauri](./architecture/tauri-contract-api.md) - Comandos e eventos entre Frontend e Rust

## API Reference

- [Cargo Doc (Gerado automaticamente)](../src-tauri/target/doc/zenite/index.html) - Documentação da API Rust

Para gerar a documentação Rust localmente:
```bash
cd src-tauri && cargo doc --no-deps --open
```

## Estrutura do Projeto

```
zenite/
├── src/              # Frontend (TypeScript)
├── src-tauri/        # Backend (Rust)
│   ├── src/          # Código Rust
│   └── Cargo.toml    # Dependências Rust
├── docs/             # Esta documentação
└── README.md         # Documentação inicial
```

## Como Contribuir com a Documentação

1. **Novas decisões**: Use o template em `docs/templates/template-adr.md`
2. **Mudanças na API**: Atualize `docs/architecture/tauri-contract-api.md` ANTES de mudar o código
3. **Documentação Rust**: Adicione comentários `///` no código Rust e gere com `cargo doc`

## Links Úteis

- [Documentação Tauri](https://tauri.app/)
- [Referência Rust](https://doc.rust-lang.org/)
- [Guia de Comandos Tauri](https://tauri.app/develop/calling-rust/)

---

**Última atualização**: 2026-02-21
**Versão do Projeto**: 0.1.0
