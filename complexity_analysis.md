<!-- markdownlint-configure-file { "MD060": { "style": "leading_and_trailing" } } -->

# Analisi della Complessità Algoritmica — Compilatore jsavrs

## Notazione e Simboli

| Simbolo | Significato                                              |
| ------- | -------------------------------------------------------- |
| $n$     | Lunghezza del sorgente (caratteri)                       |
| $t$     | Numero di token prodotti dal lexer                       |
| $s$     | Numero di statement nell'AST                             |
| $a$     | Dimensione totale dell'AST (nodi)                        |
| $B$     | Numero di basic block nel CFG                            |
| $E$     | Numero di archi nel CFG                                  |
| $I$     | Numero di istruzioni IR totali                           |
| $V$     | Numero di variabili distinte                             |
| $F$     | Numero di funzioni                                       |
| $D$     | Profondità massima dell'albero dei dominatori            |
| $S$     | Profondità massima dello scope (nesting)                 |
| $P$     | Numero di parametri per funzione                         |
| $K$     | Numero di iterazioni del punto fisso (DCE)               |

---

## Riepilogo della Pipeline

La pipeline del compilatore jsavrs attraversa sette fasi principali in successione:

1. **Lexer** — tokenizzazione del sorgente
2. **Parser** — costruzione dell'AST (Pratt parser)
3. **Type Checker** — analisi semantica e verifica dei tipi
4. **IR Generator** — generazione dell'IR con CFG
5. **Dominanza + SSA** — costruzione forma SSA
6. **SCCP** — Sparse Conditional Constant Propagation
7. **DCE** — Dead Code Elimination

**Tempo complessivo (caso peggiore):**

- $O(n + t + a + V \cdot B^2 + I \cdot B)$

**Spazio complessivo (caso peggiore):**

- $O(n + t + a + B \cdot V + I)$

I **fattori dominanti** al livello di sistema sono:

1. La trasformazione SSA con calcolo di dominanza — $O(V \cdot B^2)$
2. L'analisi di liveness nel DCE — $O(K \cdot I \cdot B)$
3. La propagazione SCCP — $O(I \cdot B)$ nel caso peggiore

---

## 1. Lexer

**File:** `src/lexer.rs` — 253 righe

### Architettura

Il lexer utilizza il crate **logos** per la tokenizzazione automatica basata su automi. La funzione principale `lexer_tokenize_with_errors()` materializza l'intero vettore di token prima che il parser inizi.

### Complessità Temporale

#### `next_token()`

- **Amortizzato:** $O(1)$ per token (logos genera un DFA)
- **Totale per l'intero sorgente:** $O(n)$

#### `lexer_tokenize_with_errors()`

- Ciclo `while let Some(token_result) = lexer.next_token()` — $O(t)$ iterazioni
- Ogni iterazione: push in `Vec<Token>` o `Vec<CompileError>` — $O(1)$ amortizzato
- **Totale:** $O(n)$ (dove $t \leq n$)

#### `post_process_tokens()`

- `has_malformed_errors()`: scansione lineare degli errori — $O(m)$
- **Caso migliore (nessun errore hashtag):** $O(m)$, ritorno immediato
- **Caso peggiore:** $O(n + m)$ — costruzione `HashMap` per posizioni, poi sostituzione

#### Conteggio Linee Eseguite

```text
next_token():
  L1: if self.eof_emitted → return None       // 1 linea
  L2: match inner.next()                       // 1 linea
  L3: line_tracker.span_for(range)             // 1 linea
  L4: match kind_result → Ok/Err              // 1 linea
  Totale per chiamata: ~4 linee eseguibili
  Totale complessivo: 4 * t ≈ 4n/avg_token_len
```

### Complessità Spaziale

- `Vec<Token>`: $O(t)$ con pre-allocazione `source_len / 8`
- `Vec<CompileError>`: $O(m)$ con capacità iniziale 4
- `HashMap` in post-processing (caso peggiore): $O(m)$
- **Totale:** $O(n)$

### Tabella Riassuntiva

| Caso                 | Big-O  | Big-Ω       | Big-Θ       |
| -------------------- | ------ | ----------- | ----------- |
| Migliore (no errori) | $O(n)$ | $\Omega(n)$ | $\Theta(n)$ |
| Peggiore             | $O(n)$ | $\Omega(n)$ | $\Theta(n)$ |

---

## 2. Parser (Pratt Parser)

**File:** `src/parser/jsav_parser.rs` — 723 righe

### Architettura

Il parser implementa un **Pratt parser** (Top-Down Operator Precedence) con tre funzioni cardine:

- `parse_expr(min_bp)` — entry point per le espressioni con binding power minimo
- `nud()` — gestione dei prefissi (null denotation)
- `led()` — gestione degli infissi (left denotation)

Il design match-based nel `parse_stmt()` fa dispatch $O(1)$ per tipo di statement.

### Complessità Temporale

#### `parse()`

```text
parse():
  while !is_at_end():                 // t iterazioni max
    parse_stmt()                      // dispatch O(1) per statement
    → parse_expr(0)                   // visita sotto-token per espressioni
  shrink_to_fit() × 2                 // O(1) amortizzato
```

- Ogni token viene consumato esattamente **una volta** tramite `advance()`
- `peek()` e `check()` sono $O(1)$ (accesso per indice a slice)
- **Totale:** $O(t)$

#### Ciclo del Pratt parser

```text
parse_expr(min_bp):
  check_recursion_limit()             // O(1)
  left = nud()                        // consuma 1+ token
  loop:
    (lbp, _) = binding_power(peek)    // O(1) pattern match
    if lbp <= min_bp → break
    left = led(left)                  // consuma 1+ token, ricorsione su parse_expr
  return left
```

- **Profondità massima di ricorsione:** 1000 (`MAX_RECURSION_DEPTH`)
- Ogni chiamata ricorsiva consuma almeno un token → max $t$ chiamate totali
- **Complessità totale della ricorsione:** $O(t)$

#### `parse_type()`

- Match su `TokenKind` per tipo base — $O(1)$
- Ciclo per dimensioni array: $O(d)$ dove $d$ = dimensioni nidificate
- **Totale:** $O(d)$ per tipo, $O(t)$ complessivo

### Complessità Spaziale

- Stack di ricorsione: $O(\min(t, 1000))$
- AST output: $O(a)$ dove $a \leq t$
- `Vec::with_capacity(tokens.len() / 4)` per statement
- **Totale:** $O(t + a)$

### Tabella Riassuntiva

| Caso     | Big-O  | Big-Ω       | Big-Θ       |
| -------- | ------ | ----------- | ----------- |
| Migliore | $O(t)$ | $\Omega(t)$ | $\Theta(t)$ |
| Peggiore | $O(t)$ | $\Omega(t)$ | $\Theta(t)$ |

---

## 3. Type Checker

**File:** `src/semantic/type_checker.rs` — 972 righe

### Architettura

Il type checker attraversa l'AST in profondità (depth-first) con visitor pattern. Gestisce:

- Promozione numerica con gerarchia a 10 livelli: `F64 > F32 > U64 > I64 > U32 > I32 > U16 > I16 > U8 > I8`
- Tabella simboli a stack di scope (`SymbolTable`)
- Cache globale per le promozioni di tipo (`TYPE_PROMOTION_CACHE`)
- Lookup table precomputata (`TYPE_PROMOTION_TABLE`)

### Complessità Temporale

#### `check(statements)`

```text
check(statements):
  visit_statements(statements)              // O(s)
    for stmt in statements:
      visit_stmt(stmt)                      // dispatch O(1)
        → visit_var_declaration()           // O(P) per variabili
        → visit_function()                  // O(P) params + ricorsione su body
        → visit_expr()                      // ricorsione sull'AST
          → visit_binary_expr()             // O(1) + promozione tipi
          → visit_literal()                 // O(1) const
  return std::mem::take(&mut self.errors)   // O(1)
```

- **Attraversamento AST:** ogni nodo visitato esattamente una volta — $O(a)$
- **Costo per-nodo:** $O(1)$ per literals, binary, unary + $O(S)$ per lookup variabili

#### `promote_numeric_types(t1, t2)` — Analisi Dettagliata

```text
promote_numeric_types(t1, t2):
  if t1 == t2 → return t1                  // O(1) fast path
  check TYPE_PROMOTION_TABLE                // O(1) array lookup
  fallback: scan HIERARCHY[0..10]           // O(10) = O(1)
  cache result in TYPE_PROMOTION_CACHE      // O(1) amortizzato
```

- Caso base identico: $O(1)$
- Lookup table 10×10: $O(1)$
- Fallback con `HIERARCHY` a 10 elementi: $O(1)$ costante
- **Complessità:** $\Theta(1)$

#### Operazioni su Symbol Table

- `push_scope()`: $O(1)$ — push nel `Vec<Scope>`
- `pop_scope()`: $O(1)$ — pop dal `Vec<Scope>`
- `declare()`: $O(1)$ amortizzato — `HashMap::insert` nello scope corrente
- `lookup()`: $O(S)$ — scansione lineare dal scope più interno al globale
- `lookup_function()` / `lookup_variable()`: $O(S)$ — tramite `find_symbol()`

### Complessità Spaziale

- Stack di scope: $O(S)$
- Simboli totali: $O(V)$
- Stack dei tipi di ritorno: $O(S)$
- Cache promozioni: $O(1)$ — max 100 coppie (10×10)
- **Totale:** $O(S + V)$

### Tabella Riassuntiva

| Caso     | Big-O          | Big-Ω       | Big-Θ          |
| -------- | -------------- | ----------- | -------------- |
| Migliore | $O(a)$         | $\Omega(a)$ | $\Theta(a)$    |
| Peggiore | $O(a \cdot S)$ | $\Omega(a)$ | $O(a \cdot S)$ |

Il caso peggiore si verifica con nesting profondo dove ogni lookup variabile costa $O(S)$.

---

## 4. IR Generator

**File:** `src/ir/generator.rs` — 1696 righe

### Architettura

Il generatore opera in **due passi** sull'AST:

1. **Passo dichiarazioni:** registra tutte le funzioni nello `ScopeManager` — $O(F)$
2. **Passo generazione:** genera il body di ogni funzione con BasicBlocks e CFG — $O(a)$

Dopo la generazione, applica la trasformazione SSA a tutte le funzioni.

### Complessità Temporale

#### `generate(stmts, module_name)`

```text
generate(stmts, module_name):
  Passo 1 — Dichiarazioni:
    for stmt in stmts:                       // O(F) funzioni
      map_type(return_type)                  // O(1)
      scope_manager.add_symbol(name, value)  // O(1)

  Passo 2 — Generazione:
    for stmt in stmts:                       // O(F) funzioni
      create_function(...)                   // O(P) per parametri
      generate_function_body(...)            // O(body_size)

  Passo 3 — SSA:
    apply_ssa_transformation(module)         // vedi sezione 5
```

- Passo 1: $O(F)$
- Passo 2: $O(a)$ totale (ogni nodo AST genera $O(1)$ istruzioni)
- **Totale (escluso SSA):** $O(F + a)$

#### Costo di `recompute_reverse_post_order()`

Ogni chiamata a `add_block()` e `add_edge()` invoca `recompute_reverse_post_order()`:

```text
recompute_reverse_post_order():
  DFS dal blocco entry                       // O(B + E)
  reverse del vettore risultante             // O(B)
  Totale: O(B + E) per chiamata
```

Con $B$ blocchi e $E$ archi aggiunti incrementalmente, il costo ammortizzato totale è:

- $O(B \cdot (B + E))$ nel caso peggiore
- Un'ottimizzazione futura potrebbe differire il ricalcolo (lazy recomputation)

#### Costo di `find_block_by_label`

```text
find_block_by_label(label):
  graph.node_indices().find(|idx| graph[idx].label == label)
  Scansione lineare: O(B)
```

Chiamata frequentemente durante la generazione. Un'ottimizzazione futura potrebbe usare una `HashMap<Label, NodeIndex>`.

### Complessità Spaziale

- Istruzioni IR: $O(I)$
- BasicBlocks: $O(B)$
- Archi CFG: $O(E)$
- `ControlFlowStack`: $O(\text{loop\_depth})$ — pre-allocata per 64 livelli
- `format_buffer`: $O(1)$ — riutilizzato
- **Totale:** $O(I + B + E)$

### Tabella Riassuntiva

| Caso     | Big-O                  | Big-Ω       | Big-Θ                  |
| -------- | ---------------------- | ----------- | ---------------------- |
| Migliore | $O(a)$                 | $\Omega(a)$ | $\Theta(a)$            |
| Peggiore | $O(a + B \cdot (B+E))$ | $\Omega(a)$ | $O(a + B \cdot (B+E))$ |

---

## 5. Dominanza e Trasformazione SSA

### 5.1 Calcolo dei Dominatori

**File:** `src/ir/dominance.rs` — 241 righe

#### Algoritmo: Cooper-Harvey-Kennedy "A Simple, Fast Dominance Algorithm"

```text
compute_dominators(cfg):
  entry_idx = cfg.get_entry_block_index()       // O(B) find_block_by_label
  idom[entry] = entry                           // O(1)

  Pre-compute predecessors per tutti i nodi:    // O(B + E)
    predecessors: HashMap<NodeIndex, Vec<NodeIndex>>

  loop until no change:                         // max O(B) iterazioni
    for node in reverse_post_order:             // O(B) nodi
      for pred in predecessors[node]:           // O(deg(node))
        new_idom = intersect(new_idom, pred)    // O(D) per intersect
      if idom[node] changed → changed = true

  build_dominator_tree()                        // O(B)
```

- **Iterazioni totali:** $O(B)$ nel caso peggiore (solitamente 2-3 per grafi strutturati)
- **Costo per iterazione:** $O(B \cdot D)$ dove $D$ è la profondità dell'albero dei dominatori
- **`intersect()`:** cammina contemporaneamente verso l'alto nell'albero — $O(D)$
- **Caso peggiore complessivo:** $O(B^2)$ (grafi patologici con $D = O(B)$)
- **Caso tipico:** $O(B \cdot E)$ con poche iterazioni

#### Spazio

- `idom`: $O(B)$
- `predecessors`: $O(B + E)$
- `dom_tree_children`: $O(B)$
- **Totale:** $O(B + E)$

### 5.2 Calcolo delle Frontiere di Dominanza

```text
compute_dominance_frontiers(cfg):
  for b in cfg.node_indices():                  // O(B)
    preds = predecessors(b)
    if preds.len() >= 2:                        // join point
      for p in preds:                           // O(deg(b))
        runner = p
        while !dominates(runner, b):            // O(D) walk up
          dominance_frontiers[runner].insert(b) // O(1) amortizzato
          runner = idom[runner]
```

- **Complessità:** $O(B \cdot E)$ totale
- `dominates()`: cammina lungo l'albero dei dominatori — $O(D)$ per chiamata
- **Spazio:** $O(B^2)$ nel caso peggiore per le frontiere

### 5.3 Trasformazione SSA

**File:** `src/ir/ssa.rs` — 553 righe

#### Sotto-fasi e Conteggio Linee

**Fase 1 — Identificazione variabili phi (`identify_phi_variables`):**

```text
L1: for node_idx in cfg.node_indices()             // B iterazioni
L2:   for instruction in block.instructions         // I/B media
L3:     match instruction.kind                      // O(1)
L4:       Store → extract var_name, record def      // O(1) amortizzato
L5:       Alloca → extract var_name, record def     // O(1) amortizzato
L6: for (var_name, defs) in var_defs                // V iterazioni
L7:   if defs.len() > 1 → phi_variables.insert()   // O(1)
Totale: O(I + V)
```

**Fase 2 — Inserimento funzioni phi (`insert_phi_functions`):**

```text
L1: for var_name in phi_vars                        // V iterazioni
L2:   worklist = def_blocks[var_name]               // O(B) init
L3:   while let Some(block) = worklist.pop()        // O(B) max
L4:     for frontier_node in dominance_frontier     // O(B) max
L5:       if added_phis.insert(frontier_node)       // O(1)
L6:         add_phi_function(cfg, node, var)        // O(1) amortizzato
L7:         worklist.insert(frontier_node)          // O(1) amortizzato
Totale: O(V · B²) caso peggiore, O(V · B) tipico
```

**Fase 3 — Rinominamento variabili (`rename_variables_recursive`):**

```text
rename_variables_recursive(func, block_idx):
  process_block(func, block_idx):
    for phi in block.phis                           // O(phi_count)
      create new SSA value, push to stack           // O(1)
    for instruction in block.instructions           // O(I/B) media
      replace_value_with_current_ssa(operands)      // O(1) per operando
      if defines variable → push new value          // O(1)
    for successor → update phi incoming             // O(deg(block) · phi)
  for child in dom_tree_children:
    rename_variables_recursive(func, child)         // ricorsione
  pop_block_values(block_idx)                       // O(I/B)
Totale: O(I) — ogni istruzione visitata una volta
```

#### Complessità Complessiva SSA

| Sotto-fase            | Tempo                 | Spazio        |
| --------------------- | --------------------- | ------------- |
| compute_dominators    | $O(B^2)$              | $O(B + E)$    |
| dominance_frontiers   | $O(B \cdot E)$        | $O(B^2)$      |
| identify_phi_vars     | $O(I + V)$            | $O(V \cdot B)$ |
| insert_phi_functions  | $O(V \cdot B^2)$      | $O(V \cdot B)$ |
| rename_variables      | $O(I)$                | $O(V)$        |
| **Totale**            | $O(V \cdot B^2 + I)$  | $O(V \cdot B + B^2)$ |

---

## 6. SCCP — Sparse Conditional Constant Propagation

**File:** `src/ir/optimizer/constant_folding/propagator.rs` — 794 righe

### Architettura

Implementazione dell'algoritmo **Wegman-Zadeck** con due worklist separate:

- **CFG worklist**: archi del CFG da esplorare (flusso di controllo)
- **SSA worklist**: valori SSA da rivalutare (flusso dei dati)

Strutture dati accessorie:

- `LatticeState`: mappa `ValueId → LatticeValue` (Bottom, Constant, Top)
- `ExecutableEdgeSet`: insieme degli archi eseguibili del CFG
- `Worklist<T>`: coda con deduplicazione tramite `HashSet`

### Algoritmo di Wegman-Zadeck

```text
propagate(function, max_iterations):
  initialize():
    mark entry edge executable                     // O(1)
    push entry edge to cfg_worklist                // O(1)

  iterations = 0
  while iterations < max_iterations:
    made_progress = false

    // Fase CFG
    while let Some(edge) = cfg_worklist.pop():     // O(E) totale
      visit_block(function, edge.to):
        for instr in block.instructions:           // O(I/B) per blocco
          visit_instruction(instr)                 // O(1) lattice meet
        visit_terminator(block.terminator)         // O(1). Mark outgoing edges

    // Fase SSA
    while let Some(value_id) = ssa_worklist.pop(): // O(I) totale
      visit_value(function, value_id)              // O(1) re-evaluate

    if !made_progress → break                      // Punto fisso raggiunto
    iterations++
```

### Complessità Temporale

- **Proprietà di monotonia:** ogni valore nel lattice può salire al massimo 2 volte (Bottom → Constant → Top)
- **Numero totale di transizioni:** $O(I)$
- **Ogni transizione** può aggiungere al più $O(\text{uses})$ elementi alla SSA worklist
- **Caso tipico:** $O(I + E)$ — punto fisso raggiunto in 2-3 iterazioni
- **Caso peggiore:** $O(I \cdot B)$ — grafi con molti back-edges e cascate di rivalutazione

### Complessità Spaziale

- `LatticeState`: $O(I)$
- `ExecutableEdgeSet`: $O(E)$
- Worklist CFG: $O(E)$
- Worklist SSA: $O(I)$
- **Totale:** $O(I + E)$

### Tabella Riassuntiva

| Caso     | Big-O          | Big-Ω           | Big-Θ          |
| -------- | -------------- | --------------- | -------------- |
| Migliore | $O(I + E)$     | $\Omega(I + E)$ | $\Theta(I+E)$  |
| Peggiore | $O(I \cdot B)$ | $\Omega(I + E)$ | $O(I \cdot B)$ |

---

## 7. DCE — Dead Code Elimination

**File:** `src/ir/optimizer/dead_code_elimination/optimizer.rs` — 461 righe

### Architettura

Ottimizzazione a **punto fisso** con due fasi per iterazione:

- **Fase 1:** Analisi di raggiungibilità e rimozione blocchi irraggiungibili
- **Fase 2:** Analisi di liveness e rimozione istruzioni morte

Impostazione predefinita: `max_iterations = 10`.

### Conteggio Linee e Sotto-fasi

```text
optimize_function(function):
  for iteration in 1..=max_iterations:             // max K iterazioni
    changed = false

    // Fase 1 — Raggiungibilità
    changed |= remove_unreachable_blocks(function)

    // Fase 2 — Istruzioni morte
    dead_removed = remove_dead_instructions(function)
    changed |= dead_removed > 0

    if !changed → break                            // convergenza
```

**`remove_unreachable_blocks`:**

```text
L1: ReachabilityAnalyzer::analyze(cfg)             // O(B + E) DFS
L2: filter blocks not in reachable set             // O(B)
L3: update_phi_nodes_for_removed_blocks            // O(B · I_phi)
L4: cfg.remove_block(label) per blocco             // O(B) totale
Totale: O(B + E)
```

**`remove_dead_instructions`:**

```text
loop:                                              // sub-iterazioni fino a convergenza
  analyzer = LivenessAnalyzer::new()
  build_def_use_chains(function)                   // O(I)
  compute_gen_kill_sets(function)                   // O(I)
  analyze(function)                                // O(I · B) punto fisso

  escape_analyzer.analyze(function)                // O(I)

  identify_dead_instructions(function)             // O(I)
  remove_instructions(function, dead_list)         // O(I) con sort + rimozione

  if dead_list.empty → break
```

### Analisi di Liveness (Backward Dataflow)

**File:** `src/ir/optimizer/dead_code_elimination/analyzer.rs` — 315 righe

```text
analyze(function):
  // Inizializzazione
  for block in cfg:
    live_in[block] = ∅
    live_out[block] = ∅

  // Punto fisso backward
  changed = true
  while changed:                                   // max O(B) iterazioni
    changed = false
    for block in reverse_post_order:               // O(B)
      // live_out = ∪ live_in[successor]
      new_out = union of live_in[succs]            // O(I)
      // live_in = gen ∪ (live_out - kill)
      new_in = gen[block] ∪ (new_out - kill[block]) // O(I)
      if new_in ≠ live_in[block]:
        live_in[block] = new_in
        changed = true
```

- **Iterazioni:** $O(D)$ nel caso tipico, $O(B)$ nel peggiore
- **Costo per iterazione:** $O(B \cdot |live\_set|)$ ≈ $O(I)$
- **Totale:** $O(I \cdot B)$

### Complessità Complessiva DCE

| Sotto-fase             | Tempo per iterazione | Spazio    |
| ---------------------- | -------------------- | --------- |
| remove_unreachable     | $O(B + E)$           | $O(B)$   |
| build_def_use_chains   | $O(I)$               | $O(I)$   |
| compute_gen_kill       | $O(I)$               | $O(I)$   |
| liveness analysis      | $O(I \cdot B)$       | $O(I)$   |
| escape analysis        | $O(I)$               | $O(I)$   |
| identify + remove dead | $O(I \log I)$        | $O(I)$   |
| **Totale per iter.**   | $O(I \cdot B)$       | $O(I)$   |

**Totale con $K$ iterazioni:** $O(K \cdot I \cdot B)$

### Tabella Riassuntiva

| Caso     | Big-O                | Big-Ω           | Big-Θ                |
| -------- | -------------------- | --------------- | -------------------- |
| Migliore | $O(I + E)$           | $\Omega(I + E)$ | $\Theta(I + E)$      |
| Peggiore | $O(K \cdot I \cdot B)$ | $\Omega(I + E)$ | $O(K \cdot I \cdot B)$ |

---

## 8. Symbol Table

**File:** `src/semantic/symbol_table.rs` — 379 righe

### Struttura dati

Stack di `Scope`, ogni scope contiene una `HashMap<Arc<str>, Symbol>`.

```text
push_scope():   O(1)
pop_scope():    O(1)
declare():      O(1) amortizzato — HashMap::insert
lookup():       O(S) — scansione lineare dal scope più interno
find_symbol():  O(S) — generico con filtro
```

- **Invariante:** almeno uno scope (globale) sempre presente
- **Lookup ottimizzato**: il pattern `for scope in scopes.iter().rev()` termina al primo match

### Spazio

- $O(\sum_i |scope_i|)$ = $O(V)$ per tutti i simboli dichiarati
- Stack overhead: $O(S)$

---

## 9. Interazione tra Componenti

### Flusso dei Dati nella SSA

Tre analisi interconnesse in sequenza stretta:

```text
Flusso dei dati:
  DominanceInfo ──→ identify_phi_variables ──→ insert_phi_functions
       │                                              │
       └──→ rename_variables_recursive ◄──────────────┘
                     │
                     ▼
              Forma SSA completa
```

### Interazione SCCP — Worklist Duale

I due worklist del SCCP interagiscono in modo fine:

```text
CFG worklist pop → visit_block() → per ogni istruzione:
  │                                   │
  │   lattice.update(value_id, val)   │
  │         │                         │
  │         └──── if changed ───→ ssa_worklist.push(uses)
  │
  └──→ visit_terminator() → mark_executable(new_edges)
                                  │
                                  └──→ cfg_worklist.push(new_edge)
```

### Parser — Ciclo nud/led

Il ciclo del Pratt parser è una danza precisa tra tre funzioni:

```text
parse_expr(min_bp):
  left = nud()                    // consuma prefix token
  │
  └─ loop ─┐
            │ peek → binding_power(token) → (lbp, rbp)
            │ if lbp <= min_bp → return left
            │ left = led(left)    // consuma infix, ricorsione parse_expr(rbp)
            └──────┘
```

### Type Checker — Scope Management

Il type checker gestisce gli scope con precisione chirurgica:

```text
visit_function():
  declare function in current scope            // O(1)
  push_scope(Function)                         // O(1)
  push return_type_stack                       // O(1)
  for param in parameters:
    declare param in function scope            // O(1)
  visit_statements(body)                       // O(body_nodes)
    visit_if → push_scope(Block) ... pop       // nested scopes
    visit_while → push_scope(Block) ... pop
  check return coverage                        // O(body)
  pop return_type_stack                        // O(1)
  pop_scope()                                  // O(1)
```

---

## 10. Riepilogo Complessità Globale

### Tempo

| Fase                  | Complessità (caso peggiore)       |
| --------------------- | --------------------------------- |
| Lexer                 | $O(n)$                            |
| Parser                | $O(t)$                            |
| Type Checker          | $O(a \cdot S)$                    |
| IR Generator          | $O(a + B \cdot (B + E))$         |
| Dominanza             | $O(B^2)$                          |
| SSA                   | $O(V \cdot B^2 + I)$             |
| SCCP                  | $O(I \cdot B)$                    |
| DCE                   | $O(K \cdot I \cdot B)$           |
| **Pipeline completa** | $O(n + V \cdot B^2 + K \cdot I \cdot B)$ |

### Spazio

| Fase                  | Complessità                  |
| --------------------- | ---------------------------- |
| Lexer                 | $O(n)$                       |
| Parser                | $O(t + a)$                   |
| Type Checker          | $O(S + V)$                   |
| IR Generator          | $O(I + B + E)$               |
| Dominanza + SSA       | $O(V \cdot B + B^2)$        |
| SCCP                  | $O(I + E)$                   |
| DCE                   | $O(I)$                       |
| **Pipeline completa** | $O(n + a + V \cdot B + I)$  |

---

## 11. Raccomandazioni per Ottimizzazioni Future 

1. **`find_block_by_label`** — Sostituire la scansione lineare $O(B)$ con una `HashMap<Label, NodeIndex>` per portarla a $O(1)$

2. **`recompute_reverse_post_order`** — Implementare lazy recomputation: differire il ricalcolo fino a quando il RPOT è effettivamente necessario, evitando $O(B+E)$ ad ogni `add_block` e `add_edge`

3. **Dominanza incrementale** — Per piccole modifiche al CFG, aggiornare l'albero dei dominatori incrementalmente anziché ricalcolare da zero

4. **SSA destruction** — Aggiungere una fase di de-SSA per la generazione di codice assembly

5. **SCCP + DCE fusion** — Combinare le due ottimizzazioni in un unico passo per ridurre il numero di attraversamenti del CFG

6. **Parallelizzazione per-funzione** — Le funzioni sono indipendenti dopo la fase di dichiarazione; la generazione IR e le ottimizzazioni possono essere parallelizzate con `rayon`
