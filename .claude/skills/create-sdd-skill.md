# Prompt: Criar Skill de Spec Driven Development

## Objetivo

Crie o arquivo `.claude/agents/sdd-docs.md` neste projeto. Esse arquivo é uma
skill reutilizável para o Claude Code que analisa qualquer projeto de software
e gera (ou atualiza) os 7 artefatos de Spec Driven Development.

## O que é uma skill de agente

Um arquivo em `.claude/agents/` com frontmatter YAML seguido de um prompt
estruturado. O Claude Code usa esse arquivo como contexto de sistema quando a
skill é ativada. A skill deve:

- Ser totalmente genérica — sem referências a tecnologias, domínios ou nomes
  específicos de projetos
- Guiar o agente por fases bem definidas: descoberta → leitura do código →
  seleção → geração → validação cruzada
- Conter templates concretos para cada artefato, com estrutura, regras e
  exemplos de preenchimento em placeholders (`<nome>`, `<tipo>`, etc.)

## Frontmatter obrigatório

```yaml
---
name: sdd-docs
description: >
  Analisa um projeto e gera artefatos de Spec Driven Development (SDD):
  OpenAPI/AsyncAPI, BDD/Gherkin, C4 Model, Sequence Diagrams, Data Schema,
  MADR e Roadmap. Use quando o usuário pedir para criar, revisar, completar
  ou auditar documentação de especificação de um serviço ou API.
tools:
  - Read
  - Write
  - Bash
  - WebSearch
---
```

## As 7 dimensões SDD a cobrir

| # | Dimensão        | Arquivo alvo             | Formato          |
|---|-----------------|--------------------------|------------------|
| 1 | Contrato de API | `docs/OPENAPI.yaml`      | OpenAPI 3.0.3    |
| 2 | Comportamento   | `docs/BEHAVIOR.feature`  | Gherkin/BDD      |
| 3 | Arquitetura     | `docs/C4_MODEL.md`       | C4 + Mermaid     |
| 4 | Interações      | `docs/SEQUENCE_DIAGRAM.md` | Mermaid sequenceDiagram |
| 5 | Modelo de dados | `docs/DATA_SCHEMA.md`    | ER + DDL + Mermaid |
| 6 | Decisões        | `docs/MADR.md`           | MADR (ADR)       |
| 7 | Roadmap         | `docs/ROADMAP.md`        | Épicos + Stories |

## Estrutura do corpo da skill

O prompt da skill deve ter as seguintes seções, nesta ordem:

### Objetivo
Uma ou duas frases descrevendo o que a skill faz e sua principal restrição
("artefatos derivados do código, nunca inventados").

### Fase 1 — Descoberta
Instrui o agente a listar `docs/` e identificar qual das 7 dimensões cada
arquivo existente cobre, usando uma tabela de indicadores de presença (ex:
arquivo `.feature` indica Comportamento; `openapi:` no YAML indica Contrato).
Antes de prosseguir, o agente relata ao usuário o que está coberto e o que falta.

### Fase 2 — Compreensão do domínio
Lista onde ler no código antes de gerar qualquer artefato:
entidades/tipos, contratos existentes, regras de negócio, persistência,
configuração. Define o que extrair: atores, recursos, ações, estados,
invariantes, sistemas externos.

### Fase 3 — Seleção
Se o usuário não especificou quais artefatos gerar: mostrar lacunas, perguntar
quais dimensões gerar (ou gerar todas com `--all`). Reforçar a ordem de
dependência entre artefatos.

### Fase 4 — Geração por dimensão (subseções 4.1 a 4.7)
Uma subseção por dimensão, cada uma contendo:
- Nome do arquivo alvo
- Estrutura esqueleto com placeholders (não dados reais)
- Regras específicas daquela dimensão (ex: "um `operationId` único por
  endpoint", "uma `Rule:` por regra de negócio", "Level 4 apenas para o
  componente mais crítico")

### Fase 5 — Validação cruzada
Tabela de verificações de coerência entre artefatos (ex: entidades do Schema
devem aparecer no OpenAPI; endpoints do OpenAPI devem ter Scenarios no BDD).
Instrução: se houver inconsistência, corrigir o artefato mais recente.

### Regras gerais
Lista curta (5 itens) de invariantes que se aplicam a todos os artefatos:
derivar do código, Mermaid para diagramas, sem dados sensíveis, metadados
de geração no topo, ordem de dependência entre artefatos.

## Restrições de qualidade

Ao escrever o prompt da skill, respeite:

1. **Sem dados de projeto** — nenhum nome de tabela, endpoint, tecnologia ou
   entidade específica deste repositório deve aparecer no arquivo gerado.
2. **Placeholders explícitos** — toda informação variável usa `<nome>`,
   `<tipo>`, `NNNN`, etc. Um leitor consegue preencher sem ver o projeto.
3. **Mermaid em todos os diagramas** — C4, sequenceDiagram, stateDiagram-v2,
   erDiagram, classDiagram.
4. **Templates completos** — cada dimensão tem um bloco de código com a
   estrutura mínima viável, não apenas uma descrição textual.
5. **Fases numeradas e sequenciais** — a skill deve ser executável de cima
   para baixo sem ambiguidade.

## Verificação após criação

Confirme que o arquivo gerado:
- Tem frontmatter YAML válido com `name`, `description` e `tools`
- Cobre as 7 dimensões em subseções dentro da Fase 4
- Não menciona nenhum projeto, tecnologia ou dado específico
- Contém templates com placeholders para cada dimensão
- Inclui a Fase 5 de validação cruzada com tabela de verificações
