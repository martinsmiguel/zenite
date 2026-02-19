# ADR-001: Escolha do Framework Tauri 2.0

## Status
Aceito

## Contexto
Precisávamos de uma solução para desenvolver uma aplicação desktop multiplataforma (Windows, macOS, Linux) com as seguintes características:
- Performance nativa e baixo consumo de recursos
- Tamanho de distribuição pequeno
- Segurança robusta
- Experiência de desenvolvimento moderna (Web tech no frontend)
- Possibilidade de expansão futura para mobile (iOS/Android)

As opções principais consideradas foram Electron, Tauri, Flutter e aplicações nativas (Swift/C#).

## Decisão
Adotar **Tauri 2.0** como framework principal para o projeto Zenite, com:
- **Backend**: Rust Core (src-tauri/)
- **Frontend**: TypeScript/Vanilla (src/)
- **Comunicação**: Commands Tauri via invoke()

## Consequências

### Positivas
- **Performance**: Binários nativos muito menores que Electron (~3MB vs ~100MB+)
- **Segurança**: Sandboxing nativo e permissões explícitas no tauri.conf.json
- **Multiplataforma**: Suporte a Windows, macOS e Linux com um único código
- **Mobile-ready**: Tauri 2.0 suporta iOS e Android para expansão futura
- **Ecossistema Rust**: Acesso a crates de alta qualidade para funcionalidades nativas
- **Atualizações OTA**: Suporte nativo para atualizações automáticas

### Negativas
- **Curva de aprendizado**: Equipe precisa aprender Rust para o backend
- **Ecossistema menor**: Menos plugins e recursos prontos comparado ao Electron
- **Debugging**: Debugging de código Rust pode ser mais complexo
- **Limitações de APIs**: APIs web restritas por segurança, exigindo Commands Tauri

## Alternativas Consideradas

### Electron
- **Descrição**: Framework maduro baseado em Node.js + Chromium
- **Por que foi rejeitado**: Tamanho de bundle muito grande (~150MB), alto consumo de memória, e não atende à necessidade de apps leves

### Flutter Desktop
- **Descrição**: Framework Google com Dart
- **Por que foi rejeitado**: Curva de aprendizado de Dart, ecossistema desktop menos maduro, e preferência por tecnologias web

### Nativo (Swift + WinUI)
- **Descrição**: Desenvolvimento nativo por plataforma
- **Por que foi rejeitado**: Duplicação de esforço de desenvolvimento e manutenção

## Notas de Implementação
- Projeto inicializado com `npm create tauri-app@latest`
- Estrutura: frontend em TypeScript vanilla, backend em Rust
- Identificador do app: `com.miguelmartins.zenite`
- Versão inicial: 0.1.0

## Data
2026-02-12

## Participantes
- Miguel Martins
