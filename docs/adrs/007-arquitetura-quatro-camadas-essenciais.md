# ADR-007: Estrutura de Arquitetura em Quatro Camadas Essenciais

## Status
Aceito

## Contexto
O desenvolvimento do Zenite enfrenta a tensão clássica entre abstração e essencialidade. Estruturas muito elaboradas introduzem custo de complexidade acidental; estruturas muito simples poluem camadas e dificultam testes. A questão central: qual é a estrutura mínima necessária que preserve testabilidade e separação de concerns, sem cair em over-engineering?

## Decisão
Adotar quatro camadas arquiteturais (`domain`, `application`, `infrastructure`, `interface`), com a camada `application` preenchida sob demanda — não antecipadamente. Cada elemento da arquitetura deve atender a pelo menos um de três critérios para existir: eliminar risco, habilitar reuso ou esconder complexidade.

O guia detalhado de trabalho (onde colocar cada coisa, regras por camada, sinais de alerta) está em **[docs/handbook/arquitetura-camadas.md](../handbook/arquitetura-camadas.md)**.

## Estrutura Resumida

```
src/
├── domain/              # Regras de negócio puras (obrigatório)
│   ├── entities/        # Transacao, Parcela, etc.
│   ├── services/        # Cálculos financeiros
│   └── repositories/    # Traits apenas (~20 linhas, sem implementação)
│
├── application/         # Coordenação quando >1 passo de orquestração
│   └── (vazio inicialmente, preenchido sob demanda)
│
├── infrastructure/      # Implementações concretas
│   └── sqlite/
│
└── interface/           # Adaptadores Tauri
    └── tauri/
        ├── commands.rs  # Adaptadores mínimos
        └── state.rs     # Injeção de dependência
```

## Quando Criar uma Camada ou Arquivo

Cada elemento da arquitetura deve justificar sua existência. Criar apenas se atender a pelo menos um dos três critérios:

1. **Elimina risco** — Habilita testes sem dependências externas ou isola uma decisão que pode mudar
2. **Habilita reuso** — Código usado em mais de um contexto (ex: Tauri e CLI futuro)
3. **Esconde complexidade** — Orquestração de múltiplos passos que poluiria outra camada

Se nenhum se aplica, não crie.

## Princípio de Criação

Criar um elemento apenas se atender a pelo menos um dos três critérios:

1. **Elimina risco** — Habilita testes sem dependências externas ou isola uma decisão que pode mudar
2. **Habilita reuso** — Código usado em mais de um contexto
3. **Esconde complexidade** — Orquestração de múltiplos passos que poluiria outra camada

## Alternativas Rejeitadas

- **3 camadas (orquestração no domain):** Polui regras puras; domínio perde testabilidade isolada
- **3 camadas (orquestração na interface):** Commands inchados; testes exigem MockRuntime para lógica que não é de interface
- **4 camadas com `application` antecipado:** Boilerplate sem valor imediato; viola princípio de essencialidade

## Consequências

### Positivas
- Testabilidade máxima com mínimo de boilerplate
- `application` existe apenas quando justificada por orquestração real
- Domain puro: reutilizável e testável sem mocks de infraestrutura

### Negativas
- Curva inicial: time deve aprender quando mover lógica para `application` vs. manter inline
- Revisão periódica necessária (ver handbook)

## Data
2026-02-24

## Participantes
- Miguel Martins
