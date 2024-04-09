; encryption.asm

section .data
    message_buffer resb 256 ; Buffer to store messages
    key db "secretkey", 0

section .text
    global encrypt_data
    global decrypt_data

encrypt_data:
    ; Parameters: 
    ;   Input: message_ptr
    ;   Output: eax = 0 (failure), eax = 1 (success)
    ;   Note: Assumes message is a null-terminated string

    ; Get the message pointer from the stack
    mov ebx, [esp + 4] ; message_ptr

    ; Get the length of the message
    mov ecx, ebx ; Pointer to message
    xor edx, edx ; Clear edx
    @@length_loop:
        cmp byte [ecx + edx], 0 ; Check for null terminator
        je @@length_done ; If null terminator, end loop
        inc edx ; Increment length counter
        jmp @@length_loop ; Continue loop
    @@length_done:

    ; Encrypt the message using a simple XOR encryption with the key
    xor esi, esi ; Clear esi for loop counter
    xor edi, edi ; Clear edi for destination pointer
    mov esi, ebx ; Set source pointer to message
    lea edi, [message_buffer] ; Set destination pointer to message_buffer
    mov eax, 0 ; Clear eax for encryption
    xor ecx, ecx ; Clear ecx for loop counter

    @@encrypt_loop:
        mov al, [esi + ecx] ; Get byte from message
        mov dl, [key + ecx % 8] ; Get byte from key (wrap around)
        xor al, dl ; XOR byte from message with byte from key
        mov [edi + ecx], al ; Store encrypted byte
        inc ecx ; Increment loop counter
        cmp ecx, edx ; Compare with message length
        jl @@encrypt_loop ; Continue loop if not at end

    mov eax, 1 ; Encryption successful
    ret

decrypt_data:
    ; Parameters: 
    ;   Input: message_ptr
    ;   Output: eax = 0 (failure), eax = 1 (success)
    ;   Note: Assumes message is a null-terminated string

    ; Decryption is essentially the same as encryption with XOR
    ; Decrypting an already encrypted message will return the original message

    mov ebx, [esp + 4] ; message_ptr
    xor esi, esi
    xor edi, edi
    mov esi, ebx
    lea edi, [message_buffer]
    mov eax, 0
    xor ecx, ecx

    @@decrypt_loop:
        mov al, [esi + ecx]
        mov dl, [key + ecx % 8]
        xor al, dl
        mov [edi + ecx], al
        inc ecx
        cmp ecx, edx
        jl @@decrypt_loop

    mov eax, 1 ; Decryption successful
    ret
