# ADR-005: Estratégia de Planejamento MVP - Zenite

## Status
Proposto

## Contexto
O desenvolvimento do Zenite precisa de um plano claro de priorização para transformar decisões arquiteturais (ADR-001 a ADR-004) em funcionalidades implementadas.

Questões centrais:
- Por que as fases estão nessa ordem específica?
- Por que cada issue tem a prioridade que tem?
- O que define o que é essencial vs. opcional?

Necessitávamos de uma decisão explícita sobre o roadmap que justificasse cada decisão, especialmente sobre o que implementar antes e o que deixar para depois.

## Decisão
Adotar estratégia de desenvolvimento sequencial em 5 fases com priorização baseada em criticidade funcional e dependências técnicas.

## Justificativas de Cada Estágio

### Estágio 1: Base do Sistema
**Razão principal**: Fundamento técnico para qualquer operação.

**Argumento técnico**:
- Schema é a base de todas as operações
- File locking impede uso em múltiplos dispositivos
- CRUD é a única forma de interagir com dados
- Regras de negócio garantem integridade

Se fizermos Funcionalidades Essenciais sem base, tudo precisa ser refeito.

### Estágio 2: Funcionalidades Essenciais
**Razão principal**: Capacidade de interação com o sistema.

**Argumento técnico**:
- Dashboard depende de CRUD para mostrar dados
- Formulário depende de validações para funcionar
- Sistema de arquivos permite persistência
- Todas usam base como fundamento

### Estágio 3: Capacidade de Acesso
**Razão principal**: Busca e visualização de informações.

**Argumento técnico**:
- Extrato depende de CRUD (já tem)
- Status depende de CRUD (já tem)
- Participantes e histórico são consultas sobre o que já existe

### Estágio 4: Preservação de Dados
**Razão principal**: Segurança e continuidade do sistema.

**Argumento técnico**:
- Backup precisa do CRUD existente
- Reconciliação precisa do histórico
- Import/export são operações sobre dados já existentes
- Plano de preservação é necessário para sobrevivência do software

### Estágio 5: Disponibilização
**Razão principal**: Distribuição final do sistema.

**Argumento técnico**:
- Otimização melhora o que já está funcionando
- Documentação ajuda no uso do sistema
- Release finaliza o desenvolvimento

**Observação sobre testes**: Cada issue deve ter critérios de aceite claros, incluindo casos de sucesso e erro. Testes são parte integrante de cada issue, não apenas no final.

## Estrutura em 5 Estágios

**1. Base do Sistema**: Schema, locking, CRUD, regras
**2. Funcionalidades Essenciais**: Dashboard, lançamento, sistema de arquivos
**3. Capacidade de Acesso**: Extrato, filtros, status, participantes, histórico
**4. Preservação de Dados**: Backup, reconciliação, CSV/JSON, plano de preservação
**5. Disponibilização**: Otimização, documentação, release

## Cadeia de Dependências

```
1. Base do Sistema
  ↓ depende de
2. Funcionalidades Essenciais
  ↓ depende de
3. Capacidade de Acesso
  ↓ depende de
4. Preservação de Dados
  ↓ depende de
5. Disponibilização
```

**Razão**: Se pularmos uma fase, a próxima terá problema de dependência.

**Exemplo**: Se fazer Estágio 3 antes de Estágio 2, não teremos formulário para lançar transações que o extrato precisará mostrar.

## Definição de Essencial vs. Opcional

### Essencial (19 issues)
Tudo que é necessário para o sistema funcionar:
- Schema do banco (base de dados)
- CRUD (criar, ler, atualizar, deletar)
- Dashboard (visão geral)
- Formulário (entrada de dados)
- Status (classificação automática)
- Sistema de arquivos (abrir/salvar .planner)
- Import/export CSV/JSON (preservação de dados)
- Plano de preservação de dados (documentação)

### Opcional (recursoiros)
Funcionalidades que melhoram mas não são necessárias:
- Otimizações de performance

**Razão de focar no essencial**: Implementar tudo é melhor que fazer metade mal feito. Foco no essencial garante MVP funcional.

## Testes e Critérios de Aceite

Testes devem ser parte integrante de cada issue, não apenas no final. Cada issue deve ter critérios de aceite claros e concretos:

**Exemplo CRUD transações**:
- [ ] Criar transação com dados válidos
- [ ] Criar transação com dados inválidos (erro)
- [ ] Ler transações existentes
- [ ] Atualizar transação existente
- [ ] Deletar transação existente
- [ ] Tratar casos de erro

**Exemplo Schema do banco**:
- [ ] Tabelas criadas com campos corretos
- [ ] Foreign keys funcionando
- [ ] Índices criados
- [ ] Tipos centavos aplicados

## Evolução do Sistema

Evolução não precisa ser futurista. Pode ser:
- Melhorias de prevenção contra corrupção de arquivos
- Ampliação do CRUD com novas operações
- Melhor formas de exportar e analisar dados
- Ferramentas de manutenção
- Melhorias baseadas em uso real

## Data
2026-02-21

## Participantes
- Miguel Martins
