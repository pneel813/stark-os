.section .text

.global _start

_start:
    #Mask all interrupts
    msr DAIFSet, #0b1111
    # Load the EVT table into VBAR_EL1
    ldr x0,=evt
    msr VBAR_EL1, x0
    isb
    ldr x30,=stack_top
    mov sp, x30
    mov x0, #0x09000000 
    mov x1, #0x3600
    movk x1, #16e, LSL #16
    mov x2, #0xC200
    movk x2, #0x1, LSL #16
    bl uart_init
    bl configure_uart
    svc #0
    bl kmain
    b .

