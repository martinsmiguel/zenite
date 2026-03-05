# Handbook: Arquitetura de Camadas do Zenite

> **Decisão de origem:** [ADR-007](../adrs/007-arquitetura-quatro-camadas-essenciais.md)
> **Última revisão:** 2026-02-24

Guia prático de como organizar código no projeto. O objetivo é manter a estrutura mínima necessária — nem tão simples que polui, nem tão elaborada que gera custo de manutenção.

---

## As Quatro Camadas

Nota: Diretriz estrutural. Arquivos serão criados sob demanda; nomes e organização podem ajustar-se conforme necessidade durante a implementação.

```
src-tauri/src/
├── domain/
│   ├── entities/
│   │   ├── transacao.rs          # struct Transacao + regras de negócio
│   │   └── parcela.rs            # struct Parcela
│   ├── services/                 # Cálculos financeiros puros
│   │   └── calculo_juros.rs
│   └── repositories/
│       └── transacao.rs          # trait TransacaoRepository (~5 linhas, sem impl)
│
├── application/                  # Vazio no dia 0; criado sob demanda
│   └── criar_transacao.rs        # fn criar_transacao_parcelada() quando necessário
│
├── infrastructure/
│   └── sqlite/
│       ├── transacao_repo.rs     # impl TransacaoRepository para SQLite
│       └── migrations.rs         # Setup do schema
│
└── interface/
    └── tauri/
        ├── commands.rs           # #[tauri::command] — adaptadores mínimos
        ├── state.rs              # AppState, injeção de dependência
        └── mappers.rs            # fn transacao_to_response() e afins
```

---

## Regra de Ouro: Justifique a Existência

Antes de criar qualquer arquivo, módulo ou camada, responda: **qual dos três critérios ele atende?**

| Critério | Descrição | Quando usar |
|----------|-----------|-------------|
| **Elimina risco** | Habilita testes sem dependências externas ou isola uma decisão que pode mudar | Traits de repositório, abstrações de configuração |
| **Habilita reuso** | Código usado em mais de um contexto (ex: Tauri e CLI futuro) | Lógica de domain, application functions |
| **Esconde complexidade** | Orquestração de múltiplos passos que poluiria outra camada | Funções de application com 3+ passos |

**Se nenhum se aplica, não crie.**

---

## Domain: O Núcleo

**O que entra:** Entidades, regras de cálculo, validações, traits de repositório.

**O que NÃO entra:** Acesso a banco, lógica de orquestração, detalhes de framework.

**Testável com:** Lógica pura — sem mocks de infraestrutura.

**Exemplo:**

```rust
// domain/entities/transacao.rs
pub struct Transacao {
    pub descricao: String,
    pub valor_centavos: i64,
    pub parcela_atual: u32,
    pub total_parcelas: u32,
}

impl Transacao {
    pub fn gerar_parcelas(&self) -> Vec<Parcela> {
        // Regra pura: calcula vencimentos, valores
    }
    
    pub fn validar(&self) -> Result<(), Erro> {
        // Regra pura: invariantes de domínio
    }
}

// domain/repositories.rs — trait apenas, sem implementação
pub trait TransacaoRepository {
    fn create(&self, t: &Transacao) -> Result<(), Erro>;
    fn exists(&self, id: Uuid) -> Result<bool, Erro>;
}
```

**Traits (interfaces em Rust):** Contrato que define "o que" sem dizer "como". Como uma interface Java ou C#: especifica métodos, mas quem implementa define o comportamento. Permite trocar SQLite por mock em testes sem mudar quem usa.

**Por que traits aqui:** Permitem testar domain sem SQLite real. A implementação concreta fica em `infrastructure/`.

---

## Application: A Ponte

**Quando criar:** Apenas quando um caso de uso tem múltiplos passos que poluiriam outra camada.

**Quando NÃO criar:** Casos de uso de 1 passo (ex: deletar transação) → vá direto de `interface` para `infrastructure`.

**Testável com:** `mockall` para as dependências de infrastructure.

**Exemplo:**

```rust
// application/criar_transacao.rs
pub fn criar_transacao_parcelada(
    repo: &impl TransacaoRepository,
    dados: NovaTransacao,
) -> Result<Vec<Transacao>, Erro> {
    // 1. Validar (domain)
    let transacao = dados.validar()?;
    
    // 2. Verificar existência (infra via trait)
    if repo.exists(transacao.id)? {
        return Err(Erro::JaExiste);
    }
    
    // 3. Gerar parcelas (domain)
    let parcelas = transacao.gerar_parcelas();
    
    // 4. Persistir (infra via trait)
    for p in &parcelas {
        repo.create(p)?;
    }
    
    Ok(parcelas)
}
```

---

## Infrastructure: O Concreto

**O que entra:** Implementações de traits, acesso a SQLite, filesystem, serviços externos.

**O que NÃO entra:** Regras de negócio, lógica de orquestração.

**Testável com:** SQLite em memória (`:memory:`) apenas para testes de infra isolados.

**Exemplo:**

```rust
// infrastructure/sqlite/transacao_repo.rs
pub struct SqliteTransacaoRepository {
    conn: Connection,
}

impl TransacaoRepository for SqliteTransacaoRepository {
    fn create(&self, t: &Transacao) -> Result<(), Erro> {
        // SQL, parâmetros, execução
    }
    
    fn exists(&self, id: Uuid) -> Result<bool, Erro> {
        // Query de existência
    }
}
```

---

## Interface: O Adaptador

**O que entra:** Commands Tauri, mapeamento de tipos, injeção de estado.

**O que NÃO entra:** Lógica de negócio, orquestração complexa.

**Regra dos commands:** Idealmente 1 linha — adapta request, chama application/infrastructure, adapta response.

**Exemplo:**

```rust
// interface/tauri/commands.rs
#[tauri::command]
fn criar_transacao(
    state: State<'_, SqliteTransacaoRepository>,
    request: CriarTransacaoRequest,
) -> Result<Vec<TransacaoResponse>, Erro> {
    let transacoes = application::criar_transacao_parcelada(&*state, request.into())?;
    Ok(transacoes.into_iter().map(into_response).collect())
}

// Mappers são funções puras, não structs
fn into_response(t: Transacao) -> TransacaoResponse {
    TransacaoResponse {
        id: t.id.to_string(),
        valor_reais: centavos_para_reais(t.valor_centavos),
        // ...
    }
}
```

---

## Decisões Práticas

### Funções > Structs em `application/`

```rust
// ADOTADO — menos boilerplate, mesmo valor
pub fn criar_transacao_parcelada(
    repo: &impl TransacaoRepository,
    input: CriarTransacaoInput,
) -> Result<Transacao, Erro> { ... }

// REJEITADO — boilerplate sem valor
pub struct CriarTransacaoParcelada {
    repo: Arc<dyn TransacaoRepository>,
}
impl CriarTransacaoParcelada {
    pub fn execute(&self, input: CriarTransacaoInput) -> Result<Transacao, Erro> { ... }
}
```

### Mappers são funções puras em `interface/`

```rust
// ADOTADO
pub fn transacao_to_response(t: &Transacao) -> TransacaoResponse { ... }

// REJEITADO — struct sem estado não agrega nada
pub struct TransacaoMapper;
impl TransacaoMapper {
    pub fn to_response(t: &Transacao) -> TransacaoResponse { ... }
}
```

### Traits são para testes, não para "futuro"

Crie `TransacaoRepository` mesmo com só 1 implementação (SQLite) porque permite testar domain sem banco. **Não** antecipe "talvez teremos PostgreSQL". Crie porque queremos testar sem SQLite *hoje*.

### Application começa vazia

Não crie `application/` no dia 0. Preencha sob demanda, quando surgir orquestração real.

---

## Exemplo Concreto: Criar Transação Parcelada

**Versão rejeitada (abstração demais):** 6 arquivos, ~200 linhas com services, DTOs, mappers, structs extras.

**Versão adotada (essencial):**

```
domain/entities/transacao.rs       → struct Transacao + fn gerar_parcelas()
domain/repositories/transacao.rs   → trait TransacaoRepository (~5 linhas)
application/transacao.rs           → fn criar_transacao_parcelada() (~30 linhas)
infrastructure/sqlite/transacao.rs → impl TransacaoRepository
interface/tauri/commands.rs        → #[tauri::command] de 1 linha que chama application
```

~50 linhas vs ~200. Mesma testabilidade.

---

## Onde Colocar Cada Coisa

| Cenário | Onde colocar | Por quê |
|---------|--------------|---------|
| Validar dados de entrada | Domain | Regra pura, reutilizável |
| Calcular juros compostos | Domain | Complexidade matemática |
| Criar transação (1 parcela) | Interface → Infrastructure | 1 passo, sem orquestração |
| Criar transação parcelada | Application função | 4 passos: valida + verifica + gera + persiste |
| Exportar CSV | Interface → Infrastructure | 1 passo, não é regra de negócio |
| Backup automático | Infrastructure service | Preservação, não lógica de domínio |

---

## Sinais de Alerta

| Sinal | Problema provável | Ação |
|-------|------------------|------|
| Command com 20+ linhas | Lógica de negócio vazando para interface | Mover para `application` ou `domain` |
| Domain importando `rusqlite` | Infrastructure vazando para domain | Criar trait + mover implementação |
| `application` com 10+ arquivos | Over-engineering antecipado | Revisar se cada arquivo atende aos 3 critérios |
| Struct com apenas `fn execute()` | Função seria suficiente | Simplificar para `fn` |

---

## Revisão e Evolução

- **3 meses:** `application/` ficou inchada ou manteve-se fina?
- **6 meses:** traits realmente facilitaram testes ou foram overkill?

Ajustar conforme aprendizado. Este guia é evolutivo, não dogma.

Decisão final registrada em atualização do [ADR-007](../adrs/007-arquitetura-quatro-camadas-essenciais.md).
