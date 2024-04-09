; error_handling.asm

section .text
    global handle_error

handle_error:
    ; Parameters: 
    ;   Input: error_code
    ;   Output: None

    ; Print an error message based on the error code
    ; For simplicity, we'll just print a generic error message

    cmp dword [esp + 4], 1 ; Compare error_code with 1
    je .error1 ; If error_code is 1, jump to error1
    cmp dword [esp + 4], 2 ; Compare error_code with 2
    je .error2 ; If error_code is 2, jump to error2
    cmp dword [esp + 4], 3 ; Compare error_code with 3
    je .error3 ; If error_code is 3, jump to error3
    cmp dword [esp + 4], 4 ; Compare error_code with 4
    je .error4 ; If error_code is 4, jump to error4
    jmp .default_error ; If error_code doesn't match any case, jump to default_error

.error1:
    mov eax, 4 ; sys_write
    mov ebx, 1 ; stdout
    mov ecx, error_message1 ; Message
    mov edx, error_message1_length ; Message length
    int 0x80 ; syscall
    jmp .end

.error2:
    mov eax, 4 ; sys_write
    mov ebx, 1 ; stdout
    mov ecx, error_message2 ; Message
    mov edx, error_message2_length ; Message length
    int 0x80 ; syscall
    jmp .end

.error3:
    mov eax, 4 ; sys_write
    mov ebx, 1 ; stdout
    mov ecx, error_message3 ; Message
    mov edx, error_message3_length ; Message length
    int 0x80 ; syscall
    jmp .end

.error4:
    mov eax, 4 ; sys_write
    mov ebx, 1 ; stdout
    mov ecx, error_message4 ; Message
    mov edx, error_message4_length ; Message length
    int 0x80 ; syscall
    jmp .end

.default_error:
    mov eax, 4 ; sys_write
    mov ebx, 1 ; stdout
    mov ecx, default_error_message ; Message
    mov edx, default_error_message_length ; Message length
    int 0x80 ; syscall

.end:
    ret

section .data
    error_message1 db "Error 1: Something went wrong!", 0
    error_message1_length equ $ - error_message1

    error_message2 db "Error 2: Another error occurred!", 0
    error_message2_length equ $ - error_message2

    error_message3 db "Error 3: Yet another error!", 0
    error_message3_length equ $ - error_message3

    error_message4 db "Error 4: Oh no, more errors!", 0
    error_message4_length equ $ - error_message4

    default_error_message db "An unknown error occurred!", 0
    default_error_message_length equ $ - default_error_message
