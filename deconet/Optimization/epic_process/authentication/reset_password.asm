; reset_password.asm

section .data
    username db "admin", 0 ; Predefined username
    password db "password", 0 ; Predefined password

section .text
    global reset_password

reset_password:
    ; Parameters: 
    ;   Input: username_ptr, new_password_ptr
    ;   Output: eax = 0 (failure), eax = 1 (success)

    ; Check if the entered username matches the predefined username
    mov esi, username ; Pointer to the predefined username
    mov edi, [esp + 4] ; Pointer to the entered username
    xor ecx, ecx ; Clear ECX register
    check_username_loop:
    mov al, [esi + ecx] ; Get a character from the predefined username
    cmp al, [edi + ecx] ; Compare it with the entered username character
    jne .username_not_found ; If they don't match, jump to username_not_found
    inc ecx ; Move to the next character
    cmp al, 0 ; Check if we've reached the end of the username
    jnz check_username_loop ; If not, continue checking

    ; If we've reached this point, the username was found
    ; Now, update the password
    mov esi, password ; Pointer to the predefined password
    mov edi, [esp + 8] ; Pointer to the new password
    xor ecx, ecx ; Clear ECX register
    update_password_loop:
    mov al, [edi + ecx] ; Get a character from the new password
    mov [esi + ecx], al ; Update the predefined password with the new character
    inc ecx ; Move to the next character
    cmp al, 0 ; Check if we've reached the end of the new password
    jnz update_password_loop ; If not, continue updating

    ; Password reset successful
    mov eax, 1 ; Set return value to 1 (success)
    ret

    .username_not_found:
    ; Username not found
    mov eax, 0 ; Set return value to 0 (failure)
    ret
