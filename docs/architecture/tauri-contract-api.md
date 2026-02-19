# Contrato da API Tauri - Zenite

## Visão Geral
Este documento define o contrato entre o Frontend (TypeScript) e o Backend (Rust) do projeto Zenite. Qualquer mudança nesta API deve ser documentada aqui ANTES da implementação.

## Estrutura de Comunicação

### Commands (Frontend → Rust)
Commands são funções Rust expostas ao frontend via `#[tauri::command]` e invocadas através de `invoke()`.

### Events (Rust → Frontend)
Events são mensagens enviadas do Rust para o frontend usando o sistema de eventos do Tauri.

---

## Comandos Disponíveis

### `greet`
Saudação inicial de exemplo.

**Input:**
```typescript
{
  name: string  // Nome da pessoa a ser saudada
}
```

**Output:**
```typescript
string  // Mensagem de saudação formatada
```

**Exemplo:**
```typescript
import { invoke } from '@tauri-apps/api/core';

const message = await invoke('greet', { name: 'Usuário' });
// Retorna: "Hello, Usuário! You've been greeted from Rust!"
```

**Implementação Rust:**
- **Arquivo**: `src-tauri/src/lib.rs`
- **Função**: `greet(name: &str) -> String`

---

## Tipos Compartilhados

### Convenções de Nomenclatura
- **Rust**: snake_case para funções e variáveis, PascalCase para structs/enums
- **TypeScript**: camelCase para funções e variáveis, PascalCase para interfaces/types

### Serialização
- Rust usa `serde` para serialização JSON
- Todos os tipos devem implementar `Serialize` e `Deserialize`

---

## Eventos (Futuro)

Quando implementarmos comunicação assíncrona do Rust para o frontend:

### Estrutura
```rust
// Rust - Emitindo evento
app.emit("progress_update", ProgressPayload { 
    id: task_id, 
    percent: 50 
});
```

```typescript
// Frontend - Escutando evento
import { listen } from '@tauri-apps/api/event';

const unlisten = await listen('progress_update', (event) => {
  console.log(`Progresso: ${event.payload.percent}%`);
});
```

---

## Versionamento da API

- **Versão atual**: 0.1.0
- **Compatibilidade**: Mantemos compatibilidade retrocompatível em minor versions
- **Breaking changes**: Documentadas em ADRs específicos

## Segurança

### Permissões Necessárias
Toda funcionalidade que requer acesso a recursos do sistema deve ser explicitamente permitida em `tauri.conf.json`:

```json
{
  "permissions": [
    "core:default",
    "opener:default"
  ]
}
```

### Validação de Input
- Todo input do frontend é considerado não confiável
- Validar todos os parâmetros no Rust antes de processar
- Retornar Result<T, String> para erros controlados

---

## Como Adicionar Novos Comandos

1. **Definir contrato**: Atualize este documento com input/output
2. **Implementar em Rust**: Adicione a função com `#[tauri::command]` em `src-tauri/src/lib.rs`
3. **Exportar**: Adicione à lista em `tauri::generate_handler![]`
4. **Documentar**: Adicione comentários rustdoc (`///`) na função
5. **Usar no Frontend**: Importe e chame via `invoke()`
6. **Testar**: Verifique se a serialização funciona corretamente

## Links Úteis
- [Documentação Tauri Commands](https://tauri.app/develop/calling-rust/)
- [Guia de Invoke](https://tauri.app/reference/javascript/api/core/#invoke)
- [Cargo Doc Local](../../src-tauri/target/doc/zenite/index.html)
