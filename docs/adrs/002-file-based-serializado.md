# ADR-002: File-based com Acesso Serializado vs Cliente-Servidor

## Status
Aceito

## Contexto
O sistema precisa ser 100% offline e portátil. Precisávamos decidir a arquitetura de persistência:
1. Banco cliente-servidor (PostgreSQL, MySQL) rodando localmente
2. Banco embedded (SQLite) em modo file-based
3. Outras opções (JSON, XML, etc.)

Requisitos críticos:
- Funcionamento totalmente offline
- Backup trivial (apenas copiar um arquivo)
- Portabilidade entre dispositivos
- Acesso por múltiplos usuários (não simultâneo)
- Simplicidade de instalação e configuração

## Decisão
Adotar SQLite em modo file-based com mecanismo de file locking para acesso serializado.

### Especificação Técnica
- **Formato**: Arquivo `.planner` (extensão customizada, mas é SQLite)
- **Banco**: SQLite 3 (embedded, zero-config)
- **Acesso**: File locking nativo do sistema operacional
- **Portabilidade**: Arquivo pode ser copiado, movido, sincronizado via cloud (Drive, Dropbox, etc.)

### Regras de Acesso
1. **Leitura**: Múltiplos usuários podem ler simultaneamente
2. **Escrita**: Apenas um usuário pode editar por vez
3. **Locking**: SQLite WAL mode com busy timeout
4. **Detecção**: Interface mostra claramente quando arquivo está bloqueado

## Consequências

### Positivas
- **Simplicidade extrema**: Nenhum servidor para instalar ou configurar
- **Backup trivial**: Copiar arquivo = backup completo
- **Portabilidade total**: Funciona em qualquer dispositivo com o app instalado
- **Sincronização via nuvem**: Pode colocar arquivo no Dropbox/Drive e acessar de múltiplos dispositivos
- **Performance**: Acesso direto ao disco, sem overhead de rede
- **Soberania de dados**: Usuário tem controle total sobre seus dados
- **Tamanho**: SQLite é leve (algumas centenas de KB)

### Negativas
- **Sem acesso simultâneo**: Dois usuários não podem editar ao mesmo tempo
- **Risco de conflito**: Se dois usuários copiarem o arquivo, editarem separadamente e tentarem mesclar
- **Single point of failure**: Corrupção do arquivo = perda total (mitigado por backups)
- **Escalabilidade limitada**: Não serve para cenários multiusuário real
- **File locking inconsistente**: Diferentes sistemas operacionais tratam locks de forma diferente

## Alternativas Consideradas

### SQLite em modo cliente-servidor (sqlited)
- **Descrição**: SQLite com servidor TCP/IP local
- **Por que foi rejeitado**: Adiciona complexidade de configuração, porta, firewall. Perde a simplicidade do "apenas copie o arquivo".

### PostgreSQL/MySQL local
- **Descrição**: Instalar banco relacional completo na máquina do usuário
- **Por que foi rejeitado**: Overkill para aplicação pessoal, complexidade de instalação e manutenção, não é portátil.

### Arquivos JSON/XML
- **Descrição**: Dados em formato texto estruturado
- **Por que foi rejeitado**: Sem transações ACID, sem indexação, performance ruim com grande volume, risco de corrupção maior.

### Sistema de arquivos com múltiplos arquivos CSV
- **Descrição**: Uma pasta com arquivos CSV para cada tabela
- **Por que foi rejeitado**: Sem integridade referencial, sem transações, dificil de manter consistência.

## Notas de Implementação

### SQLite Configuration
```rust
// Configuração recomendada para SQLite
let conn = Connection::open("arquivo.planner")?;
conn.execute_batch("
    PRAGMA journal_mode = WAL;
    PRAGMA foreign_keys = ON;
    PRAGMA busy_timeout = 5000;  -- 5 segundos esperando lock
")?;
```

### File Locking
- Usar `flock` (Unix) ou `LockFile` (Windows) quando abrir para escrita
- Interface deve detectar quando arquivo está em uso e informar usuário
- Timeout de 30 segundos antes de considerar lock "zumbi"

### Ritual de Sincronização
Como não há edição simultânea, o sistema incentiva:
1. Usuário A abre arquivo e edita
2. Usuário A fecha arquivo
3. Arquivo sincroniza via nuvem (se configurado)
4. Usuário B abre arquivo atualizado
5. Repetir

### Backup Automático
- Criar backup automático a cada N minutos durante edição
- Manter últimos 10 backups com timestamp
- Permitir restauração fácil de backup

## Data
2026-02-12

## Participantes
- Miguel Martins
