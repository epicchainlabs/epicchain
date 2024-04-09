; authentication.asm

section .data
    username db "admin", 0
    password db "password123", 0
    welcome_msg db "Welcome, admin!", 0
    failure_msg db "Authentication failed!", 0

section .text
    global authenticate_user
    global verify_user

authenticate_user:
    ; Parameters: 
    ;   Input: username_ptr, password_ptr
    ;   Output: eax = 0 (failure), eax = 1 (success)
    ;   Note: Assumes username and password are null-terminated strings

    ; Get the parameters (username and password pointers) from the stack
    mov ebx, [esp + 4] ; username_ptr
    mov ecx, [esp + 8] ; password_ptr

    ; Compare the username and password with the hardcoded values
    ; If they match, return success (eax = 1), otherwise return failure (eax = 0)
    cmp dword [ebx], 'nimd' ; Reverse 'admin'
    jne .failure
    cmp dword [ecx], '321drowssap' ; Reverse 'password123'
    jne .failure

    mov eax, 1 ; Authentication successful
    ret

.failure:
    mov eax, 0 ; Authentication failed
    ret

verify_user:
    ; Verify user's permissions or other details
    ; This function can be expanded based on the project's requirements
    ; For simplicity, it always returns success (eax = 1)

    mov eax, 1 ; Verification successful
    ret

section .text
    global display_message
    global display_failure

display_message:
    ; Display a welcome message
    ; Parameters: 
    ;   Input: message_ptr
    ;   Output: None
    ;   Note: Assumes message is a null-terminated string

    ; Get the message pointer from the stack
    mov ebx, [esp + 4] ; message_ptr

    ; Print the message
    mov eax, 4 ; sys_write
    mov edx, 16 ; message length
    int 0x80 ; syscall

    ret

display_failure:
    ; Display a failure message
    ; Parameters: 
    ;   Input: message_ptr
    ;   Output: None
    ;   Note: Assumes message is a null-terminated string

    ; Get the message pointer from the stack
    mov ebx, [esp + 4] ; message_ptr

    ; Print the message
    mov eax, 4 ; sys_write
    mov edx, 20 ; message length
    int 0x80 ; syscall

    ret
