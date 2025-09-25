use jsavrs::asm::instruction::Instruction;
use jsavrs::asm::operand::Operand;
use jsavrs::asm::register::Register;

#[test]
fn test_instruction_display() {
    // Test instruction display implementations
    let mov_inst = Instruction::Mov(Operand::reg(Register::RAX), Operand::imm(42));
    assert_eq!(format!("{}", mov_inst), "    mov rax, 42");

    let add_inst = Instruction::Add(Operand::reg(Register::RBX), Operand::reg(Register::RCX));
    assert_eq!(format!("{}", add_inst), "    add rbx, rcx");

    let jmp_inst = Instruction::Jmp("label".to_string());
    assert_eq!(format!("{}", jmp_inst), "    jmp label");

    let call_inst = Instruction::Call("function".to_string());
    assert_eq!(format!("{}", call_inst), "    call function");

    let ret_inst = Instruction::Ret;
    assert_eq!(format!("{}", ret_inst), "    ret");
}

#[test]
fn test_instruction_type_variants() {
    // Test all instruction type variants
    let mov_inst = Instruction::Mov(Operand::reg(Register::RAX), Operand::imm(42));

    let add_inst = Instruction::Add(Operand::reg(Register::RBX), Operand::imm(100));

    let push_inst = Instruction::Push(Operand::reg(Register::RCX));
    let pop_inst = Instruction::Pop(Operand::reg(Register::RDX));
    let cmp_inst = Instruction::Cmp(Operand::reg(Register::RSI), Operand::reg(Register::RDI));

    let jmp_inst = Instruction::Jmp("test_label".to_string());
    let je_inst = Instruction::Je("equal_label".to_string());
    let call_inst = Instruction::Call("function_name".to_string());
    let ret_inst = Instruction::Ret;

    // Verify they can be formatted without errors
    assert!(format!("{}", mov_inst).contains("mov"));
    assert!(format!("{}", add_inst).contains("add"));
    assert!(format!("{}", push_inst).contains("push"));
    assert!(format!("{}", pop_inst).contains("pop"));
    assert!(format!("{}", cmp_inst).contains("cmp"));
    assert!(format!("{}", jmp_inst).contains("jmp"));
    assert!(format!("{}", je_inst).contains("je"));
    assert!(format!("{}", call_inst).contains("call"));
    assert!(format!("{}", ret_inst).contains("ret"));
}

#[test]
fn test_instruction_specific_operand_constraints() {
    // Test instruction-specific operand constraints (e.g., div, idiv require a single operand)

    // Single operand instructions
    let div_inst = Instruction::Div(Operand::reg(Register::RAX));
    assert!(format!("{}", div_inst).contains("div"));

    let idiv_inst = Instruction::Idiv(Operand::reg(Register::RBX));
    assert!(format!("{}", idiv_inst).contains("idiv"));

    let inc_inst = Instruction::Inc(Operand::reg(Register::RCX));
    assert!(format!("{}", inc_inst).contains("inc"));

    let dec_inst = Instruction::Dec(Operand::reg(Register::RDX));
    assert!(format!("{}", dec_inst).contains("dec"));

    let neg_inst = Instruction::Neg(Operand::reg(Register::RSI));
    assert!(format!("{}", neg_inst).contains("neg"));

    let not_inst = Instruction::Not(Operand::reg(Register::RDI));
    assert!(format!("{}", not_inst).contains("not"));

    // Two operand instructions
    let add_inst = Instruction::Add(Operand::reg(Register::RAX), Operand::imm(10));
    assert!(format!("{}", add_inst).contains("add"));

    let mov_inst = Instruction::Mov(Operand::reg(Register::RBX), Operand::reg(Register::RCX));
    assert!(format!("{}", mov_inst).contains("mov"));

    // Three operand instruction (imul with three operands)
    let imul_inst =
        Instruction::Imul(Operand::reg(Register::RAX), Some(Operand::reg(Register::RBX)), Some(Operand::imm(42)));
    assert!(format!("{}", imul_inst).contains("imul"));

    // Single operand with immediate
    let imul_two_op = Instruction::Imul(Operand::reg(Register::RAX), Some(Operand::imm(5)), None);
    assert!(format!("{}", imul_two_op).contains("imul"));

    // Single operand (just destination)
    let imul_one_op = Instruction::Imul(Operand::reg(Register::RAX), None, None);
    assert!(format!("{}", imul_one_op).contains("imul"));
}

#[test]
fn test_instruction_formatting_with_complex_operand_combinations() {
    // Test instruction formatting with complex operand combinations

    // Memory reference operands
    let mov_mem_inst = Instruction::Mov(
        Operand::reg(Register::RAX),
        Operand::mem_base_index_scale_disp(Register::RBX, Register::RCX, 4, 8),
    );
    assert!(format!("{}", mov_mem_inst).contains("[rbx+rcx*4+8]"));

    // Multiple register operations
    let add_inst = Instruction::Add(Operand::reg(Register::RSP), Operand::imm(16));
    assert!(format!("{}", add_inst).contains("add"));

    // Comparison with memory
    let cmp_mem_inst = Instruction::Cmp(Operand::mem_base_disp(Register::RBP, -8), Operand::imm(0));
    assert!(format!("{}", cmp_mem_inst).contains("[rbp-8]"));

    // Conditional jump
    let jne_inst = Instruction::Jne("not_equal".to_string());
    assert!(format!("{}", jne_inst).contains("jne"));
}

#[test]
fn test_instruction_utility_methods() {
    // Test if we can work with instruction variants as needed
    let mov_inst = Instruction::Mov(Operand::reg(Register::RAX), Operand::imm(42));

    // Just verify the instruction can be created and formatted
    assert_eq!(format!("{}", mov_inst), "    mov rax, 42");

    let add_inst = Instruction::Add(Operand::reg(Register::RBX), Operand::reg(Register::RCX));
    assert_eq!(format!("{}", add_inst), "    add rbx, rcx");

    let syscall_inst = Instruction::Syscall;
    assert_eq!(format!("{}", syscall_inst), "    syscall");

    let int_inst = Instruction::Int(0x80);
    assert_eq!(format!("{}", int_inst), "    int 0x80");

    let cmp_inst = Instruction::Cmp(Operand::reg(Register::RAX), Operand::reg(Register::RBX));
    assert_eq!(format!("{}", cmp_inst), "    cmp rax, rbx");
}

#[test]
fn test_imul_instruction_variants() {
    // Test the different Imul instruction variants that correspond to different match arms
    // In Instruction::Imul(dst, src, third) match (src, third):

    // Case 1: (Some(src), Some(third)) => three operand imul
    let imul_three_op =
        Instruction::Imul(Operand::reg(Register::RAX), Some(Operand::reg(Register::RBX)), Some(Operand::imm(42)));
    assert_eq!(format!("{}", imul_three_op), "    imul rax, rbx, 42");

    // Case 2: (Some(src), None) => two operand imul
    let imul_two_op = Instruction::Imul(Operand::reg(Register::RAX), Some(Operand::imm(5)), None);
    assert_eq!(format!("{}", imul_two_op), "    imul rax, 5");

    // Case 3: (None, _) => single operand imul (just destination)
    let imul_one_op = Instruction::Imul(Operand::reg(Register::RAX), None, None);
    assert_eq!(format!("{}", imul_one_op), "    imul rax");
}

#[test]
fn test_control_flow_instructions() {
    // Test Retn instruction with immediate value
    let retn_inst = Instruction::Retn(16);
    assert_eq!(format!("{}", retn_inst), "    ret 16");

    // Test various conditional jumps
    let je_inst = Instruction::Je("label_eq".to_string());
    assert_eq!(format!("{}", je_inst), "    je label_eq");

    let jne_inst = Instruction::Jne("label_ne".to_string());
    assert_eq!(format!("{}", jne_inst), "    jne label_ne");

    let jl_inst = Instruction::Jl("label_less".to_string());
    assert_eq!(format!("{}", jl_inst), "    jl label_less");

    let jle_inst = Instruction::Jle("label_less_equal".to_string());
    assert_eq!(format!("{}", jle_inst), "    jle label_less_equal");

    let jg_inst = Instruction::Jg("label_greater".to_string());
    assert_eq!(format!("{}", jg_inst), "    jg label_greater");

    let jge_inst = Instruction::Jge("label_greater_equal".to_string());
    assert_eq!(format!("{}", jge_inst), "    jge label_greater_equal");

    let jz_inst = Instruction::Jz("label_zero".to_string());
    assert_eq!(format!("{}", jz_inst), "    jz label_zero");

    let jnz_inst = Instruction::Jnz("label_not_zero".to_string());
    assert_eq!(format!("{}", jnz_inst), "    jnz label_not_zero");

    let ja_inst = Instruction::Ja("label_above".to_string());
    assert_eq!(format!("{}", ja_inst), "    ja label_above");

    let jb_inst = Instruction::Jb("label_below".to_string());
    assert_eq!(format!("{}", jb_inst), "    jb label_below");

    let jae_inst = Instruction::Jae("label_above_equal".to_string());
    assert_eq!(format!("{}", jae_inst), "    jae label_above_equal");

    let jbe_inst = Instruction::Jbe("label_below_equal".to_string());
    assert_eq!(format!("{}", jbe_inst), "    jbe label_below_equal");

    let jo_inst = Instruction::Jo("label_overflow".to_string());
    assert_eq!(format!("{}", jo_inst), "    jo label_overflow");

    let jno_inst = Instruction::Jno("label_no_overflow".to_string());
    assert_eq!(format!("{}", jno_inst), "    jno label_no_overflow");

    let js_inst = Instruction::Js("label_sign".to_string());
    assert_eq!(format!("{}", js_inst), "    js label_sign");

    let jns_inst = Instruction::Jns("label_no_sign".to_string());
    assert_eq!(format!("{}", jns_inst), "    jns label_no_sign");

    let loop_inst = Instruction::Loop("loop_label".to_string());
    assert_eq!(format!("{}", loop_inst), "    loop loop_label");
}

#[test]
fn test_system_instructions() {
    // Test system instructions
    let syscall_inst = Instruction::Syscall;
    assert_eq!(format!("{}", syscall_inst), "    syscall");

    let int_80_inst = Instruction::Int(0x80); // Linux system call
    assert_eq!(format!("{}", int_80_inst), "    int 0x80");

    let int_21_inst = Instruction::Int(0x21); // DOS interrupt
    assert_eq!(format!("{}", int_21_inst), "    int 0x21");

    let int_3_inst = Instruction::Int(3); // Breakpoint interrupt
    assert_eq!(format!("{}", int_3_inst), "    int 0x3");

    let hlt_inst = Instruction::Hlt;
    assert_eq!(format!("{}", hlt_inst), "    hlt");
}

#[test]
fn test_control_instructions() {
    // Test control instructions
    let nop_inst = Instruction::Nop;
    assert_eq!(format!("{}", nop_inst), "    nop");

    let cdq_inst = Instruction::Cdq;
    assert_eq!(format!("{}", cdq_inst), "    cdq");

    let cqo_inst = Instruction::Cqo;
    assert_eq!(format!("{}", cqo_inst), "    cqo");
}

#[test]
fn test_conditional_move_instructions() {
    // Test conditional move instructions
    let cmove_inst = Instruction::Cmove(Operand::reg(Register::RAX), Operand::reg(Register::RBX));
    assert_eq!(format!("{}", cmove_inst), "    cmove rax, rbx");

    let cmovne_inst = Instruction::Cmovne(Operand::reg(Register::RCX), Operand::imm(42));
    assert_eq!(format!("{}", cmovne_inst), "    cmovne rcx, 42");

    let cmovl_inst = Instruction::Cmovl(Operand::reg(Register::RDX), Operand::reg(Register::RSI));
    assert_eq!(format!("{}", cmovl_inst), "    cmovl rdx, rsi");

    let cmovle_inst = Instruction::Cmovle(Operand::reg(Register::RDI), Operand::reg(Register::R8));
    assert_eq!(format!("{}", cmovle_inst), "    cmovle rdi, r8");

    let cmovg_inst = Instruction::Cmovg(Operand::reg(Register::R9), Operand::reg(Register::R10));
    assert_eq!(format!("{}", cmovg_inst), "    cmovg r9, r10");

    let cmovge_inst = Instruction::Cmovge(Operand::reg(Register::R11), Operand::reg(Register::R12));
    assert_eq!(format!("{}", cmovge_inst), "    cmovge r11, r12");
}

#[test]
fn test_conditional_set_instructions() {
    // Test conditional set instructions
    let sete_inst = Instruction::Sete(Operand::reg(Register::AL));
    assert_eq!(format!("{}", sete_inst), "    sete al");

    let setne_inst = Instruction::Setne(Operand::reg(Register::BL));
    assert_eq!(format!("{}", setne_inst), "    setne bl");

    let setl_inst = Instruction::Setl(Operand::reg(Register::CL));
    assert_eq!(format!("{}", setl_inst), "    setl cl");

    let setle_inst = Instruction::Setle(Operand::reg(Register::DL));
    assert_eq!(format!("{}", setle_inst), "    setle dl");

    let setg_inst = Instruction::Setg(Operand::reg(Register::SIL));
    assert_eq!(format!("{}", setg_inst), "    setg sil");

    let setge_inst = Instruction::Setge(Operand::reg(Register::DIL));
    assert_eq!(format!("{}", setge_inst), "    setge dil");

    let sets_inst = Instruction::Sets(Operand::reg(Register::R8B));
    assert_eq!(format!("{}", sets_inst), "    sets r8b");

    let setns_inst = Instruction::Setns(Operand::reg(Register::R9B));
    assert_eq!(format!("{}", setns_inst), "    setns r9b");

    let seta_inst = Instruction::Seta(Operand::reg(Register::R10B));
    assert_eq!(format!("{}", seta_inst), "    seta r10b");

    let setb_inst = Instruction::Setb(Operand::reg(Register::R11B));
    assert_eq!(format!("{}", setb_inst), "    setb r11b");

    let setae_inst = Instruction::Setae(Operand::reg(Register::R12B));
    assert_eq!(format!("{}", setae_inst), "    setae r12b");

    let setbe_inst = Instruction::Setbe(Operand::reg(Register::R13B));
    assert_eq!(format!("{}", setbe_inst), "    setbe r13b");

    let seto_inst = Instruction::Seto(Operand::reg(Register::R14B));
    assert_eq!(format!("{}", seto_inst), "    seto r14b");

    let setno_inst = Instruction::Setno(Operand::reg(Register::R15B));
    assert_eq!(format!("{}", setno_inst), "    setno r15b");

    let setz_inst = Instruction::Setz(Operand::reg(Register::AL));
    assert_eq!(format!("{}", setz_inst), "    setz al");

    let setnz_inst = Instruction::Setnz(Operand::reg(Register::BL));
    assert_eq!(format!("{}", setnz_inst), "    setnz bl");
}

#[test]
fn test_additional_instructions() {
    // Test bit operations
    let bt_inst = Instruction::Bt(Operand::reg(Register::RAX), Operand::reg(Register::RBX));
    assert_eq!(format!("{}", bt_inst), "    bt rax, rbx");

    let bts_inst = Instruction::Bts(Operand::reg(Register::RCX), Operand::imm(5));
    assert_eq!(format!("{}", bts_inst), "    bts rcx, 5");

    let btr_inst = Instruction::Btr(Operand::mem("rax+8"), Operand::reg(Register::RDX));
    assert_eq!(format!("{}", btr_inst), "    btr [rax+8], rdx");

    let btc_inst = Instruction::Btc(Operand::reg(Register::RSI), Operand::reg(Register::RDI));
    assert_eq!(format!("{}", btc_inst), "    btc rsi, rdi");

    let bsf_inst = Instruction::Bsf(Operand::reg(Register::R8), Operand::reg(Register::R9));
    assert_eq!(format!("{}", bsf_inst), "    bsf r8, r9");

    let bsr_inst = Instruction::Bsr(Operand::reg(Register::R10), Operand::reg(Register::R11));
    assert_eq!(format!("{}", bsr_inst), "    bsr r10, r11");

    // Test byte swap
    let bswap_inst = Instruction::Bswap(Operand::reg(Register::R12));
    assert_eq!(format!("{}", bswap_inst), "    bswap r12");

    // Test exchange
    let xchg_inst = Instruction::Xchg(Operand::reg(Register::R13), Operand::reg(Register::R14));
    assert_eq!(format!("{}", xchg_inst), "    xchg r13, r14");

    // Test lock prefix
    let lock_inst = Instruction::Lock;
    assert_eq!(format!("{}", lock_inst), "    lock");
}
