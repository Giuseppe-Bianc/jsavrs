# Feature Specification: IR Type Promotion System Correction

**Feature Branch**: `002-the-intermediate-representation`  
**Created**: 29 settembre 2025  
**Status**: Draft  
**Input**: User description: "The intermediate representation (IR) currently exhibits incorrect or inconsistent type-promotion behavior when evaluating operations that involve operands of different types. This change request requires a systematic correction of the IR's type-promotion rules so that mixed-type operations are handled deterministically and according to a clearly specified promotion policy. The policy must define a type lattice (or ranking), a deterministic rule for selecting a common type (least upper bound), and explicit insertion of conversion (cast) nodes in the IR where promotion is required. The implementation should preserve numeric precision whenever possible, correctly handle signed/unsigned interactions, and include regression tests covering representative cases (e.g., int + float, int32 + int64, signed/unsigned mixes, and special floating-point cases such as NaN and infinities). Finally, update the documentation and the code-generation backend to reflect the corrected promotion semantics."

## Execution Flow (main)
```
1. Parse user description from Input
   → Feature: Systematic correction of IR type-promotion rules for mixed-type operations
2. Extract key concepts from description
   → Actors: IR generator, Type promotion system, Code generation backend
   → Actions: Type promotion, Cast insertion, Deterministic type resolution
   → Data: Type lattice/ranking, Promotion policies, Mixed-type operations
   → Constraints: Deterministic behavior, Precision preservation, Standard compliance
3. For each unclear aspect:
   → All major requirements are clearly specified in the description
4. Fill User Scenarios & Testing section
   → User flow: Compiler processes mixed-type expressions with consistent behavior
5. Generate Functional Requirements
   → Deterministic promotion rules, explicit cast insertion, comprehensive testing
6. Identify Key Entities
   → Type lattice, Promotion policies, IR nodes, Cast operations
7. Run Review Checklist
   → Comprehensive specification with clear requirements
8. Return: SUCCESS (spec ready for planning)
```

---

## ⚡ Quick Guidelines
- ✅ Focus on WHAT users need and WHY
- ❌ Avoid HOW to implement (no tech stack, APIs, code structure)
- 👥 Written for business stakeholders, not developers

---

## Clarifications

### Session 2025-09-29
- Q: What should be the promotion rule for integer-float mixing (i32 + f32)? → A: Floating-point types take precedence (i32 + f32 → f32)
- Q: When mixing signed and unsigned integers of same width (i32 + u32), what should be the promotion behavior? → A: Promote to next larger signed type (i32 + u32 → i64)
- Q: When type conversion could result in data loss (e.g., f64 to i32), what should be the system's behavior? → A: Insert runtime checks with configurable behavior
- Q: What should be the performance impact tolerance for the new type promotion system? → A: Focus on correctness over performance initially
- Q: When corrected type promotion rules conflict with existing code expectations, what should be the compatibility approach? → A: Gradual transition with warnings

## User Scenarios & Testing *(mandatory)*

### Primary User Story
As a developer using the jsavrs compiler, when I write expressions that involve operands of different types (such as mixing integers with floats, or different integer sizes), I expect the compiler's intermediate representation to handle type promotion in a consistent, deterministic, and mathematically sound manner that preserves precision whenever possible and follows clearly documented rules.

### Acceptance Scenarios
1. **Given** an expression with mixed integer and floating-point types, **When** the IR generator processes this expression, **Then** the promotion must follow a deterministic type lattice with floating-point types taking precedence while preserving precision
2. **Given** mixed integer operations of different sizes (e.g., int32 + int64), **When** the IR processes the operation, **Then** the result must promote to the wider integer type with appropriate signedness handling
3. **Given** signed and unsigned integer mixing, **When** type promotion occurs, **Then** the system must handle the interaction according to well-defined rules that prevent unexpected behavior
4. **Given** special floating-point values (NaN, infinity), **When** involved in mixed-type operations, **Then** the system must handle these cases correctly according to IEEE standards
5. **Given** any mixed-type operation requiring promotion, **When** the IR is generated, **Then** explicit cast nodes must be inserted where type conversion is required
6. **Given** updated promotion rules, **When** code generation occurs, **Then** the backend must correctly implement the new semantics

### Edge Cases
- How does the system handle operations between the smallest and largest representable types?
- What happens when precision loss is unavoidable during type promotion?
- How are overflow and underflow conditions handled during promotion?
- What happens with user-defined types that have conversion operators?
- How does the system handle promotion in complex expressions with multiple operators?

## Requirements *(mandatory)*

### Functional Requirements
- **FR-001**: System MUST define a complete type lattice (ranking) that determines promotion precedence for all supported numeric types, with floating-point types taking precedence over integer types in mixed operations
- **FR-002**: System MUST implement deterministic rules for selecting a common type (least upper bound) for any pair of operand types
- **FR-003**: System MUST insert explicit conversion (cast) nodes in the IR wherever type promotion is required
- **FR-004**: System MUST preserve numeric precision whenever mathematically possible during type promotion
- **FR-005**: System MUST handle signed/unsigned integer interactions by promoting to the next larger signed type when operands have the same width (e.g., i32 + u32 → i64)
- **FR-006**: System MUST correctly process special floating-point cases (NaN, infinity, denormalized numbers) during mixed-type operations
- **FR-007**: System MUST provide comprehensive regression test coverage for representative mixed-type operation cases
- **FR-008**: System MUST update all documentation to reflect the corrected promotion semantics
- **FR-009**: System MUST ensure the code-generation backend correctly implements the new promotion rules
- **FR-010**: System MUST handle complex expressions with multiple mixed-type operations consistently
- **FR-011**: System MUST provide clear error messages when type promotion cannot be performed safely
- **FR-012**: System MUST implement a gradual transition approach with compiler warnings when corrected type promotion rules conflict with existing code expectations
- **FR-013**: System MUST insert runtime checks for potential data loss during type conversion with configurable behavior (saturate, wrap, trap)
- **FR-014**: System MUST prioritize correctness over performance in the initial implementation, with performance optimization as a subsequent phase
- **FR-015**: System MUST provide clear compiler warnings during the transition period to help developers identify code that relies on the old incorrect promotion behavior

### Key Entities *(include if feature involves data)*
- **Type Lattice**: A hierarchical ordering of all numeric types that defines promotion precedence and relationships between types
- **Promotion Policy**: The algorithmic rules and decision logic for determining the target type when multiple types are involved in an operation
- **Cast Nodes**: Explicit IR nodes that represent type conversion operations, inserted automatically when promotion requires actual data transformation
- **Mixed-Type Operations**: Binary and n-ary operations involving operands of different numeric types that require promotion resolution
- **Precision Rules**: Guidelines that govern when and how numeric precision should be preserved or managed during type promotion
- **Backend Integration**: The interface between the corrected IR type system and the code generation phases that produce target machine code

---

## Review & Acceptance Checklist
*GATE: Automated checks run during main() execution*

### Content Quality
- [x] No implementation details (languages, frameworks, APIs)
- [x] Focused on user value and business needs
- [x] Written for non-technical stakeholders
- [x] All mandatory sections completed

### Requirement Completeness
- [x] No [NEEDS CLARIFICATION] markers remain
- [x] Requirements are testable and unambiguous  
- [x] Success criteria are measurable
- [x] Scope is clearly bounded
- [x] Dependencies and assumptions identified

### Community Guidelines
- [x] Specifications promote collaboration and respect among contributors
- [x] Requirements consider shared learning opportunities
- [x] Community impact is considered in feature design

---

## Execution Status
*Updated by main() during processing*

- [x] User description parsed
- [x] Key concepts extracted
- [x] Ambiguities marked
- [x] User scenarios defined
- [x] Requirements generated
- [x] Entities identified
- [x] Review checklist passed

---
