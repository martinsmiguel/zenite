# Spec: Sistema de Gestão Financeira Colaborativa

## Versão
1.0

## Data
2026-02-12

## Status
Aceito

---

## 1. Visão e Hipótese

### Hipótese Central
Acreditamos que um sistema de gestão financeira baseado em caixa contínuo com registro consensual resolverá conflitos financeiros interpessoais e proporcionará transparência total entre participantes de um grupo (casais, famílias, pequenas empresas).

### Definição
Sistema desktop offline para coordenação financeira colaborativa, operando como "livro-caixa da verdade" compartilhado. Não se conecta a APIs bancárias nem valida dados externos - a "verdade" é o que o grupo registra conjuntamente.

---

## 2. Princípios Fundamentais

### 2.1 Fluxo de Caixa Contínuo (Rollover)
O saldo não zera no fim do mês. Superávits e déficits são transportados, espelhando a realidade financeira acumulada.

### 2.2 Registro Consensual
A verdade do sistema é o que o grupo registra conjuntamente. Serve como ferramenta de mediação para evitar conflitos sobre responsabilidades de pagamento.

### 2.3 Offline e Portátil
Todo o estado reside em um único arquivo `.planner` (SQLite), garantindo:
- Soberania total dos dados
- Backup simples (copiar arquivo)
- Funcionamento sem internet
- Portabilidade entre dispositivos

### 2.4 Acesso Serializado
Apenas um usuário edita por vez (file locking), incentivando rituais de revisão síncrona entre os participantes.

---

## 3. O Que o Sistema Não É

- Não é contabilidade por regime de competência stricto sensu (não calcula depreciação de ativos)
- Não é ferramenta de execução de pagamentos (não transfere dinheiro)
- Não é sistema multiusuário em tempo real com conflito de edição
- Não se conecta a instituições financeiras externas

---

## 4. Critérios de Aceitação

### Funcionais
1. Deve calcular saldo rollover corretamente entre períodos
2. Deve prevenir erros de arredondamento (precisão de centavos)
3. Deve detectar e bloquear dupla abertura do mesmo arquivo na mesma máquina, liberando locks zumbis de sessões encerradas de forma abrupta
4. Deve suportar múltiplos participantes configuráveis por instância
5. Deve calcular status de transações automaticamente (Pago/Pendente/Atrasado)

### Não-Funcionais

1. **Integridade transacional sob falha** — nenhuma escrita parcial deve corromper o `.planner` em caso de crash ou queda de energia.
   > O arquivo é a única fonte da verdade sem redundância (seção 2.3). Sem WAL mode e transações explícitas, uma interrupção durante escrita deixa o banco em estado inconsistente irrecuperável. Ver: [ADR-002](../adrs/002-file-based-serializado.md), risco "corrupção do SQLite" (seção 10).

2. **Portabilidade do arquivo entre plataformas suportadas** — um `.planner` criado em qualquer OS suportado deve abrir corretamente nos demais sem intervenção do usuário.
   > O arquivo trafega entre máquinas de participantes diferentes via pendrive, e-mail ou cloud sync. Diferenças de encoding, path separator ou versão de schema entre plataformas podem silenciosamente corromper dados se não tratadas. Ver: restrição multiplataforma (seção 5), migrações automáticas (seção 11).

3. **Rastreabilidade de autoria** — toda escrita deve registrar o participante responsável de forma verificável antes de persistir.
   > A identificação por PIN é o único mecanismo de auditoria do sistema (seção 6). Sem sessão ativa validada antes da escrita, o histórico de "quem fez o quê" perde integridade e o sistema perde sua função de mediação (seção 2.2).

---

## 5. Restrições de Design

1. **Offline-first**: Sem dependência de conectividade
2. **Single-file**: Todo estado em um arquivo SQLite
3. **Multiplataforma**: Windows, macOS, Linux
4. **Sem APIs externas**: Não integra com bancos ou serviços financeiros
5. **Transparência total**: Dados não criptografados (princípio de confiança do grupo)

---

## 6. Participantes do Sistema

### Configuração por Instância
Cada arquivo `.planner` define seus próprios participantes. O sistema pré-registra três perfis base configuráveis:

- **P1**: Participante 1 (configurável)
- **P2**: Participante 2 (configurável)
- **Comum**: Gastos compartilhados (inclui despesas da residência, pets, etc.)

### Autenticação
Login simples (Identificação + PIN) apenas para fins de auditoria (log de quem fez o quê), não para criptografia granular.

---

## 7. Modelo Conceitual de Dados

### Entidade Principal: Transação
Representa um movimento financeiro no sistema.

**Atributos Essenciais:**
- Identificador único
- Participante responsável (P1, P2 ou Comum)
- Data de emissão (quando o compromisso nasceu)
- Descrição do item/serviço
- Categoria (Moradia, Saúde, Educação, Dívidas, Alimentação, etc.)
- Fornecedor/Origem
- Data de vencimento (quando deveria ser pago)
- Data de pagamento (quando foi pago - pode ser nulo)
- Valor acordado (sem juros)
- Valor pago (real, pode incluir juros)
- Parcelamento (atual/total)
- Forma de pagamento
- Status (calculado automaticamente)
- Observações livres

### Regras de Negócio Fundamentais

#### Regra dos Centavos
Todos os valores armazenados como inteiros representando centavos para garantir precisão monetária absoluta.

Exemplo: R$ 1.500,00 armazenado como 150000.

#### Trilogia de Datas
1. **Emissão**: Quando o gasto nasceu (competência)
2. **Vencimento**: Quando o dinheiro deveria sair (planejamento)
3. **Pagamento**: Quando o dinheiro saiu de fato (caixa real)

#### Linha Independente para Parcelas
Cada parcela é uma linha independente no banco. Se houver entrada, ela é a Parcela 1.

Exemplo: Compra com entrada + 3 parcelas = 4 linhas no banco.
Benefício: Facilita projeção de fluxo de caixa futuro.

#### Cálculo de Status
Status determinado logicamente, não manualmente:
- **PAGO**: Data de pagamento preenchida
- **PENDENTE**: Data de pagamento nula E vencimento >= hoje
- **ATRASADO**: Data de pagamento nula E vencimento < hoje

#### Gestão de Juros
Sistema não calcula juros automaticamente. Registra o desvio entre valor acordado e valor pago.

Exemplo: Boleto de R$ 100 pago com atraso por R$ 102.
- Valor acordado: -10000 (centavos)
- Valor pago: -10200 (centavos)
- Análise: Diferença de -200 centavos = R$ 2,00 de juros/multa

---

## 8. Interfaces do Usuário (MVP)

### Dashboard (Visão Macro)
- Saldo atual (soma de valores pagos)
- Próximos vencimentos (top 5 pendentes)
- Resumo por participante (quanto gastou P1 vs P2 vs Comum)
- Indicadores de saúde financeira

### Lançamento (Input)
- Formulário único para transações
- Botão "Gerar Parcelas": usuário informa total de parcelas, sistema cria as linhas calculando vencimentos mês a mês
- Validação em tempo real

### Extrato e Filtros
- Lista tabular das transações
- Filtros por: participante, categoria, status, período
- Ordenação e busca

### Sincronização/Revisão
- Tela de reconciliação para revisão conjunta
- Histórico de alterações (auditoria)
- Exportação para CSV/JSON

---

## 9. Métricas de Sucesso

1. **Adoção**: 100% das transações do grupo registradas no sistema após 30 dias
2. **Conflitos**: Redução de 90% em discussões sobre "quem pagou o quê"
3. **Tempo**: Reconciliação semanal inferior a 5 minutos
4. **Precisão**: Zero erros de arredondamento em cálculos financeiros
5. **Satisfação**: NPS superior a 8 entre os participantes

---

## 10. Riscos e Mitigações

| Risco | Probabilidade | Impacto | Mitigação |
|-------|--------------|---------|-----------|
| Dupla abertura acidental do arquivo na mesma máquina | Baixa | Médio | Detecção de lock ativo com mensagem clara; liberação automática de locks zumbis |
| Corrupção do arquivo SQLite | Baixa | Alto | Backups automáticos, exportação periódica |
| Dificuldade de adoção | Média | Médio | Interface simples, onboarding guiado |
| Perda do arquivo | Baixa | Alto | Backup manual fácil (apenas copiar arquivo) |

---

## 11. Notas de Implementação

### Tecnologia
- Backend: SQLite (zero-server, ACID compliant)
- Frontend: Tauri 2.0 (Rust + TypeScript)
- Formato: Arquivo `.planner` (SQLite com extensão customizada)

### Estratégia de Versionamento
- Versionamento semântico para o aplicativo
- Migrações automáticas de schema SQLite entre versões
- Compatibilidade retroativa garantida para arquivos .planner

---

## 12. Referências

- [ADR-000: Estratégia de Documentação como Código](../adrs/000-estrategia-documentacao.md)
- [ADR-001: Escolha do Framework Tauri 2.0](../adrs/001-escolha-do-framework.md)
- [ADR-002: File-based com Acesso Serializado vs Cliente-Servidor](../adrs/002-file-based-serializado.md)
- [ADR-003: Armazenamento em Centavos como Inteiros](../adrs/003-armazenamento-centavos.md)
- [ADR-004: Linha Independente para Parcelas](../adrs/004-linha-independente-parcelas.md)

---

**Participantes da Definição desta Spec:**
- Miguel Martins

**Revisões:**
- 1.0 (2026-02-12): Versão inicial
- 1.1 (2026-02-19): Status atualizado para Aceito, participantes preenchidos
- 1.2 (2026-02-19): Critérios de aceitação revisados — NFRs fundamentados na arquitetura; F#3 e risco de edição ajustados ao caso de uso sequencial real
