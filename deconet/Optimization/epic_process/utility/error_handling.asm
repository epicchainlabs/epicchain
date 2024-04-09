; error_handling.asm

section .data
    error_log_file db "error.log", 0 ; File to log errors
    error_message db 256 dup(0) ; Buffer to store error messages

section .text
    global log_error, handle_exception

; Function: log_error
; Description: Logs an error message to a file.
; Parameters: error_message_ptr - Pointer to the error message
; Returns: None
log_error:
    mov edx, error_log_file ; File to log errors
    mov eax, 4 ; syscall for sys_write
    mov ebx, 1 ; file descriptor for stdout
    int 0x80 ; call kernel

    ret

; Function: handle_exception
; Description: Handles an exception by logging the error message and terminating the program.
; Parameters: error_message_ptr - Pointer to the error message
; Returns: None
handle_exception:
    ; Log the error message
    mov edx, error_message ; Load the address of error_message
    call log_error ; Call the log_error function

    ; Terminate the program
    mov eax, 1 ; syscall for sys_exit
    xor ebx, ebx ; exit code 0
    int 0x80 ; call kernel

    ret
