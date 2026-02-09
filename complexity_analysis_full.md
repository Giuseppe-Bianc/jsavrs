# Analisi Rigorosa di Complessità Algoritmica — Compilatore jsavrs

> Analisi aggiornata al 9 febbraio 2026 — basata su ispezione diretta del codice sorgente.

---

## Metrica di Costo

La metrica primaria è il **numero di operazioni elementari eseguite** (accessi a strutture dati, confronti, assegnamenti, inserzioni in collezioni). Le variabili dimensionali usate uniformemente:

| Simbolo | Significato |
|---------|-------------|
| $n$ | Lunghezza del sorgente in caratteri |
| $t$ | Numero di token prodotti dal lexer |
| $s$ | Numero di nodi AST (statement + expression) |
| $B$ | Numero di basic block nel CFG di una funzione |
| $E$ | Numero di archi nel CFG |
| $V$ | Numero di variabili/valori SSA in una funzione |
| $I$ | Numero totale di istruzioni IR in una funzione |
| $F$ | Numero di funzioni nel modulo |
| $d$ | Profondità massima di annidamento degli scope |
| $p$ | Numero massimo di parametri di una funzione |
| $k$ | Numero di livelli di precedenza operatori (costante = 13) |

---

## 1. Panoramica a Livello di Sistema

### Pipeline di Compilazione

Il compilatore jsavrs esegue una pipeline **sequenziale** a 6 fasi (come visibile in `src/main.rs`):

$$T_{\text{totale}}(n) = T_{\text{lex}}(n) + T_{\text{parse}}(t) + T_{\text{sem}}(s) + T_{\text{irgen}}(s) + T_{\text{opt}}(I, B, E, V) + T_{\text{codegen}}(I)$$

Poiché le fasi sono sequenziali, la complessità totale è determinata dalla fase dominante.

### Sommario Complessità a Livello di Sistema

| Fase | Complessità Temporale (worst) | Complessità Spaziale |
|------|-------------------------------|---------------------|
| Lexer | $\Theta(n)$ | $\Theta(t)$ |
| Parser | $\Theta(t)$ | $O(s)$ |
| Semantic Analysis | $O(s \cdot d)$ | $O(V \cdot d)$ |
| IR Generation | $O(s \cdot d + B^2)$ | $O(I + B + E)$ |
| SSA Transform | $O(B^2 \cdot (E + V))$ | $O(B^2 + B \cdot V)$ |
| Dominance | $O(B^2 \cdot E)$ | $O(B + E)$ |
| DCE (totale) | $O(E \cdot V + I)$ | $O(B \cdot V + I)$ |
| SCCP | $\Theta(I + E)$ | $O(I + E)$ |
| Codegen | $\Theta(I)$ | $\Theta(I)$ |

**Fase dominante nel caso peggiore:** la trasformazione SSA con complessità $O(B^2 \cdot (E + V))$ per funzione, ovvero $O(F \cdot B^2 \cdot (E + V))$ a livello di modulo. In pratica, per CFG sparsi tipici, è molto più vicina a $O(F \cdot (B + E) \cdot V)$.

---

## 2. Connessioni Inter-Sistema

### 2.1 Lexer → Parser

- **Interfaccia:** `Vec<Token>` prodotta da `lexer_tokenize_with_errors` e consumata da `JsavParser::new(&tokens)`.
- **Costo di trasferimento:** $\Theta(1)$ — il parser riceve un riferimento a slice `&[Token]`, nessuna copia.
- **Fattore di espansione:** $t = O(n)$. Ogni token consuma almeno 1 carattere → $t \leq n$.

### 2.2 Parser → Analisi Semantica

- **Interfaccia:** `Vec<Stmt>` passata per riferimento `&[Stmt]`.
- **Costo di trasferimento:** $\Theta(1)$.
- **Invariante:** $s \leq t$ — ogni nodo AST è prodotto consumando $\geq 1$ token.

### 2.3 Analisi Semantica → IR Generator

- **Interfaccia:** lo stesso `Vec<Stmt>` è **clonato** (`statements.clone()`) in `main.rs:124`.
- **Costo di clonazione:** $O(s)$ — deep clone dell'AST. Costo additivo lineare evitabile con move semantics.

### 2.4 IR Generator → Optimizer Pipeline

- **Interfaccia:** `&mut Module` passato a `run_pipeline`.
- **Costo di trasferimento:** $\Theta(1)$ — riferimento mutabile.
- **Pipeline:** sequenziale — Constant Folding → DCE (come in `main.rs:140-143`).

### 2.5 Optimizer → Codegen

- **Interfaccia:** `Module` passato per move ad `AsmGen::new(module)`.
- **Costo:** $\Theta(1)$ — move semantics, zero-copy.

### Overhead Aggregato Inter-Sistema

$$T_{\text{inter}}(n, s) = O(s)$$

Dominato dalla clonazione AST. Lineare, non altera la complessità asintotica totale.

---

## 3. Analisi per Componente

---

### 3.1 Lexer (`src/lexer.rs`)

#### Algoritmo

Il lexer usa il crate `logos` che genera un **automa a stati finiti deterministico (DFA)** a compile-time. L'operazione è un singolo passaggio sequenziale.

#### Relazione di Ricorrenza

$$T_{\text{lex}}(n) = \sum_{i=1}^{t} c_i + T_{\text{post}}(t)$$

dove $c_i$ è il costo costante per riconoscere il token $i$-esimo.

#### Derivazione Step-by-Step

1. `lexer_tokenize_with_errors` (riga 153): itera con `while let Some(token_result) = lexer.next_token()`
2. `next_token()` (riga 99): chiama `self.inner.next()` — una transizione DFA → $O(1)$ per carattere
3. `line_tracker.span_for(range)`: calcolo dello span → $O(1)$
4. Push in `Vec<Token>` con pre-allocazione `Vec::with_capacity(source_len / 8)` → $O(1)$ ammortizzato
5. `post_process_tokens` (riga 170+): scansione lineare per gestione errori hashtag → $O(t)$

$$T_{\text{lex}}(n) = \sum_{i=1}^{n} O(1) + O(t) = O(n)$$

Poiché ogni carattere **deve** essere letto almeno una volta: $\Omega(n)$.

#### Complessità Temporale

| | |
|---|---|
| **Caso migliore** | $\Omega(n)$ |
| **Caso peggiore** | $O(n)$ |
| **Limite stretto** | $\Theta(n)$ |

#### Complessità Spaziale

- Output `Vec<Token>`: $O(t) \subseteq O(n)$
- `LineTracker`: $O(L)$ dove $L \leq n$ (numero di linee)
- Stato DFA: $O(1)$
- **Totale:** $\Theta(n)$

#### Applicabilità del Teorema Master

**Non applicabile** — il lexer è iterativo, non ricorsivo.

---

### 3.2 Parser (`src/parser/jsav_parser.rs`)

#### Algoritmo

Parser ricorsivo discendente con **Pratt parsing** (operatori a precedenza) per le espressioni. 723 righe di codice.

#### Relazione di Ricorrenza — Statement Parsing

Il ciclo in `parse()` (riga 53):

```rust
while !self.is_at_end() {
    if let Some(stmt) = self.parse_stmt() { ... }
    else { self.advance(); }
}
```

$$T_{\text{parse\_stmts}}(t) = \sum_{i=1}^{S} T_{\text{parse\_stmt}}(t_i) \quad \text{con } \sum t_i = t$$

`parse_stmt` (riga 69): match in $O(1)$ sul `TokenKind` → dispatch a parser specifico.

#### Relazione di Ricorrenza — Expression Parsing (Pratt)

Nucleo in `parse_expr_inner` (riga 458):

```rust
fn parse_expr_inner(&mut self, min_bp: u8) -> Option<Expr> {
    let mut left = self.nud()?;           // consuma ≥1 token
    while let Some(token) = self.peek() {
        let (lbp, _) = binding_power(token);
        if lbp <= min_bp { break; }
        left = self.led(left)?;           // consuma ≥1 token
    }
    Some(left)
}
```

**Proprietà fondamentale:** ogni chiamata a `nud()` e `led()` consuma **almeno un token** (`self.advance()`). Pertanto il numero totale di operazioni è limitato dal numero di token nell'espressione.

Per un'espressione con $m$ token:

$$T_{\text{expr}}(m) = \Theta(m)$$

La profondità di ricorsione è limitata dai livelli di precedenza $k = 13$ (costante) e dal `MAX_RECURSION_DEPTH = 1000`.

#### Derivazione — Complessità Totale

Ogni token viene processato esattamente una volta (advance) o al più due volte (con peek):

$$T_{\text{parse}}(t) = \sum_{\text{ogni token}} O(1) = \Theta(t)$$

#### Complessità Temporale

| | |
|---|---|
| **Caso migliore** | $\Omega(t)$ |
| **Caso peggiore** | $O(t)$ |
| **Limite stretto** | $\Theta(t)$ |

#### Complessità Spaziale

- AST: $O(s)$ dove $s \leq t$
- Stack di ricorsione: $O(\min(d, 1000))$
- Errori: $O(e)$ con $e \ll t$ tipicamente
- **Totale:** $O(s + d) = O(t)$

#### Applicabilità del Teorema Master

**Non applicabile** — il Pratt parser ha ricorsione ma non della forma $T(n) = aT(n/b) + f(n)$. È un parser lineare con ricorsione limitata dalla profondità di precedenza (costante).

---

### 3.3 Analisi Semantica (`src/semantic/type_checker.rs`, `symbol_table.rs`)

#### Algoritmo

Visita in profondità (DFS) dell'AST con gestione della symbol table basata su stack di scope. 972 righe nel type checker, 379 nella symbol table.

#### Relazione di Ricorrenza

$$T_{\text{sem}}(s) = \sum_{i=1}^{s} T_{\text{visit}}(n_i)$$

Per la maggior parte dei nodi:

$$T_{\text{visit}}(n_i) = O(1) + T_{\text{lookup/declare}} + \sum_{\text{figli}} T_{\text{visit}}$$

#### Costo delle Operazioni sulla Symbol Table

La `SymbolTable` (riga 96) è un `Vec<Scope>` dove ogni `Scope` contiene un `HashMap<Arc<str>, Symbol>`:

| Operazione | Implementazione | Costo |
|-----------|----------------|-------|
| `declare(name, symbol)` | `HashMap::insert` nello scope corrente (riga 257) | $O(1)$ ammortizzato |
| `lookup(name)` | `find_symbol`: iterazione `for scope in self.scopes.iter().rev()` (riga 280) | $O(d)$ |
| `push_scope` | `Vec::push` (riga 147) | $O(1)$ |
| `pop_scope` | `Vec::pop` (riga 162) | $O(1)$ |

La lookup traversa lo stack di scope dal più interno al più esterno:

$$T_{\text{lookup}}(d) = O(d)$$

#### Costo Type Promotion

`TYPE_PROMOTION_TABLE` (riga 87): array statico `[u8; 100]` con `OnceLock`. Accesso: $O(1)$.

#### Derivazione Complessità Totale

Ogni nodo AST visitato una volta. Il costo per nodo è dominato dalle lookup:

$$T_{\text{sem}}(s, d) = \sum_{i=1}^{s} O(d_i) \leq s \cdot d$$

dove $d$ è la profondità massima dello stack.

#### Complessità Temporale

| | |
|---|---|
| **Caso migliore** | $\Omega(s)$ — variabili sempre in scope locale |
| **Caso peggiore** | $O(s \cdot d)$ — lookup attraversa tutto lo stack |
| **Caso medio** | $\Theta(s \cdot \bar{d})$ con $\bar{d}$ profondità media |

*In pratica:* $d$ è piccolo ($\leq 10$) → effettivamente $\Theta(s)$.

#### Complessità Spaziale

- Symbol Table: $O(V \cdot d)$
- Stack di ricorsione: $O(d)$
- Return type stack: $O(d_f)$ (profondità funzioni annidate)
- **Totale:** $O(V \cdot d + s)$

---

### 3.4 IR Generator (`src/ir/generator.rs`)

#### Algoritmo

Due passaggi sull'AST (1696 righe):

1. **Pass di dichiarazione** (righe 249-266): registra funzioni → $O(F)$
2. **Pass di generazione** (righe 269-289): genera IR per ogni corpo → $O(s)$ per passaggio di base

#### Relazione di Ricorrenza

$$T_{\text{irgen}}(s, F) = O(F) + \sum_{f=1}^{F} T_{\text{gen\_body}}(s_f) + T_{\text{SSA}}$$

#### Costo Critico: Costruzione CFG

`ControlFlowGraph::add_block` (riga 41 in `cfg.rs`):

```rust
pub fn add_block(&mut self, block: BasicBlock) -> NodeIndex {
    let idx = self.graph.add_node(block);
    self.recompute_reverse_post_order();  // O(B + E) ogni volta!
    idx
}
```

`recompute_reverse_post_order` (riga 205): DFS completo → $O(B + E)$.

`find_block_by_label` (riga 54): ricerca lineare → $O(B)$.

**Quando si aggiungono $B$ blocchi incrementalmente:**

$$T_{\text{cfg\_build}} = \sum_{i=1}^{B} O(i + E_i) = O\left(\frac{B^2}{2} + B \cdot E\right) = O(B^2)$$

per CFG sparsi dove $E = O(B)$.

#### Complessità Temporale

| | |
|---|---|
| **Caso migliore** | $\Omega(s)$ |
| **Caso peggiore** | $O(s \cdot d + B^2)$ |
| **Caso medio** | $O(s + B \cdot \sqrt{B})$ |

Con $B = O(s)$ nel caso peggiore → $O(s^2)$.

#### Complessità Spaziale

- CFG: $O(B + E)$
- Istruzioni IR: $O(I) = O(s)$
- ScopeManager: $O(V \cdot d)$
- ControlFlowStack: $O(d_{\text{loop}})$
- Format buffer: $O(1)$
- **Totale:** $O(I + B + E + V \cdot d)$

---

### 3.5 Trasformazione SSA (`src/ir/ssa.rs`, `dominance.rs`)

#### 3.5.1 Calcolo dei Dominatori (Cooper-Harvey-Kennedy)

**Algoritmo** (righe 37-118 in `dominance.rs`): iterazione a punto fisso.

```
repeat until no changes:
    for each b in RPO \ {entry}:
        new_idom = first processed predecessor
        for each other processed pred p:
            new_idom = intersect(new_idom, p)
        if idom[b] changed: mark changed
```

**`intersect`** (righe 183-199): risale l'albero dei dominatori confrontando indici:

```rust
while finger1.index() > finger2.index() {
    finger1 = idom[finger1];
}
while finger2.index() > finger1.index() {
    finger2 = idom[finger2];
}
while finger1 != finger2 {
    finger1 = idom[finger1];
    finger2 = idom[finger2];
}
```

Costo: $O(\text{depth of dominator tree})$. Worst case: $O(B)$.

**Costo per iterazione:**

$$T_{\text{dom\_iter}} = \sum_{b \in B} |\text{pred}(b)| \cdot O(B) = O(E \cdot B)$$

**Numero di iterazioni:** tipicamente 2-3 per CFG riducibili, $O(\text{loop nesting depth})$ nel caso peggiore.

- **Worst case:** $O(B^2 \cdot E)$ (molte iterazioni su CFG con loop annidati)
- **Caso pratico:** $\Theta(B + E)$ con $\leq 3$ iterazioni
- **Spazio:** `idom` $O(B)$ + `predecessors` $O(B + E)$ + `dom_tree_children` $O(B)$ → $O(B + E)$

#### 3.5.2 Dominance Frontier (righe 124-160 in `dominance.rs`)

```
for each b with |pred(b)| ≥ 2:
    for each p in pred(b):
        runner = p
        while runner does not dominate b:
            DF[runner] ∪= {b}
            runner = idom[runner]
```

- **Worst case:** $O(B \cdot E)$
- **Caso pratico:** $\Theta(B + E)$
- **Spazio:** $O(B^2)$ per le frontier sets nel caso peggiore

#### 3.5.3 Phi-Function Insertion (righe 192-236 in `ssa.rs`)

Worklist algorithm:

```
for each variable v in phi_variables:
    worklist = def_blocks(v)
    while worklist not empty:
        b = pop(worklist)
        for each d in DF(b):
            if not already added phi for v at d:
                add phi for v at d
                if d not in def_blocks(v): worklist.add(d)
```

- **Worst case:** $O(V \cdot B^2)$ (ogni variabile potrebbe richiedere phi in molti blocchi)
- **Caso pratico:** $O(V \cdot B)$ per CFG sparsi
- **Spazio:** $O(V \cdot B)$

**Nota critica:** `block.instructions.insert(0, phi_inst)` (riga per l'inserimento) costa $O(|I_b|)$ per spostare gli elementi. Costo aggregato: $O(V \cdot I)$ nel caso peggiore.

#### 3.5.4 Variable Renaming (righe 282-370 in `ssa.rs`)

DFS ricorsivo sull'albero dei dominatori:

$$T_{\text{rename}} = \sum_{b \in B} \left[ O(|I_b|) + |\text{succ}(b)| \cdot V_\phi \right]$$

dove $V_\phi = |\text{phi\_variables}|$.

- **Worst case:** $O(I + E \cdot V)$
- **Spazio:** $O(B + V)$ stack di ricorsione + value stacks

#### 3.5.5 SSA Verification (righe 486-520 in `ssa.rs`)

Scansione lineare con HashSet:
$$T_{\text{verify}} = O(I), \quad S_{\text{verify}} = O(I)$$

#### Complessità Complessiva SSA per Funzione

$$T_{\text{SSA}} = T_{\text{dom}} + T_{\text{DF}} + T_\phi + T_{\text{rename}} + T_{\text{verify}}$$

| | |
|---|---|
| **Worst** | $O(B^2 \cdot E + V \cdot B^2 + I + E \cdot V) = O(B^2 \cdot (E + V))$ |
| **Pratico** | $\Theta(B + E + B \cdot V + I) = \Theta((B + E) \cdot V + I)$ |
| **Spazio** | $O(B^2 + B \cdot V + I)$ |

---

### 3.6 Dead Code Elimination (`src/ir/optimizer/dead_code_elimination/`)

#### Struttura (461 righe in `optimizer.rs`, 334 in `analyzer.rs`, 162 in `escape.rs`)

Iterazione a punto fisso con `max_iterations = 10`. Ogni iterazione:

1. **Reachability** — DFS dal blocco entry
2. **Liveness** — backward dataflow analysis
3. **Escape** — flow-insensitive analysis
4. **Removal** — rimozione istruzioni morte

#### 3.6.1 Reachability (`ReachabilityAnalyzer::analyze`)

DFS standard: $T = O(B + E)$, $S = O(B)$

#### 3.6.2 Liveness Analysis

**Build def-use chains** (righe 62-84 in `analyzer.rs`): scansione lineare → $O(I)$

**Gen-Kill sets** (righe 87-108): per-block → $O(I)$

**Backward dataflow** (righe 145-198):
```
repeat (max 10 times) until convergence:
    for each block in reverse RPO:
        live_out[b] = ∪ live_in[succ(b)]
        live_in[b]  = gen[b] ∪ (live_out[b] - kill[b])
```

Costo per iterazione: $O(E \cdot V)$ (unioni di set di dimensione $\leq V$).

Con $\leq 10$ iterazioni: $O(E \cdot V)$.

**Spazio:** $O(B \cdot V)$ per live_in/live_out sets.

#### 3.6.3 Escape Analysis (righe 83-100 in `escape.rs`)

- `initialize_allocas`: $O(I)$
- `scan_for_escapes`: $O(I)$
- **Totale:** $O(I)$, **Spazio:** $O(V)$

#### Complessità Complessiva DCE per Funzione

Con `max_iterations = 10` per il loop esterno, ciascuno contenente liveness (max 10 iter):

$$T_{\text{DCE}} = O(10 \cdot (B + E + I + E \cdot V + I)) = O(E \cdot V + I)$$

| | |
|---|---|
| **Worst** | $O(E \cdot V + I)$ |
| **Best** | $\Omega(I)$ — nessun codice morto, 1 iterazione |
| **Spazio** | $O(B \cdot V + I)$ |

---

### 3.7 SCCP — Constant Folding (`src/ir/optimizer/constant_folding/`)

#### Algoritmo

Algoritmo di **Wegman-Zadeck** (Sparse Conditional Constant Propagation) con due worklist (794 righe nel propagator).

#### Proprietà del Lattice

Il lattice ha altezza 3: $\bot$ (Bottom/unreachable) → Constant → $\top$ (Top/overdefined).

Ogni valore SSA può cambiare stato al più **2 volte**. Ogni arco CFG diventa eseguibile al più **1 volta**.

#### Analisi del Loop Principale

```rust
while iterations < max_iterations {
    // Process CFG worklist
    while let Some(edge) = self.cfg_worklist.pop() { ... }
    // Process SSA worklist
    while let Some(value_id) = self.ssa_worklist.pop() { ... }
    if !made_progress { break; }
}
```

- CFG worklist: ogni arco entra al più 1 volta → $O(E)$ pop totali
- SSA worklist: ogni valore entra al più 2 volte → $O(I)$ pop totali
- Ogni pop costa $O(1)$ (VecDeque + HashSet per dedup)

$$T_{\text{SCCP}} = O(E + 2 \cdot I) = \Theta(I + E)$$

| | |
|---|---|
| **Limite stretto** | $\Theta(I + E)$ |
| **Spazio** | $O(I + E)$ |

#### Applicabilità del Teorema Master

**Non applicabile** — algoritmo iterativo basato su worklist monotono, non ricorsivo.

---

### 3.8 Code Generation (`src/codegen/asmgen.rs`)

L'attuale implementazione (100 righe) è una scaffolding — `gen_asm()` restituisce l'`AssemblyFile` senza traduzione IR.

**Complessità attuale:** $\Theta(1)$

**Complessità progettata (traduzione completa):** $\Theta(I)$ — ogni istruzione IR → numero costante di istruzioni assembly.

---

## 4. Connessioni Intra-Componente (Analisi di Massimo Dettaglio)

### 4.1 Intra-Lexer: DFA `logos` ↔ `LineTracker`

Per ogni token prodotto dal DFA:

1. `logos::Lexer::next()` → produce `(TokenKind, Range<usize>)` — $O(1)$
2. `line_tracker.span_for(range)` → converte byte offset in `(linea, colonna)` — $O(1)$ con indice precomputato
3. Costruzione `Token { kind, span }` — $O(1)$

Le due strutture comunicano **solo attraverso il byte range**. Nessuna dipendenza circolare. L'interazione è **puntuale e a costo costante** per token.

$$T_{\text{DFA↔LT}} = \Theta(1) \text{ per token}, \quad \Theta(t) \text{ totale}$$

### 4.2 Intra-Parser: Pratt Engine ↔ Statement Dispatch ↔ Error Recovery

#### Statement → Expression

`parse_stmt` delega a parser specifici (e.g., `parse_if` riga 203, `parse_while` riga 213) che chiamano `parse_expr(0)` per le sotto-espressioni.

Un'espressione **non** può contenere statement nel linguaggio .vn, quindi la profondità di alternanza stmt↔expr è al più 1.

#### Pratt: `nud` ↔ `led`

- `nud` (prefix/null denotation, riga 474): gestisce letterali, unari, raggruppamenti → consuma ≥1 token
- `led` (left denotation, riga 510): gestisce operatori binari, chiamate, accessi array → consuma ≥1 token

La **binding power** (file `precedence.rs`) è una funzione const $O(1)$ che restituisce coppie `(lbp, rbp)`.

**Proprietà di terminazione:** il ciclo `while lbp > min_bp` in `parse_expr_inner` converge perché:
- Ogni iterazione consuma ≥1 token via `led` → `advance()`
- Il numero di token è finito
- Il `MAX_RECURSION_DEPTH = 1000` previene stack overflow

#### Error Recovery

In caso di errore in `parse_stmt`:
```rust
if let Some(stmt) = self.parse_stmt() { ... }
else { self.advance(); }  // skip 1 token
```

Questo garantisce progresso anche in presenza di errori: ogni iterazione del loop principale consuma ≥1 token.

$$T_{\text{error\_recovery}} = O(t) \text{ nel caso peggiore (tutti errori)}$$

### 4.3 Intra-Semantic: TypeChecker ↔ SymbolTable ↔ Type Promotion Cache

#### Flusso di Interazione

```
visit_stmt/visit_expr
    ├─→ symbol_table.declare()     [write: O(1)]
    ├─→ symbol_table.lookup()      [read: O(d)]
    ├─→ promote_types()            [read: O(1) dalla cache]
    └─→ type_error_with_code()     [write: O(1)]
```

#### Pattern di Accesso alla Symbol Table

Per `visit_var_declaration` (riga 192):
1. Per ogni variabile $v_i$ nel multi-decl: `self.visit_expr(initializer_i)` → può triggerare lookup ricorsive
2. Type check del tipo dichiarato vs tipo dell'inizializzatore → `promote_types` $O(1)$
3. `declare_symbol(name, symbol)` → `HashMap::insert` $O(1)$ ammortizzato

Per `visit_function` (riga 235):
1. `symbol_table.declare(name, Function(...))` → $O(1)$
2. `symbol_table.push_scope(ScopeKind::Function)` → $O(1)$
3. Per ogni parametro: `declare_symbol(param)` → $O(1)$
4. `visit_statements(body)` → ricorsione nel corpo
5. `symbol_table.pop_scope()` → $O(1)$

**Costo aggregato per nodo:** dominato dalla lookup $O(d)$. Con $s$ nodi e $l$ lookup totali ($l \leq s$):

$$T_{\text{TC↔ST}} = \sum_{j=1}^{l} O(d_j) \leq l \cdot d \leq s \cdot d$$

#### Type Promotion Cache

`TYPE_PROMOTION_TABLE` (riga 87): `OnceLock<[u8; 100]>` — inizializzata una volta, poi $O(1)$ per accesso. Zero contesa: semantica write-once.

Anche `TYPE_PROMOTION_CACHE` (riga 84): `OnceLock<Mutex<HashMap<(Type, Type), Type>>>` — usata come cache di secondo livello con lock. In ambiente single-threaded (come il compilatore attuale), il costo del lock è $O(1)$.

### 4.4 Intra-IR Generator: ScopeManager ↔ CFG ↔ ControlFlowStack ↔ SSA

#### ScopeManager ↔ IR Generation

Per ogni variabile:
1. `scope_manager.add_symbol(name, value)` → `HashMap::insert` → $O(1)$
2. `scope_manager.resolve(name)` → traversal degli scope → $O(d)$

#### CFG: Bottleneck del Ricalcolo RPOT

Sequenza per un blocco `if-else`:
```
1. add_block("if_then")     → RPOT recompute O(B+E)
2. add_block("if_else")     → RPOT recompute O(B+E)
3. add_block("if_merge")    → RPOT recompute O(B+E)
4. connect_blocks(...)       → RPOT recompute O(B+E) × 4 archi
```

Per un singolo `if-else`: 7 ricalcoli RPOT → $O(7 \cdot (B + E))$.

**Costo accumulato per $B$ blocchi:**

$$T_{\text{RPOT\_accum}} = \sum_{i=1}^{B} O(i) = O\left(\frac{B(B+1)}{2}\right) = O(B^2)$$

#### ControlFlowStack per Loop

Per loop annidati di profondità $d_{\text{loop}}$:
- `push` su break_stack e continue_stack: $O(1)$ per livello
- `pop` alla chiusura del loop: $O(1)$
- Spazio: $O(d_{\text{loop}})$

#### SSA Transformer invocazione

`apply_ssa_transformation` è chiamata una volta per modulo (riga 291 in `generator.rs`):

```rust
if self.apply_ssa {
    self.apply_ssa_transformation(&mut module);
}
```

Itera su tutte le funzioni → per-function cost analizzato in §3.5.

### 4.5 Intra-SSA: Dominance ↔ Phi Insertion ↔ Renaming

#### Flusso di dati

```
DominanceInfo
    ├─→ idom[]              → usato da insert_phi_functions (dominance frontier)
    ├─→ dominance_frontiers  → usato da insert_phi_functions (worklist)
    └─→ dom_tree_children    → usato da rename_variables_recursive (DFS)
```

Le tre fasi sono **strettamente sequenziali** con dipendenze:
1. `compute_dominators` produce `idom` → richiesto da `compute_dominance_frontiers`
2. `compute_dominance_frontiers` produce `dominance_frontiers` → richiesto da `insert_phi_functions`
3. `insert_phi_functions` modifica il CFG → viene letto da `rename_variables_recursive`

**Nessun parallelismo possibile** tra queste fasi.

#### Interazione critica: Phi Insertion ↔ CFG

L'inserimento phi modifica `block.instructions` inserendo in testa:
```rust
block.instructions.insert(0, phi_inst);  // O(|I_b|) shift
```

Se un blocco riceve $\phi$ phi-functions e ha $I_b$ istruzioni:
- Prima phi: shift di $I_b$ elementi
- Seconda phi: shift di $I_b + 1$ elementi
- $k$-esima phi: shift di $I_b + k - 1$ elementi

$$T_{\text{phi\_insert\_block}} = \sum_{j=0}^{\phi-1} (I_b + j) = \phi \cdot I_b + \frac{\phi(\phi-1)}{2}$$

Aggregato su tutti i blocchi: $O(V \cdot I)$ nel caso peggiore.

#### Renaming ↔ Value Stacks

`value_stack: HashMap<Arc<str>, Vec<Value>>` gestisce lo scoping SSA:
- Push: $O(1)$ ammortizzato per ogni nuova definizione
- Pop: $O(1)$ in `pop_block_values`
- Lookup (`stack.last()`): $O(1)$

Ogni `replace_value_with_current_ssa` (riga 272): lookup nel value_stack → $O(1)$.

Chiamata per ogni operando di ogni istruzione → $O(I)$ totale.

### 4.6 Intra-DCE: Liveness ↔ DefUseChains ↔ Escape ↔ Decision

#### Build Phase (una tantum)

```
DefUseChains::new()
  ← LivenessAnalyzer::build_def_use_chains(func)    O(I)
  ← LivenessAnalyzer::compute_gen_kill_sets(func)    O(I)
EscapeAnalyzer::analyze(func)                         O(I)
```

#### Query Phase (per istruzione)

Per ogni istruzione, `identify_dead_instructions` (riga 160 in `optimizer.rs`):

1. `analyzer.is_instruction_dead(idx)` → `def_use_chains.get_defined_value` $O(1)$ + `has_uses` $O(1)$ → **$O(1)$**
2. `can_remove_instruction(...)` → pattern match su `InstructionKind`:
   - Per `Store`: `escape_analyzer.get_status(dest)` → HashMap lookup $O(1)$
   - Per `Load`: `has_uses_value(result)` → $O(1)$
   - Per `Alloca`: `has_uses_value(result)` → $O(1)$

$$T_{\text{decision\_per\_inst}} = O(1)$$

Totale decisioni: $O(I)$ per iterazione.

#### Removal Phase

`remove_instructions` raccoglie indici e rimuove in batch con iterazione inversa per preservare gli indici:

$$T_{\text{removal}} = O(I)$$ per iterazione (nel caso peggiore tutte le istruzioni sono morte).

### 4.7 Intra-SCCP: Worklist ↔ Lattice ↔ ExecutableEdges

Le tre strutture hanno interazione **simmetrica**:

| Operazione | Costo | Frequenza max |
|-----------|-------|--------------|
| `cfg_worklist.pop()` | $O(1)$ | $E$ volte |
| `cfg_worklist.push(edge)` | $O(1)$ | $E$ volte |
| `ssa_worklist.pop()` | $O(1)$ | $2I$ volte |
| `ssa_worklist.push(id)` | $O(1)$ | $2I$ volte |
| `lattice.get(id)` | $O(1)$ | $O(I + E)$ volte |
| `lattice.update(id, val)` | $O(1)$ | $2I$ volte (max 2 cambi per valore) |
| `executable_edges.mark(e)` | $O(1)$ | $E$ volte |
| `executable_edges.has_predecessor(b)` | $O(E)$ scan | Per funzione $O(B)$ volte |

**Nota:** `has_executable_predecessor` esegue un'iterazione lineare su `self.edges`:

```rust
pub fn has_executable_predecessor(&self, block_id: usize) -> bool {
    self.edges.iter().any(|e| e.to == block_id)
}
```

Costo: $O(E)$ per chiamata. Se chiamata $O(B)$ volte → contributo $O(B \cdot E)$.

Tuttavia, poiché $B \cdot E$ è tipicamente $O(B^2)$ per CFG sparsi, e $I$ è tipicamente $\gg B^2$, il costo dominante rimane $O(I + E)$.

---

## Tabella Riepilogativa Finale

| Componente | $T$ worst | $T$ medio | $S$ | Algoritmo |
|-----------|-----------|-----------|-----|-----------|
| **Lexer** | $\Theta(n)$ | $\Theta(n)$ | $\Theta(n)$ | DFA (logos) |
| **Parser** | $\Theta(t)$ | $\Theta(t)$ | $O(t)$ | Pratt + recursive descent |
| **Type Checker** | $O(s \cdot d)$ | $\Theta(s)$ | $O(V \cdot d)$ | DFS + symbol table stack |
| **IR Generator** | $O(s^2)$ | $O(s \cdot B)$ | $O(I + B + E)$ | 2-pass, CFG construction |
| **Dominance** | $O(B^2 \cdot E)$ | $\Theta(B + E)$ | $O(B + E)$ | Cooper-Harvey-Kennedy |
| **SSA** | $O(B^2(E+V))$ | $O(BV + I)$ | $O(B^2 + BV)$ | Cytron et al. |
| **DCE** | $O(EV + I)$ | $\Theta(I)$ | $O(BV + I)$ | Mark-sweep + liveness |
| **SCCP** | $\Theta(I + E)$ | $\Theta(I + E)$ | $O(I + E)$ | Wegman-Zadeck |
| **Codegen** | $\Theta(I)$ | $\Theta(I)$ | $\Theta(I)$ | Linear translation |

### Complessità End-to-End

**Worst case per modulo:**

$$T_{\text{totale}} = O\left(n + s \cdot d + \sum_{f=1}^{F} B_f^2 \cdot (E_f + V_f)\right)$$

**Caso pratico ($E = O(B)$, $d = O(1)$, dominance converge in 2-3 iter):**

$$T_{\text{pratico}} = \Theta\left(n + \sum_{f=1}^{F} (B_f \cdot V_f + I_f)\right)$$

**Spazio totale:**

$$S_{\text{totale}} = O\left(n + s + \sum_{f=1}^{F} (I_f + B_f^2 + B_f \cdot V_f)\right)$$

---

## Raccomandazioni di Ottimizzazione Basate sull'Analisi

| # | Problema | Complessità attuale | Fix proposto | Complessità risultante |
|---|---------|-------------------|-------------|----------------------|
| 1 | `add_block` → RPOT recompute | $O(B^2)$ accumulato | Dirty flag + lazy recompute | $O(B + E)$ una tantum |
| 2 | `find_block_by_label` lineare | $O(B)$ per chiamata | Indice `HashMap<Arc<str>, NodeIndex>` | $O(1)$ ammortizzato |
| 3 | AST clonazione in `main.rs` | $O(s)$ | Move semantics (`statements` → IR) | $O(1)$ |
| 4 | `instructions.insert(0, phi)` | $O(I_b)$ per inserimento | `VecDeque` o inserimento batch con reverse | $O(1)$ per inserimento |
| 5 | `has_executable_predecessor` scan | $O(E)$ per chiamata | `HashMap<usize, HashSet<usize>>` | $O(1)$ |
