# ADR-000: Estratégia de Documentação como Código - Engenharia de Conhecimento Cumulativo

## Status
Aceito

## Contexto
O projeto Zenite tem como premissa ser um sistema de gestão financeira colaborativa que funcionará por anos, passando por muitas evoluções e possivelmente diferentes desenvolvedores. Precisávamos de uma estratégia de documentação que:

1. **Preservasse o conhecimento** sobre decisões técnicas e de negócio
2. **Fosse versionada** junto com o código-fonte
3. **Fosse acessível** sem ferramentas especiais (apenas leitura de Markdown)
4. **Descrevesse o "porquê"** das decisões, não apenas o "como"
5. **Evitasse documentação obsoleta** que ninguém atualiza
6. **Facilitasse onboarding** de novos desenvolvedores

A pergunta central: como documentar um projeto de forma que o conhecimento seja cumulativo e não se perda ao longo do tempo?

## Decisão
Adotar uma **Estratégia de Documentação como Código** baseada em três pilares:

### 1. ADRs (Architecture Decision Records)
Registros de decisões arquiteturais significativas no diretório `docs/adrs/`.

**Formato**: `XXX-titulo-descritivo.md`
**Template**: Estrutura padronizada (ver `docs/templates/template-adr.md`)
**Conteúdo**: Contexto, Decisão, Consequências, Alternativas, Notas

### 2. Specs
Documentos de visão de alto nível no diretório `docs/specs/`.

**Propósito**: Descrever sistemas inteiros, requisitos de negócio, hipóteses
**Público-alvo**: Stakeholders, product owners, desenvolvedores
**Formato**: Markdown livre, mas estruturado

### 3. Contratos de API
Documentação técnica de interfaces no diretório `docs/architecture/`.

**Propósito**: Definir contratos entre Frontend e Backend ANTES da implementação
**Atualização**: Deve ser atualizada ANTES de mudar o código
**Conteúdo**: Input/output, tipos, exemplos, segurança

### Princípios Fundamentais

#### Documentação Antecipada
**Regra de Ouro**: Documente ANTES de implementar.
- Mudança na API? Atualize o contrato primeiro.
- Nova decisão arquitetural? Crie o ADR antes do código.
- Isso força clareza de pensamento e evita retrabalho.

#### Documentação Versionada
- Toda documentação vive no repositório Git
- Histórico completo de evolução das decisões
- Code review inclui revisão de documentação

#### Documentação Executável
- Quando possível, exemplos de código devem ser válidos
- Contratos de API são verificados contra implementação real
- Documentação não é "aspiracional" - é realidade

#### Documentação Minimalista
- Documente o necessário, não tudo
- Foque no "porquê", não no "como" (o código mostra o como)
- Se algo é óbvio pelo código, não documente

## Consequências

### Positivas
- **Conhecimento preservado**: Decisões de 2026 serão compreendidas em 2030
- **Onboarding rápido**: Novos devs entendem o projeto lendo docs/
- **Decisões conscientes**: O ato de escrever força pensar melhor
- **Histórico rastreável**: Git mostra quando e por que cada decisão foi tomada
- **Sem documentação fantasmas**: Se está no repo, é realidade ou foi realidade
- **Autonomia**: Devs não precisam perguntar "por que fizemos X?" - está documentado

### Negativas
- **Overhead inicial**: Escrever ADRs demanda tempo
- **Manutenção**: Documentação pode ficar desatualizada se não for disciplina
- **Curva de aprendizado**: Time precisa aprender a estrutura e template
- **Volume**: Projeto grande pode gerar muitos ADRs (mas isso é bom!)
- **Decisões pequenas**: Nem toda mudança precisa de ADR (onde traçar a linha?)

## Alternativas Consideradas

### Wiki ou Confluence
- **Descrição**: Usar plataforma externa de documentação
- **Por que foi rejeitada**: Separação do código, requer acesso especial, não é versionado com Git, tende a ficar obsoleto

### READMEs espalhados
- **Descrição**: Documentar apenas nos READMEs de cada pasta
- **Por que foi rejeitada**: Difícil encontrar decisões específicas, sem estrutura padronizada, não escala bem

### Comentários no código
- **Descrição**: Documentar tudo em comentários
- **Por que foi rejeitada**: Não escala para decisões arquiteturais, polui código, difícil de navegar

### Documentação gerada automaticamente
- **Descrição**: Gerar docs a partir de código (OpenAPI, JSDoc, etc.)
- **Por que foi rejeitada**: Boa para "como" mas ruim para "porquê", não captura decisões de negócio

## Notas de Implementação

### Quando Criar um ADR
Crie um ADR quando:
- Escolher uma tecnologia/framework
- Definir uma estratégia de armazenamento
- Mudar significativamente a arquitetura
- Rejeitar uma alternativa importante
- Definir convenções de código significativas

NÃO crie ADR para:
- Bugs específicos
- Refatorações internas sem impacto arquitetural
- Estilos de código triviais

### Ciclo de Vida de um ADR
1. **Proposto**: Criado, em revisão
2. **Aceito**: Aprovado, passa a ser realidade
3. **Deprecado**: Substituído por decisão nova (mantido para histórico)
4. **Rejeitado**: Nunca implementado (mantido para histórico)

### Template ADR
Usar `docs/templates/template-adr.md` como base.

Númeração sequencial: ADR-000, ADR-001, etc.

### Documentação Rust
- Usar `///` para documentar funções e structs
- Gerar com `cargo doc --no-deps --open`
- Referenciar no contrato de API quando relevante

### Revisão de Documentação
Code reviews devem verificar:
- [ ] ADR criado para decisões arquiteturais?
- [ ] Contrato de API atualizado para mudanças de interface?
- [ ] Spec atualizada se mudança afeta requisitos?
- [ ] Links entre documentos funcionam?

## Data
2026-02-12

## Participantes
- Miguel Martins
