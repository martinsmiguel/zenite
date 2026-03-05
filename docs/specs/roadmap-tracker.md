# Roadmap e Tracker - Zenite

Planejamento e desenvolvimento do MVP do Zenite.

## Como Usar

1. Leia as 5 fases e entenda a sequencialidade
2. Acompanhe progresso (Progresso: X/19 issues)
3. Consulte ADRs (001-004 e 005) em `../adrs/`

## Estrutura

**1. Base do Sistema**: Schema, locking, CRUD, regras
**2. Funcionalidades Essenciais**: Dashboard, lançamento, sistema de arquivos
**3. Capacidade de Acesso**: Extrato, filtros, status, participantes, histórico
**4. Preservação de Dados**: Backup, reconciliação, CSV/JSON, plano de preservação
**5. Disponibilização**: Otimização, documentação, release

## Dados

CSV/JSON garantem dados independentes do software.

## Evolução

Melhorias de prevenção, ampliação CRUD, ferramentas de manutenção.

---

## Progresso: 1/19 issues

---

## 1. Base do Sistema

**Issue-001**: Schema do banco

Contexto: Define a estrutura do banco com tabelas para transações, participantes e auditoria usando tipos centavos.

Para estar pronto: Tabelas criadas com tipos centavos, foreign keys e índices. Não vai fazer: configurar regras de negócio ou validações.

Status: CONCLUIDO

**Issue-002**: File locking

Contexto: Previne que dois usuários editem o mesmo arquivo simultaneamente.

Para estar pronto: Sistema de bloqueio detecta locks ativos e libera locks zumbis. Não vai fazer: sincronização real entre dispositivos.

Status: PENDING

**Issue-003**: CRUD transações

Contexto: Permite criar, ler, atualizar e deletar transações com validações.

Para estar pronto: Operações CRUD completas com validações funcionando. Não vai fazer: filtros avançados ou status automático.

Status: PENDING

**Issue-004**: Regras de negócio

Contexto: Valida dados, calcula status automaticamente e garante integridade.

Para estar pronto: Validações centavos, datas e participantes; status automático. Não vai fazer: cálculos complexos de juros.

Status: PENDING

---

## 2. Funcionalidades Essenciais

**Issue-005**: Dashboard

Contexto: Mostra resumo financeiro para o usuário entender o estado atual do sistema.

Para estar pronto: Saldo roll-over, próximos vencimentos e resumo por participante. Não vai fazer: previsões futuras ou métricas complexas.

Status: PENDING

**Issue-006**: Formulário lançamento

Contexto: Permite entrada de dados com validação e formatação.

Para estar pronto: Formulário completo com validação em tempo real. Não vai fazer: filtros ou exportação.

Status: PENDING

**Issue-007**: Gerador parcelas

Contexto: Cria múltiplas linhas automaticamente baseadas em entrada e número de parcelas.

Para estar pronto: Criação automática de linhas com vencimentos mensais. Não vai fazer: cálculo de juros ou parcelas especiais.

Status: PENDING

**Issue-008**: Sistema de arquivos

Contexto: Permite abrir e salvar arquivos .planner com detecção de existência.

Para estar pronto: Abrir, salvar e detectar arquivos existentes. Não vai fazer: sincronização ou backup automático.

Status: PENDING

---

## 3. Capacidade de Acesso

**Issue-009**: Extrato e filtros

Contexto: Lista transações permitindo encontrar informações específicas.

Para estar pronto: Tabela com filtros (participante, categoria, status, período) e busca. Não vai fazer: filtros complexos ou agrupamentos.

Status: PENDING

**Issue-010**: Status transações

Contexto: Classifica transações como PAGO, PENDENTE ou ATRASADO.

Para estar pronto: Cálculo automático baseado em datas. Não vai fazer: marcação manual ou cálculo de juros.

Status: PENDING

**Issue-011**: Participantes

Contexto: Permite configurar e gerenciar participantes do sistema.

Para estar pronto: Adicionar, remover e selecionar participantes. Não vai fazer: complexidade de multi-organizações.

Status: PENDING

**Issue-012**: Histórico

Contexto: Rastreia quem alterou o quê e quando.

Para estar pronto: Auditoria de alterações com timestamps. Não vai fazer: diffs detalhados ou versionamento.

Status: PENDING

---

## 4. Preservação de Dados

**Issue-013**: Backup automático

Contexto: Preserva dados em caso de corrupção ou erro.

Para estar pronto: Cópia periódica com histórico e restore. Não vai fazer: sincronização cloud ou restore manual.

Status: PENDING

**Issue-014**: Reconciliação

Contexto: Permite revisão conjunta entre usuários.

Para estar pronto: Interface para comparar e marcar reconciliação. Não vai fazer: conflito resolution ou merge automático.

Status: PENDING

**Issue-015**: Import/export CSV/JSON

Contexto: Permite preservação de dados caso o Zenite seja descontinuado. CSV/JSON são formatos abertos, independentes do software.

Para estar pronto: Exportação CSV/JSON e importação de backups. Não vai fazer: validação de schemas ou parsing de outros formatos.

Status: PENDING

**Issue-016**: Plano de preservação de dados

Contexto: Garantir que dados não sejam prisioneiros do software quando ele for descontinuado. Documentar como acessar .planner com ferramentas externas.

Para estar pronto: Documentar como abrir .planner com DB Browser, converter para CSV/JSON e instruções de backup. Não vai fazer: manter suporte técnico indefinido.

Status: PENDING

---

## 5. Disponibilização

**Issue-017**: Otimização

Contexto: Melhora performance do sistema.

Para estar pronto: Queries eficientes e cache básico. Não vai fazer: otimização de interfaces ou pré-cálculos.

Status: PENDING

**Issue-018**: Documentação

Contexto: Guia para novos usuários.

Para estar pronto: Tutorial simples com exemplos. Não vai fazer: wiki completa ou suporte.

Status: PENDING

**Issue-019**: Release

Contexto: Produção do sistema.

Para estar pronto: Build funcional para Windows, macOS, Linux. Não vai fazer: auto-update ou analytics.

Status: PENDING
