# ADR-006: Estratégia de Testes para Rust/Tauri

## Status
Aceito

## Contexto
O Zenite é uma aplicação financeira onde a precisão dos cálculos, a consistência das datas e a integridade das regras de negócio são críticas. Para garantir a evolução segura do software, precisamos de uma estratégia de testes que seja:
1. **Determinística**: Resultados idênticos em qualquer ambiente, sem dependência de estado externo.
2. **Veloz**: Feedback instantâneo para o desenvolvedor (milhares de testes em segundos).
3. **Desacoplada**: Capacidade de testar lógica de negócio sem instanciar infraestrutura (banco de dados ou IPC).
4. **Legível**: Testes que servem como documentação viva do comportamento esperado.

## Decisão
Adotar uma estratégia de testes baseada em **Inversão de Dependência via Traits**, **Isolamento de Infraestrutura** e **Centralização de Dados (Fixtures/Helpers)**. A arquitetura de testes é dividida em três camadas complementares:

### 1. Princípio de Zero Infraestrutura em Testes Lógicos
Para testes de unidade e integração lógica (serviços e comandos), **não utilizamos banco de dados real ou serviços externos**. 

A persistência é abstraída por `Traits` (repositórios). Em ambiente de teste, injetamos mocks (gerados via `mockall`) ou implementações em memória. Isso garante que falhas nos testes indiquem erros de lógica, não problemas de configuração de ambiente ou estado residual em banco de dados.

### 2. Arquitetura em Três Camadas

#### Camada A: Testes de Unidade (Domínio e Utilitários)
- **Foco**: Regras de negócio puras, validações financeiras e helpers.
- **Implementação**: Localizados em módulos `#[cfg(test)]` dentro dos próprios arquivos de código.
- **Justificativa**: Garante que o núcleo da aplicação (cálculo de juros, parcelamento, saldo) esteja blindado.
- **O que não testar**: DTOs anêmicos, constantes e mappers simples (sem lógica condicional).

#### Camada B: Testes de Integração Lógica (Application & Commands)
- **Foco**: Fluxos entre comandos Tauri, serviços de aplicação e domínio.
- **Ferramentas**: `tauri::test` (MockRuntime) e `mockall`.
- **Justificativa**: Valida se o "encanamento" da aplicação (orquestração de serviços) funciona corretamente sem a lentidão de uma UI real ou banco de dados em disco.

#### Camada C: Testes E2E (Fluxos Reais)
- **Foco**: Caminhos felizes e fluxos críticos de ponta a ponta.
- **Ferramentas**: WebDriver com SQLite real em arquivo temporário.
- **Justificativa**: Valida a integração final entre Frontend, Rust e persistência real no SQLite.

### 3. Gestão de Dados: Fixtures e Helpers Globais

Para evitar "números mágicos", inconsistências entre cenários e a repetição de lógica complexa de setup, centralizamos a criação e manipulação de dados:

- **Helpers Globais**: Centralização de constantes e utilitários de apoio (ex: `DATA_BASE` e `add_meses(n)` para consistência temporal). Ao mover a lógica de apoio para helpers, os testes tornam-se visualmente mais enxutos, facilitando a análise do comportamento real e reduzindo o custo de manutenção.
- **Fixtures Tipadas**: Funções que retornam entidades e suas respectivas respostas esperadas. O teste deve comparar o objeto retornado integralmente com o objeto esperado da fixture (`assert_eq!(result, expected)`), garantindo a integridade de todos os campos sem poluir o corpo do teste com detalhes de implementação.

---

### Exemplos de Implementação

#### Helpers de Teste Centralizados
```rust
#[cfg(test)]
pub mod test_helpers {
    use chrono::NaiveDate;

    /// Referencial temporal fixo para garantir determinismo
    pub const DATA_BASE: NaiveDate = match NaiveDate::from_ymd_opt(2026, 2, 24) {
        Some(d) => d,
        None => panic!("Data base inválida"),
    };

    pub fn add_meses(meses: u32) -> NaiveDate {
        DATA_BASE.checked_add_months(chrono::Months::new(meses)).unwrap()
    }
    
    /// Helper para normalização de valores monetários
    pub fn centavos(reais: f64) -> i64 {
        (reais * 100.0).round() as i64
    }
}
```

#### Teste com Mock e Fixture
```rust
#[test]
fn test_should_create_transaction_successfully() {
    // Arrange
    let mut repo = MockTransacaoRepository::new();
    let entity = TransacaoFixture::default();
    let expected = TransacaoFixture::response(&entity);
    
    repo.expect_create()
        .with(eq(entity.clone()))
        .times(1)
        .returning(|e| Ok(e));
        
    let service = TransacaoService::new(repo);
    
    // Act
    let result = service.criar(entity);
    
    // Assert
    assert_eq!(result, Ok(expected));
}
```

## Consequências

### Positivas
- **Confiabilidade**: Testes financeiros tornam-se imunes a variações de fuso horário ou data atual do sistema.
- **Manutenibilidade**: Alterações em regras de negócio ou contratos de API exigem atualizações apenas nas fixtures centralizadas.
- **Escalabilidade**: Testes rodam em paralelo por padrão (sem concorrência de banco de dados), mantendo o pipeline de CI rápido.
- **Design de Código**: A necessidade de testar isoladamente força o uso de injeção de dependência e interfaces limpas.

### Negativas
- **Boilerplate Inicial**: Exige a criação de traits para repositórios e manutenção de arquivos de fixture.
- **Risco de Abstração**: O comportamento do Mock pode divergir de constraints reais do SQLite (ex: chaves estrangeiras). Isso é mitigado pela Camada C (E2E).

## Notas de Implementação

### Padrão AAA (Arrange-Act-Assert)
Todo teste deve seguir visualmente o padrão:
1. **Arrange**: Configuração de mocks, injeção de dados e definição de expectativas.
2. **Act**: Execução da função ou comando sob teste.
3. **Assert**: Comparação direta entre o resultado e a saída esperada (proveniente da fixture).

### Checklist de Revisão
- O teste usa banco de dados real? (Se sim, deve ser movido para E2E ou infra).
- As datas são criadas manualmente ou via `test_helpers`?
- O nome da função descreve claramente o resultado e a condição? (`test_should_X_when_Y`)

## Data
2026-03-01

## Participantes
- Miguel Martins
