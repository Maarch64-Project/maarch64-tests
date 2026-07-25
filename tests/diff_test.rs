use maarch64_core::{cpu::CpuContext, interp::Interpreter, memory::MemoryManager};
use unicorn_engine::{Arch, Mode, Prot, RegisterARM64, Unicorn};

#[test]
fn test_unicorn_differential_execution() -> Result<(), Box<dyn std::error::Error>> {
    let code_addr: u64 = 0x400000;
    let stack_top: u64 = 0x800000;
    let mem_size: usize = 0x800000;

    let mut uc = Unicorn::new(Arch::ARM64, Mode::ARM)?;
    uc.mem_map(code_addr, mem_size as u64, Prot::ALL)?;
    uc.reg_write(RegisterARM64::SP, stack_top)?;

    let mut mem = MemoryManager::new();
    mem.map_anonymous(code_addr, mem_size)?;
    let mut ctx = CpuContext::new();
    ctx.pc = code_addr;
    ctx.sp = stack_top;
    ctx.pstate = uc.reg_read(RegisterARM64::NZCV)? as u32;

    let code: &[u8] = &[
        0x40, 0x05, 0x80, 0xd2, // MOV X0, #42
        0x81, 0x0c, 0x80, 0xd2, // MOV X1, #100
        0x02, 0x00, 0x01, 0x8b, // ADD X2, X0, X1
        0x23, 0x00, 0x00, 0xeb, // SUBS X3, X1, X0
        0x80, 0x46, 0xa2, 0xf2, // MOVK X0, #0x1234, lsl #16
        0xe0, 0x07, 0xbf, 0xa9, // STP X0, X1, [sp, #-16]!
        0xe4, 0x17, 0xc1, 0xa8, // LDP X4, X5, [sp], #16
        0x1f, 0x00, 0x05, 0xeb, // CMP X4, X5
        0x86, 0x00, 0x85, 0x1a, // CSEL X6, X4, X5, EQ
    ];

    let num_instructions = code.len() / 4;

    uc.mem_write(code_addr, code)?;
    mem.write(code_addr, code)?;

    let registers: &[(usize, RegisterARM64, &str)] = &[
        (0, RegisterARM64::X0, "X0"),
        (1, RegisterARM64::X1, "X1"),
        (2, RegisterARM64::X2, "X2"),
        (3, RegisterARM64::X3, "X3"),
        (4, RegisterARM64::X4, "X4"),
        (5, RegisterARM64::X5, "X5"),
        (6, RegisterARM64::X6, "X6"),
    ];

    for _step in 0..num_instructions {
        let pc_before = ctx.pc;
        uc.emu_start(pc_before, pc_before + 4, 0, 1)?;
        Interpreter::step(&mut ctx, &mut mem)?;

        assert_eq!(ctx.sp, uc.reg_read(RegisterARM64::SP)?);

        for &(reg_idx, uc_reg, _name) in registers {
            assert_eq!(ctx.get_x(reg_idx), uc.reg_read(uc_reg)?);
        }

        let uc_nzcv = (uc.reg_read(RegisterARM64::NZCV)? as u32) & 0xf0000000;
        let maarch_nzcv = ctx.pstate & 0xf0000000;
        assert_eq!(maarch_nzcv, uc_nzcv);
    }

    Ok(())
}
