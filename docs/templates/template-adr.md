# ADR-XXX: [TÍTULO DA DECISÃO]

## Status
Proposto / Aceito / Rejeitado / Deprecado

## Contexto
[Qual problema ou oportunidade surgiu no projeto?]

Exemplo: O frontend precisa de acesso ao sistema de arquivos, mas o Tauri tem uma API específica para isso. Precisamos decidir a melhor abordagem mantendo a segurança e performance.

## Decisão
[O que vamos fazer? Seja específico e técnico.]

Exemplo: Usar o comando `fs::read_file` do Tauri via plugin `tauri-plugin-fs` em vez de invocar um servidor Node.js ou usar APIs web.

## Consequências

### Positivas
- Benefício 1: [Ex: Segurança nativa do Rust]
- Benefício 2: [Ex: Performance e tamanho reduzido do binário]
- Benefício 3: [Ex: Integração limpa com o ecossistema Tauri]

### Negativas
- Risco 1: [Ex: Aumenta o tamanho do binário final em X MB]
- Risco 2: [Ex: Requer aprendizado da equipe em Rust]
- Risco 3: [Ex: Limitações de permissões do Tauri]

## Alternativas Consideradas

### Alternativa 1: [Nome da Alternativa]
- **Descrição**: [Como funcionaria]
- **Por que foi rejeitada**: [Motivo técnico ou de negócio]

### Alternativa 2: [Nome da Alternativa]
- **Descrição**: [Como funcionaria]
- **Por que foi rejeitada**: [Motivo técnico ou de negócio]

## Notas de Implementação
[Detalhes técnicos importantes, referências a arquivos, ou passos de implementação]

## Data
[Data da decisão]

## Participantes
- [Nome 1]
- [Nome 2]
