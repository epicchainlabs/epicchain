; communication.asm

section .data
    message_buffer resb 256 ; Buffer to store messages
    welcome_msg db "Welcome to the communication module!", 0
    prompt_msg db "Enter your message: ", 0
    success_msg db "Message sent successfully!", 0

section .text
    global send_message
    global receive_message

send_message:
    ; Parameters: 
    ;   Input: message_ptr
    ;   Output: eax = 0 (failure), eax = 1 (success)
    ;   Note: Assumes message is a null-terminated string

    ; Get the message pointer from the stack
    mov ebx, [esp + 4] ; message_ptr

    ; Copy the message to the message_buffer
    mov esi, ebx ; Source pointer
    mov edi, message_buffer ; Destination pointer
    mov ecx, 256 ; Message length
    rep movsb ; Copy message

    ; Simulate sending the message (in real code, this would perform actual communication)
    ; For simplicity, we'll just print a success message
    mov eax, 1 ; Success
    ret

receive_message:
    ; Parameters: 
    ;   Output: message_ptr
    ;   Note: Assumes message is a null-terminated string

    ; Simulate receiving a message (in real code, this would read from a communication channel)
    ; For simplicity, we'll just print a prompt and then copy a hardcoded message to the buffer
    mov eax, 4 ; sys_write
    mov ebx, 1 ; stdout
    mov ecx, prompt_msg ; Message prompt
    mov edx, 20 ; Message length
    int 0x80 ; syscall

    ; Copy the hardcoded message to the message buffer
    mov esi, welcome_msg ; Source pointer
    mov edi, message_buffer ; Destination pointer
    mov ecx, 40 ; Message length
    rep movsb ; Copy message

    ; Return the message pointer
    mov eax, message_buffer
    ret
