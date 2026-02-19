# ADR-004: Linha Independente para Parcelas

## Status
Aceito

## Contexto
O sistema precisa lidar com compras parceladas de forma consistente, incluindo:
- Compras com entrada (down payment) + parcelas
- Compras sem entrada
- Diferentes datas de vencimento para cada parcela
- Cálculo de fluxo de caixa futuro

A questão central: como representar uma compra parcelada no banco de dados de forma que:
1. Permita rastreamento individual de cada parcela
2. Facilite o cálculo de fluxo de caixa futuro
3. Mantenha consistência na modelagem de dados
4. Seja simples de implementar e entender

## Decisão
Cada parcela será uma linha independente na tabela de transações. Se houver entrada, ela é considerada a Parcela 1.

### Regra Formal
- Nunca usar parcela 0 (0/10)
- Entrada = Parcela 1
- Sistema sempre usa base 1 (primeira parcela é 1, não 0)
- Total de parcelas inclui a entrada

### Exemplo Prático
Compra de geladeira com:
- Entrada: R$ 500,00
- + 3 parcelas de R$ 300,00

**Representação no banco:**
```
Linha 1: Parcela 1 de 4, Valor: 50000 (entrada), Vencimento: hoje
Linha 2: Parcela 2 de 4, Valor: 30000, Vencimento: mês que vem
Linha 3: Parcela 3 de 4, Valor: 30000, Vencimento: mês + 2
Linha 4: Parcela 4 de 4, Valor: 30000, Vencimento: mês + 3
```

## Consequências

### Positivas
- **Fluxo de caixa preciso**: Cada parcela tem sua própria data de vencimento, permitindo projeções exatas
- **Flexibilidade**: Parcelas podem ter valores diferentes (ex: entrada diferenciada)
- **Rastreamento individual**: É possível pagar parcelas fora de ordem ou antecipar específicas
- **Simplicidade mental**: Não há distinção especial entre "entrada" e "parcelas"
- **Consultas simples**: SUM WHERE parcela_atual = X funciona diretamente
- **Consistência**: Toda transação segue o mesmo padrão, independente de ter entrada ou não

### Negativas
- **Mais linhas no banco**: Uma compra gera N linhas em vez de 1
- **Complexidade na geração**: Sistema precisa calcular e criar múltiplas linhas
- **Atualização em massa**: Se houver alteração no contrato, pode precisar atualizar várias linhas
- **Deleção**: Cancelar uma compra parcelada requer deletar múltiplas linhas

## Alternativas Consideradas

### Tabela Separada para Parcelas
- **Descrição**: Tabela transacoes + tabela parcelas (relacionamento 1:N)
- **Por que foi rejeitado**: Adiciona complexidade de JOINs para consultas simples. O sistema é pequeno o suficiente para não justificar normalização excessiva.

### Campo JSON para Parcelas
- **Descrição**: Coluna parcelas JSON com array de datas/valores
- **Por que foi rejeitado**: SQLite tem suporte limitado a JSON, consultas ficam complexas, perde-se a capacidade de indexar datas de vencimento individualmente.

### Modelo Híbrido (uma linha com array de vencimentos)
- **Descrição**: Uma linha com campos valor_total, num_parcelas, vencimentos[]
- **Por que foi rejeitado**: Dificulta o rastreamento do pagamento de parcelas individuais. O fluxo de caixa futuro fica menos preciso.

## Notas de Implementação

### Geração de Parcelas
```rust
pub fn gerar_parcelas(
    entrada: Option<Centavos>,
    valor_parcela: Centavos,
    num_parcelas: u32,
    data_primeiro_vencimento: NaiveDate,
) -> Vec<Transacao> {
    let total_parcelas = num_parcelas + if entrada.is_some() { 1 } else { 0 };
    let mut transacoes = Vec::new();
    
    // Gera cada linha com seu próprio vencimento (mês + n)
    // ...
}
```

### Validações
- parcela_atual >= 1
- parcela_atual <= total_parcelas
- total_parcelas >= 1
- Data de vencimento calculada incrementando meses (cuidado com dia 31)

### Interface de Usuário
- Formulário único para compra parcelada
- Campo "tem entrada?" (boolean)
- Se sim: campo valor da entrada
- Campo valor da parcela
- Campo quantidade de parcelas (sem contar entrada)
- Botão "Gerar Parcelas" cria as N+1 linhas automaticamente

## Data
2026-02-12

## Participantes
- Miguel Martins
