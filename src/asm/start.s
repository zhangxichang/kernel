.section .text, "ax"
.code32
.global _start32
_start32:
    cli // 屏蔽中断
    // 开启PAE物理地址扩展
    mov eax, cr4
    or eax, 1 << 5
    mov cr4, eax
    // 设置允许进入长模式
    mov ecx, 0xC0000080
    rdmsr
    or eax, 1 << 8
    wrmsr
    // 设置分页地址转换表的顶层表
    lea eax, [pml4]
    mov cr3, eax
    // 开启内存分页
    mov eax, cr0
    or eax, 1 << 31
    mov cr0, eax
    lgdt [gdt64_descriptor] // 设置GDT全局描述符表
    lea esp, [stack_top] //设置32位模式栈区
    // 推送执行64位代码段
    push 0x08
    lea eax, [_start64]
    push eax
    retf
.code64
_start64:
    lea rsp, [rip + stack_top]// 设置64位模式栈区
    call main // 调用内核
// GDT全局描述符表
.section .rodata, "a"
.align 8
gdt64:
    .quad 0x0000000000000000
    .quad 0x00AF9A000000FFFF
gdt64_end:
gdt64_descriptor:
    .word gdt64_end - gdt64 - 1
    .long gdt64
// 分页地址转换表
.section .data, "aw"
.align 4096
pml4:
    .quad pdpt + 0x003
    .fill 511, 8, 0
.align 4096
pdpt:
    .quad pd + 0x003
    .fill 511, 8, 0
.align 4096
pd:
    .quad 0x0000000000000083
    .fill 511, 8, 0
// 栈区
.section .bss, "aw", @nobits
.align 16
    .skip 16384
stack_top:
