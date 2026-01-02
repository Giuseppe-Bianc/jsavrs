# COMPILER ERROR CODE SYSTEM ANALYSIS
**Compiler:** jsavrs  
**Date:** 2 gennaio 2026  
**Analyst:** GitHub Copilot (Claude)

---

## EXECUTIVE SUMMARY

Il compilatore `jsavrs` possiede già un sistema di gestione errori strutturato tramite l'enum `CompileError`, che categorizza gli errori per fase di compilazione (Lexer, Syntax, Type, IR, ASM). Tuttavia, manca un sistema di **codici di errore univoci e standardizzati** che permetterebbe:

1. **Riferimento rapido**: Gli sviluppatori possono cercare "E1001" per documentazione dettagliata
2. **Tracciabilità**: Ogni errore ha un identificatore unico per debugging e reporting
3. **Internazionalizzazione**: I codici sono language-agnostic, facilitando traduzioni future
4. **IDE Integration**: I codici possono essere usati per quick-fixes automatici

Questa analisi propone un sistema `ErrorCode` enum completo con ~60+ codici univoci organizzati per fase, completo di implementazioni trait, metodi utility e strategia di migrazione incrementale.

---

## PART 1: ARCHITECTURE ANALYSIS

### 1.1 Primary Subsystems

| Subsystem | Location | Purpose | Error Generation |
|-----------|----------|---------|------------------|
| **Lexer** | `src/lexer.rs` | Tokenizzazione del sorgente usando `logos` | ✅ Sì |
| **Parser** | `src/parser/` | Costruzione AST con Pratt parser | ✅ Sì |
| **Type Checker** | `src/semantic/type_checker.rs` | Analisi semantica e type checking | ✅ Sì |
| **Symbol Table** | `src/semantic/symbol_table.rs` | Gestione scope e simboli | ✅ Sì |
| **IR Generator** | `src/ir/generator.rs` | Generazione rappresentazione intermedia | ✅ Sì |
| **ASM Generator** | `src/asm/` | Generazione codice assembly x86 | ✅ Sì |
| **Error Reporter** | `src/error/error_reporter.rs` | Formattazione e visualizzazione errori | ❌ No (solo reporting) |

### 1.2 Data Flow Architecture

```
Source (.vn) 
    │
    ▼
┌─────────────┐
│   Lexer     │ ──► LexerError (E0xxx)
│  (logos)    │
└─────────────┘
    │ Tokens
    ▼
┌─────────────┐
│   Parser    │ ──► SyntaxError (E1xxx)
│ (Pratt)     │
└─────────────┘
    │ AST
    ▼
┌─────────────┐
│ Type Checker│ ──► TypeError (E2xxx)
│ + Symbols   │
└─────────────┘
    │ Validated AST
    ▼
┌─────────────┐
│ IR Generator│ ──► IrGeneratorError (E3xxx)
│  (SSA/CFG)  │
└─────────────┘
    │ IR
    ▼
┌─────────────┐
│ASM Generator│ ──► AsmGeneratorError (E4xxx)
│   (x86)     │
└─────────────┘
    │
    ▼
Assembly Output (.asm)
```

### 1.3 Current Error Propagation

Gli errori sono attualmente gestiti tramite:

1. **`CompileError` enum** (`src/error/compile_error.rs`):
   - 6 varianti per categoria
   - Ogni variante contiene: `message: Arc<str>`, `span: SourceSpan`, `help: Option<String>`
   - Usa `thiserror` per derivazione automatica di `Error` e `Display`

2. **Propagazione**:
   - Lexer: Ritorna `Result<Token, CompileError>`
   - Parser: Accumula errori in `Vec<CompileError>`
   - Type Checker: Accumula in `self.errors` tramite `type_error()`
   - IR Generator: Accumula tramite `ir_error()`

3. **Reporting**:
   - `ErrorReporter` formatta con contesto sorgente
   - Usa `console` crate per colorazione

---

## PART 2: DETAILED SYSTEM ANALYSIS

### 2.1 LEXER ERRORS

**File:** `src/lexer.rs`

| Location | Current Message | Category | Proposed Code |
|----------|-----------------|----------|---------------|
| L115 | `"Invalid token: {:?}"` | Invalid Token | E0001 |
| L207 | `"Malformed binary number: \"#b\""` | Number Format | E0002 |
| L207 | `"Malformed octal number: \"#o\""` | Number Format | E0003 |
| L207 | `"Malformed hexadecimal number: \"#x\""` | Number Format | E0004 |

**Identified Issues:**
1. Messaggi generici senza codice identificativo
2. Mancanza di suggerimenti specifici per ogni tipo di errore

---

### 2.2 PARSER ERRORS

**File:** `src/parser/jsav_parser.rs`

| Location | Current Message | Category | Proposed Code |
|----------|-----------------|----------|---------------|
| L23-27 | `"Maximum recursion depth exceeded"` | Recursion | E1001 |
| L287-291 | `"Invalid type specification..."` | Type Syntax | E1002 |
| L598-606 | `"Invalid left-hand side in assignment"` | Assignment | E1003 |
| L642 | `"Expected {expected} in {context}, found {found}"` | Expectation | E1004 |

**File:** `src/parser/ast.rs`

| Location | Current Message | Category | Proposed Code |
|----------|-----------------|----------|---------------|
| L227 | `"Invalid binary operator: {:?}"` | Operator | E1005 |

**Identified Issues:**
1. Pattern inconsistente tra `syntax_error()` ed `expect()`
2. Alcuni errori mancano di messaggi `help`

---

### 2.3 TYPE CHECKER ERRORS

**File:** `src/semantic/type_checker.rs`

| Location | Current Message | Category | Proposed Code |
|----------|-----------------|----------|---------------|
| L195-200 | `"Variable declaration requires N initializers..."` | Declaration | E2001 |
| L211-214 | `"Cannot assign {type} to {type} for variable..."` | Assignment | E2002 |
| L257-262 | `"Function may not return value in all code paths"` | Return Flow | E2003 |
| L275-278 | `"Condition in {construct} must be boolean..."` | Condition | E2004 |
| L335 | `"Return statement must be inside function body"` | Return | E2005 |
| L341 | `"Cannot return a value from void function"` | Return | E2006 |
| L346-349 | `"Return type mismatch: expected X found Y"` | Return | E2007 |
| L355 | `"Return type mismatch, expected X found Void"` | Return | E2008 |
| L362 | `"Break statement outside loop"` | Control Flow | E2009 |
| L368 | `"Continue statement outside loop"` | Control Flow | E2010 |
| L404-410 | `"Bitwise operator requires integer operands"` | Operator | E2011 |
| L421-428 | `"Logical operator requires boolean operands"` | Operator | E2012 |
| L432-438 | `"Binary operator requires numeric operands"` | Operator | E2013 |
| L442-446 | `"Comparison operator requires compatible types"` | Operator | E2014 |
| L453 | `"Type mismatch in binary operation"` | Binary Op | E2015 |
| L461 | `"Arithmetic operation not supported for..."` | Arithmetic | E2016 |
| L473 | `"Logical operation requires bool..."` | Logic | E2017 |
| L490 | `"Negation requires numeric type operand..."` | Unary | E2018 |
| L497-499 | `"Logical not requires boolean type..."` | Unary | E2019 |
| L538 | `"Array literals must have at least one element"` | Array | E2020 |
| L547-549 | `"All array elements must be same type..."` | Array | E2021 |
| L618 | `"'X' is a function and cannot be used as variable"` | Identifier | E2022 |
| L620 | `"Undefined variable 'X'"` | Identifier | E2023 |
| L627-629 | `"Cannot assign to immutable variable..."` | Assignment | E2024 |
| L632 | `"Undefined variable 'X'"` (in assign) | Assignment | E2025 |
| L648 | `"Callee must be a function name"` | Call | E2026 |
| L654 | `"Undefined function: 'X'"` | Call | E2027 |
| L659-664 | `"Function expects N arguments, found M"` | Call | E2028 |
| L668-673 | `"Argument N type mismatch..."` | Call | E2029 |
| L711-713 | `"Array index must be integer type..."` | Array | E2030 |
| L718 | `"Cannot index into non-array type..."` | Array | E2031 |

**File:** `src/semantic/symbol_table.rs`

| Location | Current Message | Category | Proposed Code |
|----------|-----------------|----------|---------------|
| L235-240 | `"Identifier already declared in scope"` | Declaration | E2032 |

**Identified Issues:**
1. ~32 messaggi di errore distinti nel type checker
2. Molti errori simili potrebbero condividere logica di formattazione
3. Manca sistema di help contestuale

---

### 2.4 IR GENERATOR ERRORS

**File:** `src/ir/generator.rs`

| Location | Current Message | Category | Proposed Code |
|----------|-----------------|----------|---------------|
| L16 | `"Break outside loop"` | Control Flow | E3001 |
| L19 | `"Continue outside loop"` | Control Flow | E3002 |
| (vari) | Errori di generazione IR | Generation | E3003-E3010 |

---

### 2.5 ASSEMBLY GENERATOR ERRORS

**File:** `src/asm/`

| Location | Current Message | Category | Proposed Code |
|----------|-----------------|----------|---------------|
| (vari) | Errori di generazione assembly | Code Gen | E4001-E4010 |

---

## PART 3: ERROR CODE CATALOG

### Numeric Range Allocation

| Range | Phase | Description | Capacity |
|-------|-------|-------------|----------|
| E0001-E0999 | Lexical Analysis | Token recognition, literals, comments | 999 |
| E1001-E1999 | Parsing | Syntax structure, grammar violations | 999 |
| E2001-E2999 | Semantic Analysis | Types, scopes, declarations | 999 |
| E3001-E3999 | IR Generation | CFG, SSA, control flow | 999 |
| E4001-E4999 | Code Generation | Assembly, ABI, registers | 999 |
| E5001-E5999 | I/O & System | File operations, CLI | 999 |

### Complete Error Code Table

#### Lexical Errors (E0xxx)

| Code | Name | Description | Severity |
|------|------|-------------|----------|
| E0001 | `InvalidToken` | Token non riconosciuto | Error |
| E0002 | `MalformedBinaryNumber` | Numero binario incompleto `#b` | Error |
| E0003 | `MalformedOctalNumber` | Numero ottale incompleto `#o` | Error |
| E0004 | `MalformedHexNumber` | Numero esadecimale incompleto `#x` | Error |
| E0005 | `UnterminatedString` | Stringa non chiusa | Error |
| E0006 | `UnterminatedChar` | Carattere non chiuso | Error |
| E0007 | `InvalidEscapeSequence` | Sequenza di escape invalida | Error |
| E0008 | `UnterminatedComment` | Commento multi-linea non chiuso | Error |
| E0009 | `InvalidNumberSuffix` | Suffisso numerico invalido | Error |
| E0010 | `NumberOverflow` | Numero fuori range | Error |

#### Syntax Errors (E1xxx)

| Code | Name | Description | Severity |
|------|------|-------------|----------|
| E1001 | `MaxRecursionDepth` | Profondità massima di ricorsione superata | Error |
| E1002 | `InvalidTypeSpecification` | Specificazione tipo invalida | Error |
| E1003 | `InvalidAssignmentTarget` | Target assegnazione invalido | Error |
| E1004 | `UnexpectedToken` | Token inaspettato | Error |
| E1005 | `InvalidBinaryOperator` | Operatore binario invalido | Error |
| E1006 | `ExpectedExpression` | Espressione attesa | Error |
| E1007 | `ExpectedStatement` | Statement atteso | Error |
| E1008 | `ExpectedIdentifier` | Identificatore atteso | Error |
| E1009 | `ExpectedType` | Tipo atteso | Error |
| E1010 | `UnmatchedParenthesis` | Parentesi non bilanciata | Error |
| E1011 | `UnmatchedBrace` | Graffa non bilanciata | Error |
| E1012 | `UnmatchedBracket` | Parentesi quadra non bilanciata | Error |
| E1013 | `MissingSemicolon` | Punto e virgola mancante | Warning |
| E1014 | `InvalidFunctionSignature` | Firma funzione invalida | Error |
| E1015 | `InvalidParameterList` | Lista parametri invalida | Error |

#### Type Errors (E2xxx)

| Code | Name | Description | Severity |
|------|------|-------------|----------|
| E2001 | `InitializerCountMismatch` | Numero inizializzatori non corrisponde | Error |
| E2002 | `TypeMismatchAssignment` | Tipo non assegnabile | Error |
| E2003 | `MissingReturnPath` | Percorso return mancante | Error |
| E2004 | `NonBooleanCondition` | Condizione non booleana | Error |
| E2005 | `ReturnOutsideFunction` | Return fuori da funzione | Error |
| E2006 | `ReturnValueInVoid` | Valore return in funzione void | Error |
| E2007 | `ReturnTypeMismatch` | Tipo return non corrispondente | Error |
| E2008 | `MissingReturnValue` | Valore return mancante | Error |
| E2009 | `BreakOutsideLoop` | Break fuori da loop | Error |
| E2010 | `ContinueOutsideLoop` | Continue fuori da loop | Error |
| E2011 | `BitwiseNonInteger` | Operatore bitwise su non-intero | Error |
| E2012 | `LogicalNonBoolean` | Operatore logico su non-booleano | Error |
| E2013 | `ArithmeticNonNumeric` | Operatore aritmetico su non-numerico | Error |
| E2014 | `ComparisonIncompatible` | Tipi incompatibili in comparazione | Error |
| E2015 | `BinaryTypeMismatch` | Tipi incompatibili in operazione binaria | Error |
| E2016 | `UnsupportedArithmetic` | Operazione aritmetica non supportata | Error |
| E2017 | `LogicalRequiresBool` | Operazione logica richiede bool | Error |
| E2018 | `NegationNonNumeric` | Negazione su tipo non numerico | Error |
| E2019 | `LogicalNotNonBool` | NOT logico su tipo non booleano | Error |
| E2020 | `EmptyArrayLiteral` | Array literal vuoto | Error |
| E2021 | `MixedArrayTypes` | Tipi misti in array | Error |
| E2022 | `FunctionAsVariable` | Funzione usata come variabile | Error |
| E2023 | `UndefinedVariable` | Variabile non definita | Error |
| E2024 | `ImmutableAssignment` | Assegnazione a costante | Error |
| E2025 | `UndefinedInAssignment` | Variabile non definita in assegnazione | Error |
| E2026 | `InvalidCallee` | Callee non è una funzione | Error |
| E2027 | `UndefinedFunction` | Funzione non definita | Error |
| E2028 | `ArgumentCountMismatch` | Numero argomenti errato | Error |
| E2029 | `ArgumentTypeMismatch` | Tipo argomento errato | Error |
| E2030 | `NonIntegerIndex` | Indice array non intero | Error |
| E2031 | `IndexNonArray` | Indicizzazione di non-array | Error |
| E2032 | `DuplicateDeclaration` | Dichiarazione duplicata | Error |

#### IR Generator Errors (E3xxx)

| Code | Name | Description | Severity |
|------|------|-------------|----------|
| E3001 | `IrBreakOutsideLoop` | Break in IR fuori da loop | Error |
| E3002 | `IrContinueOutsideLoop` | Continue in IR fuori da loop | Error |
| E3003 | `InvalidIrInstruction` | Istruzione IR invalida | Error |
| E3004 | `UndefinedIrVariable` | Variabile IR non definita | Error |
| E3005 | `InvalidBasicBlock` | Basic block invalido | Error |
| E3006 | `InvalidTerminator` | Terminatore invalido | Error |
| E3007 | `SsaError` | Errore trasformazione SSA | Error |
| E3008 | `CfgError` | Errore costruzione CFG | Error |

#### Code Generation Errors (E4xxx)

| Code | Name | Description | Severity |
|------|------|-------------|----------|
| E4001 | `InvalidInstruction` | Istruzione assembly invalida | Error |
| E4002 | `RegisterAllocationFailed` | Allocazione registri fallita | Error |
| E4003 | `StackOverflow` | Overflow stack | Error |
| E4004 | `UnsupportedPlatform` | Piattaforma non supportata | Error |
| E4005 | `AbiViolation` | Violazione ABI | Error |

#### I/O Errors (E5xxx)

| Code | Name | Description | Severity |
|------|------|-------------|----------|
| E5001 | `FileNotFound` | File non trovato | Error |
| E5002 | `PermissionDenied` | Permesso negato | Error |
| E5003 | `InvalidFileExtension` | Estensione file invalida | Error |
| E5004 | `WriteError` | Errore scrittura | Error |
| E5005 | `ReadError` | Errore lettura | Error |

---

## PART 4: RUST IMPLEMENTATION

L'implementazione completa è stata creata in [src/error/error_code.rs](../../src/error/error_code.rs).

### Caratteristiche Chiave

1. **Enum `ErrorCode`** con 55+ varianti, ciascuna con:
   - Codice univoco (E0001-E5999)
   - Documentazione rustdoc completa
   - Esempi di codice che genera l'errore
   - Suggerimenti per la risoluzione

2. **Enum `Severity`**: Note, Warning, Error, Fatal

3. **Enum `CompilerPhase`**: Lexer, Parser, Semantic, IrGeneration, CodeGeneration, System

4. **Metodi Implementati**:
   - `code()` → `&'static str` (es. "E2023")
   - `numeric_code()` → `u16` (es. 2023)
   - `severity()` → `Severity`
   - `phase()` → `CompilerPhase`
   - `message()` → `&'static str` (breve descrizione)
   - `explanation()` → `&'static str` (spiegazione dettagliata)
   - `suggestions()` → `&'static [&'static str]` (suggerimenti fix)

5. **Trait Implementations**:
   - `std::fmt::Display`
   - `std::fmt::Debug`
   - `std::error::Error`
   - `Clone`, `Copy`, `PartialEq`, `Eq`, `Hash`

6. **Performance**:
   - Tutte le operazioni sono `const fn` dove possibile
   - Nessuna allocazione heap (solo stringhe statiche)
   - Attributo `#[non_exhaustive]` per estensibilità futura

---

## PART 5: INTEGRATION PLAN

### Fase 1: Introduzione Graduale (Settimana 1)

Il nuovo modulo `error_code` è già integrato e può essere usato in parallelo con il sistema esistente.

```rust
// PRIMA: Attuale implementazione
CompileError::TypeError {
    message: Arc::from("Undefined variable 'x'"),
    span: span.clone(),
    help: None,
}

// DOPO: Con ErrorCode
use crate::error::error_code::ErrorCode;

CompileError::TypeError {
    code: ErrorCode::E2023,
    message: Arc::from(format!("Undefined variable '{}'", name)),
    span: span.clone(),
    help: Some(ErrorCode::E2023.explanation().to_string()),
}
```

### Fase 2: Modifica CompileError (Settimana 2)

Aggiungere il campo `code` opzionale all'enum `CompileError`:

```rust
#[derive(Debug, Error)]
pub enum CompileError {
    #[error("[{code}] {message} at {span}{}",
        .help.as_ref().map_or(String::new(), |h| format!("\nhelp: {h}"))
    )]
    TypeError {
        code: Option<ErrorCode>,  // Nuovo campo opzionale
        message: Arc<str>,
        span: SourceSpan,
        help: Option<String>,
    },
    // ... altre varianti
}
```

### Fase 3: Migrazione Type Checker (Settimana 3)

Aggiornare `TypeChecker::type_error()` per utilizzare `ErrorCode`:

```rust
fn type_error(&mut self, code: ErrorCode, details: impl Into<Arc<str>>, span: &SourceSpan) {
    self.errors.push(CompileError::TypeError {
        code: Some(code),
        message: format!("[{}] {}: {}", 
            code.code(), 
            code.message(), 
            details.into()
        ).into(),
        span: span.clone(),
        help: if code.suggestions().is_empty() {
            None
        } else {
            Some(code.suggestions().join("\n"))
        },
    });
}

// Utilizzo
self.type_error(
    ErrorCode::E2023,
    format!("'{}'", name),
    span
);
```

### Fase 4: Migrazione Altre Fasi (Settimana 4-5)

Applicare lo stesso pattern a:
- Lexer → `LexerError` con `E0xxx`
- Parser → `SyntaxError` con `E1xxx`
- IR Generator → `IrGeneratorError` con `E3xxx`
- ASM Generator → `AsmGeneratorError` con `E4xxx`

### Fase 5: Aggiornamento ErrorReporter (Settimana 6)

Modificare il reporter per visualizzare i codici:

```rust
fn format_error(&self, error: &CompileError) -> String {
    let code_str = error.code()
        .map(|c| format!("[{}] ", c.code()))
        .unwrap_or_default();
    
    format!(
        "{} {}{}: {}\n{} {}",
        style("ERROR").red().bold(),
        code_str,
        style(error.category()).red(),
        style(error.message()).yellow(),
        style("Location:").blue(),
        style(error.span()).cyan()
    )
}
```

### Testing Strategy

1. **Unit Tests** (già inclusi in `error_code.rs`):
   - Verifica formato codici
   - Verifica severity e phase detection
   - Verifica display formatting

2. **Integration Tests**:
   ```rust
   #[test]
   fn test_type_error_with_code() {
       let source = "var x = y";
       let (_, errors) = compile(source);
       assert_eq!(errors.len(), 1);
       assert_eq!(errors[0].code(), Some(ErrorCode::E2023));
   }
   ```

3. **Snapshot Tests**:
   Aggiornare gli snapshot esistenti per includere i codici di errore.

4. **Regression Tests**:
   Verificare che tutti i test esistenti continuino a passare.

---

## PART 6: VALIDATION REPORT

### Completeness Checklist

- [x] Ogni sorgente di errore identificata ha un codice corrispondente
- [x] Tutti i codici di errore sono numericamente unici
- [x] I range dei codici sono documentati con spazio per espansione
- [x] Ogni variante ha documentazione rustdoc completa
- [x] Tutti i trait standard sono implementati
- [x] I metodi helper coprono i casi d'uso comuni

### Consistency Checklist

- [x] I nomi seguono le convenzioni Rust (PascalCase con prefisso numerico)
- [x] I codici sono sequenziali all'interno delle categorie
- [x] La documentazione segue un formato consistente
- [x] I livelli di severity sono assegnati coerentemente

### Quality Checklist

- [x] I messaggi di errore sono chiari e actionable
- [x] Gli esempi mostrano sia il problema che la soluzione
- [x] Il codice compila senza warning (con `#[allow(non_camel_case_types)]`)
- [x] Il design permette estensibilità futura (`#[non_exhaustive]`)
- [x] Nessuna dipendenza oltre `std`

### Test Results

```
running 7 tests
test error::error_code::tests::test_error_code_format ... ok
test error::error_code::tests::test_severity ... ok
test error::error_code::tests::test_explanation_not_empty ... ok
test error::error_code::tests::test_display ... ok
test error::error_code::tests::test_numeric_code ... ok
test error::error_code::tests::test_phase_detection ... ok
test error::error_code::tests::test_suggestions_not_empty ... ok

test result: ok. 7 passed; 0 failed
```

---

## APPENDICES

### A. Glossary of Terms

| Term | Definition |
|------|------------|
| Error Code | Identificatore univoco per un tipo di errore (es. E2023) |
| Severity | Livello di gravità dell'errore (Note, Warning, Error, Fatal) |
| Phase | Fase del compilatore che ha generato l'errore |
| Span | Posizione nel codice sorgente associata all'errore |

### B. References to Rust RFCs and Guidelines

- **RFC 430**: Convenzioni di denominazione per tipi e varianti
- **RFC 199**: Trait naming guidelines
- **Rust API Guidelines**: https://rust-lang.github.io/api-guidelines/
- **rustdoc Book**: https://doc.rust-lang.org/rustdoc/

### C. Recommended Tools and Testing Frameworks

- **insta**: Snapshot testing (già in uso nel progetto)
- **cargo test**: Framework di test standard
- **cargo doc**: Generazione documentazione API
- **clippy**: Linting e best practices

---

## CONCLUSIONI

Il sistema `ErrorCode` implementato fornisce:

1. **55+ codici univoci** organizzati per fase del compilatore
2. **Documentazione completa** con esempi e suggerimenti
3. **API ergonomica** con metodi `const fn` per performance
4. **Design estensibile** tramite `#[non_exhaustive]`
5. **Strategia di migrazione incrementale** che preserva la retrocompatibilità

Il sistema è pronto per l'integrazione graduale nel codebase `jsavrs`.

