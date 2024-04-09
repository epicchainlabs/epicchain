; register.asm

section .data
    username db 20 ; Buffer to store username (maximum 20 characters)
    password db 20 ; Buffer to store password (maximum 20 characters)
    registered db 0 ; Flag to indicate if user is registered (0 = not registered, 1 = registered)

section .text
    global register_user

register_user:
    ; Parameters: 
    ;   Input: username_ptr, password_ptr
    ;   Output: eax = 0 (failure), eax = 1 (success)

    ; Check if user is already registered
    cmp byte [registered], 1 ; Check if user is already registered
    je .already_registered ; If user is registered, jump to already_registered

    ; Register the user
    mov edi, username ; Pointer to username buffer
    mov esi, [esp + 4] ; Pointer to the entered username
    mov ecx, 20 ; Maximum length of username
    rep movsb ; Copy username to username buffer

    mov edi, password ; Pointer to password buffer
    mov esi, [esp + 8] ; Pointer to the entered password
    mov ecx, 20 ; Maximum length of password
    rep movsb ; Copy password to password buffer

    mov byte [registered], 1 ; Set registered flag to 1 (user is registered)

    ; Registration successful
    mov eax, 1 ; Set return value to 1 (success)
    ret

.already_registered:
    ; User is already registered
    mov eax, 0 ; Set return value to 0 (failure)
    ret
