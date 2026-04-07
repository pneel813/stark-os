.section .text

.global _start

_start:
    #Enable FPU/Neon Feature
    mov x0, #0x300000
    msr CPACR_EL1, x0
    isb sy
    #load stack top
    ldr x30, =stack_top
    mov sp, x30
    # Mask all interrupts
    msr DAIFSet, #0b1111
    # Load the interrupt vector address into VBAR_EL1
    ldr x0, =evt
    msr VBAR_EL1, x0
    isb sy
    #Enable GICv3
    #Step 1: Enable Distributor
    ldr x0, =0x08000000
    bl init_gic_distributor
    #Step 2: Enable Redistributor
    ldr x0, =0x080A0000
    bl init_gic_redistributor
    #Step 3: Enable CPU interface
    mrs x0, ICC_SRE_EL1
    orr x0, x0, #1
    msr ICC_SRE_EL1, x0
    isb sy
    #Step 4: Set priority of interrupts
    mov x0, #0xff
    bl set_priority_mask
    #Step 5: Enable group 1 interrupts
    bl enable_grp1_ints

    #Test 1 - timer interrupt
    mov w1, #30
    mov w2, #0x80
    ldr x0, =AFFINITY_ENABLED
    ldr x0, [x0]
    cbz x0, not_enabled
    ldr x0, =0x080A0000
    bl set_int_priority
    b next
not_enabled:
    ldr x0, =0x08000000
    bl set_int_priority
next:
    ldr x0, =0x080A0000
    mov w1, #30
    bl set_int_grp
    ldr x0, =0x080A0000
    mov w1, #30
    bl enable_int
    mrs x0, CNTFRQ_EL0
    msr CNTP_TVAL_EL0, x0
    mov x0, #0x1
    msr CNTP_CTL_EL0, x0
    #Unmask IRQ's only
    msr DAIFClr, #0b0010
    # UART config 
    mov x0, #0x09000000
    mov x1, #0x3600
    movk x1,#0x16e,LSL #16
    mov x2, #0xc200
    movk x2, #0x1, LSL #16
    bl init_uart
    bl configure_uart
    svc #0
    bl kmain
    b .
