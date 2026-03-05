# Diretrizes de Testes - Zenite

Este documento descreve as diretrizes de teste aplicadas ao projeto Zenite, seguindo as definições estabelecidas no [ADR-006: Estratégia de Testes para Rust/Tauri](./adrs/006-estrategia-testes-rust-tauri.md).

## 1. Arquitetura de Testes em Três Camadas

O Zenite utiliza uma abordagem de testes segmentada para garantir velocidade e confiabilidade:

| Camada | Escopo | Ferramentas | Infraestrutura |
| :--- | :--- | :--- | :--- |
| **A: Unidade** | Regras de negócio puras, cálculos, validações e utilitários. | `cargo test --lib` | Nenhuma (Puro) |
| **B: Integração Lógica** | Comandos Tauri e Serviços de Aplicação (orquestração). | `tauri::test`, `mockall` | Mocks/Traits (Zero DB) |
| **C: E2E** | Fluxos completos de ponta a ponta. | WebDriver, `tauri-driver` | SQLite Real (Temp File) |

## 2. Princípio de Zero Infraestrutura (Camada B)

Para testes de comandos e serviços, **não utilizamos banco de dados real (incluindo `:memory:`)**. 
- A persistência deve ser abstraída por **Traits** (Repositórios).
- Nos testes, injetamos **Mocks** gerados pela crate `mockall`.
- Isso garante testes determinísticos e extremamente rápidos.

## 3. Padrão AAA (Arrange-Act-Assert)

Todos os testes devem seguir rigorosamente o padrão AAA:

1.  **Arrange**: Configuração de mocks, injeção de dependências e definição de dados via Fixtures.
2.  **Act**: Execução da função, método ou comando sob teste.
3.  **Assert**: Comparação do resultado integral com o objeto esperado (`expected`) definido na Fixture.

## 4. Gestão de Dados e Helpers

### Helpers Globais e Constantes
Utilize helpers centralizados para remover lógica de apoio (como datas, cálculos recorrentes ou normalizações) do corpo do teste:
- **Redução de Ruído**: Testes ficam visualmente mais enxutos, focados apenas na ação e no resultado.
- **Manutenção Simples**: Mudanças globais (ex: trocar a data base do sistema) são feitas em um único lugar.
- **Exemplo de Data**: O uso de `DATA_BASE` e `add_meses(n)` em cálculos financeiros para manter o determinismo temporal.

### Fixtures e Fonte Única da Verdade
Fixtures devem centralizar tanto a entrada quanto a saída esperada:

```rust
// Exemplo de uso de Fixture e Assert Integral
#[test]
fn test_should_calculate_installments_correctly() {
    // Arrange
    let transacao = TransacaoFixture::parcelada(); // Fixture centraliza a entrada
    let expected = TransacaoFixture::parcelas_esperadas(); // Fixture centraliza o esperado

    // Act
    let result = transacao.gerar_parcelas();

    // Assert
    assert_eq!(result, expected); // Comparação integral rápida e limpa
}
```

## 5. Nomenclatura de Testes

Testes seguem o padrão em inglês: `test_should_<resultado>_when_<condicao>`

Exemplos:
- `test_should_create_transaction_successfully_when_valid_data`
- `test_should_return_error_when_amount_is_zero`
- `test_should_calculate_balance_correctly_when_multiple_entries`

## 6. Localização dos Testes

- **Testes Unitários (Camada A)**: Dentro do próprio módulo em um submódulo `#[cfg(test)] mod tests`.
- **Testes de Integração Lógica (Camada B)**: No diretório `tests/` na raiz do crate `src-tauri`.
- **Testes E2E (Camada C)**: No diretório raiz `e2e-tests/`.

## 7. Checklist de Boas Práticas

- [ ] O teste usa banco de dados real em lógica de negócio? (Se sim, remova e use Mocks).
- [ ] O teste segue o padrão AAA?
- [ ] As datas são fixas através de `test_helpers::DATA_BASE`?
- [ ] O nome do teste segue o padrão `test_should_...`?
- [ ] O `assert_eq!` compara o objeto/resultado integral com uma fixture de `expected`?
- [ ] O teste é rápido o suficiente para rodar em milissegundos?

---

**Data de Criação**: 2026-02-21                               
**Versão**: 1.0     
**Revisores**: Miguel Martins
