; logging.asm

section .data
    log_message db 256 dup(0) ; Buffer to store log messages

section .text
    global log_message

; Function: log_message
; Description: Logs a message to the console.
; Parameters: message_ptr - Pointer to the message to log
; Returns: None
log_message:
    mov edx, log_message ; Load the address of log_message
    call copy_string ; Call the copy_string function to copy the message
    mov edx, log_message ; Load the address of log_message
    call print_string ; Call the print_string function to print the message
    ret

; Function: copy_string
; Description: Copies a null-terminated string from source to destination.
; Parameters: None
; Returns: None
copy_string:
    xor ecx, ecx ; Clear ECX register for loop counter
copy_string_loop:
    mov al, [edx + ecx] ; Load byte from source
    cmp al, 0 ; Check for null terminator
    je copy_string_done ; Exit loop if null terminator is found
    mov [log_message + ecx], al ; Store byte in destination
    inc ecx ; Move to next byte
    jmp copy_string_loop ; Continue loop
copy_string_done:
    ret

; Function: print_string
; Description: Prints a null-terminated string to the console.
; Parameters: None
; Returns: None
print_string:
    ; syscall for sys_write
    mov eax, 4
    ; file descriptor for stdout
    mov ebx, 1
    ; pointer to the message to print
    mov ecx, edx
    ; length of the message
    xor edx, edx ; Clear EDX register
    ; loop through the string until null terminator
print_string_loop:
    cmp byte [ecx + edx], 0 ; Check for null terminator
    je print_string_done ; Exit loop if null terminator is found
    inc edx ; Move to next byte
    jmp print_string_loop ; Continue loop
print_string_done:
    ; syscall for sys_write
    int 0x80
    ret
