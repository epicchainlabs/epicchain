; logging.asm

section .data
    log_file db "log.txt", 0
    log_buffer resb 256

section .text
    global log_message

log_message:
    ; Parameters: 
    ;   Input: message_ptr
    ;   Output: None
    ;   Note: Assumes message_ptr is a null-terminated string

    ; Open the log file
    mov eax, 5 ; sys_open
    mov ebx, log_file ; Filename
    mov ecx, 65 ; Flags: O_CREAT | O_WRONLY | O_APPEND
    mov edx, 0644 ; Mode: Read/Write
    int 0x80 ; syscall

    ; Check for successful file open
    cmp eax, -1
    je .error ; If file open failed, jump to error

    ; Write the message to the log file
    mov ebx, eax ; File descriptor
    mov eax, 4 ; sys_write
    mov ecx, [esp + 4] ; message_ptr
    mov edx, 256 ; Message length
    int 0x80 ; syscall

    ; Close the log file
    mov eax, 6 ; sys_close
    int 0x80 ; syscall

    ret

.error:
    ; Print an error message if file open failed
    mov eax, 4 ; sys_write
    mov ebx, 1 ; stdout
    mov ecx, error_message ; Error message
    mov edx, error_message_length ; Message length
    int 0x80 ; syscall

    ret

section .data
    error_message db "Error opening log file!", 0
    error_message_length equ $ - error_message
