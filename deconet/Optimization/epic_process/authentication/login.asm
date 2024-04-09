; login.asm

section .data
    username db "admin", 0 ; Predefined username
    password db "password", 0 ; Predefined password

section .text
    global login

login:
    ; Parameters: 
    ;   Input: username_ptr, password_ptr
    ;   Output: eax = 0 (failure), eax = 1 (success)

    ; Check if the entered username matches the predefined username
    mov esi, username ; Pointer to the predefined username
    mov edi, [esp + 4] ; Pointer to the entered username
    xor ecx, ecx ; Clear ECX register
check_username_loop:
    mov al, [esi + ecx] ; Get a character from the predefined username
    cmp al, [edi + ecx] ; Compare it with the entered username character
    jne .login_failed ; If they don't match, jump to login_failed
    inc ecx ; Move to the next character
    cmp al, 0 ; Check if we've reached the end of the username
    jnz check_username_loop ; If not, continue checking

    ; Check if the entered password matches the predefined password
    mov esi, password ; Pointer to the predefined password
    mov edi, [esp + 8] ; Pointer to the entered password
    xor ecx, ecx ; Clear ECX register
check_password_loop:
    mov al, [esi + ecx] ; Get a character from the predefined password
    cmp al, [edi + ecx] ; Compare it with the entered password character
    jne .login_failed ; If they don't match, jump to login_failed
    inc ecx ; Move to the next character
    cmp al, 0 ; Check if we've reached the end of the password
    jnz check_password_loop ; If not, continue checking

    ; If we've reached this point, the login was successful
    mov eax, 1 ; Set return value to 1 (success)
    ret

.login_failed:
    ; If we've reached this point, the login failed
    mov eax, 0 ; Set return value to 0 (failure)
    ret
