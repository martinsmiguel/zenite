# ADR-003: Armazenamento de Valores Monetários em Centavos como Inteiros

## Status
Aceito

## Contexto
Em sistemas financeiros, a precisão monetária é crítica. O uso de tipos de ponto flutuante (f32/f64 em Rust, number em JavaScript) introduz erros de arredondamento devido à representação binária de números decimais.

Exemplo do problema:
- 0.1 + 0.2 em ponto flutuante = 0.30000000000000004
- Em valores monetários, isso causa discrepâncias nos cálculos de saldo

Precisávamos de uma estratégia que garantisse precisão absoluta em todos os cálculos financeiros do sistema.

## Decisão
Todos os valores monetários serão armazenados como inteiros (INTEGER) representando centavos.

### Especificação Técnica
- **Rust**: Tipo `i64` para armazenamento
- **SQLite**: Tipo `INTEGER` 
- **TypeScript**: Tipo `number` (mas sempre em centavos, nunca em reais)

### Regras de Conversão
1. **Armazenamento**: Valor em reais * 100 = valor em centavos
   - Exemplo: R$ 1.500,00 -> 150000
   - Exemplo: R$ 0,50 -> 50
   - Exemplo: R$ 0,01 -> 1

2. **Exibição**: Valor em centavos / 100 = valor em reais
   - Formatação com separadores de milhar e decimal localizados

3. **Sinais**: Despesas são negativas, receitas são positivas
   - Exemplo: Despesa de R$ 100,00 -> -10000
   - Exemplo: Receita de R$ 100,00 -> 10000

## Consequências

### Positivas
- **Precisão absoluta**: Zero erros de arredondamento em cálculos
- **Performance**: Operações com inteiros são mais rápidas que ponto flutuante
- **Simplicidade**: Sem necessidade de bibliotecas de decimal arbitrário
- **Portabilidade**: Inteiros são suportados universalmente em todos os bancos de dados e linguagens
- **Determinismo**: Comportamento idêntico em todas as plataformas

### Negativas
- **Overhead de conversão**: Necessidade de converter em todos os pontos de entrada e saída
- **Risco de erro humano**: Desenvolvedor pode esquecer de converter e tratar centavos como reais
- **Limite de valor**: i64 suporta até aproximadamente R$ 92 quadrilhões (suficiente para uso pessoal/familiar)
- **Interface com APIs externas**: Se futuramente houver integração, será necessário converter

## Alternativas Consideradas

### DECIMAL/NUMERIC do SQLite
- **Descrição**: Usar tipo DECIMAL(10,2) ou similar
- **Por que foi rejeitado**: SQLite armazena DECIMAL como REAL internamente, mantendo o problema de ponto flutuante. Além disso, menos performático.

### Bibliotecas de Decimal Arbitrário (rust-decimal, bigdecimal)
- **Descrição**: Usar crates especializadas para aritmética decimal
- **Por que foi rejeitado**: Adicionam complexidade e dependências externas desnecessárias para um problema que inteiros resolvem elegantemente

### String com parsing
- **Descrição**: Armazenar como string e parsear quando necessário
- **Por que foi rejeitado**: Perda de performance, complexidade desnecessária, sem benefícios claros sobre inteiros

## Notas de Implementação

### Camada Rust
```rust
// Tipo para valores monetários
pub type Centavos = i64;

// Funções de conversão
pub fn reais_para_centavos(reais: f64) -> Result<Centavos, String> {
    // Validação e conversão
}

pub fn centavos_para_reais(centavos: Centavos) -> f64 {
    centavos as f64 / 100.0
}
```

### Validações
- Rejeitar valores que não sejam múltiplos de 1 centavo
- Validar ranges (evitar overflow)
- Rejeitar NaN e infinitos na entrada

### Interface TypeScript
- Criar tipo `type Centavos = number`
- Funções utilitárias de formatação
- Validação em formulários

## Data
2026-02-12

## Participantes
- Miguel Martins
