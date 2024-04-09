; secure_communication.asm

section .data
    key db "secretkey", 0 ; Predefined encryption key
    message db "hello", 0 ; Message to be encrypted and decrypted
    encrypted_message db 256 dup(0) ; Buffer to store encrypted message
    decrypted_message db 256 dup(0) ; Buffer to store decrypted message

section .text
    global secure_communication

secure_communication:
    ; Parameters: 
    ;   Input: message_ptr
    ;   Output: encrypted_message_ptr, decrypted_message_ptr

    ; Encrypt the message using the predefined key
    mov esi, message ; Pointer to the message
    mov edi, encrypted_message ; Pointer to the buffer for encrypted message
    mov ebx, key ; Pointer to the encryption key
    xor ecx, ecx ; Clear ECX register
encrypt_loop:
    mov al, [esi + ecx] ; Get a character from the message
    xor al, [ebx + ecx] ; XOR it with the corresponding character from the key
    mov [edi + ecx], al ; Store the encrypted character in the buffer
    inc ecx ; Move to the next character
    cmp al, 0 ; Check if we've reached the end of the message
    jnz encrypt_loop ; If not, continue encrypting

    ; Decrypt the encrypted message using the same key
    mov esi, encrypted_message ; Pointer to the encrypted message
    mov edi, decrypted_message ; Pointer to the buffer for decrypted message
    mov ebx, key ; Pointer to the decryption key
    xor ecx, ecx ; Clear ECX register
decrypt_loop:
    mov al, [esi + ecx] ; Get a character from the encrypted message
    xor al, [ebx + ecx] ; XOR it with the corresponding character from the key
    mov [edi + ecx], al ; Store the decrypted character in the buffer
    inc ecx ; Move to the next character
    cmp al, 0 ; Check if we've reached the end of the encrypted message
    jnz decrypt_loop ; If not, continue decrypting

    ; Return the pointers to the encrypted and decrypted messages
    mov eax, encrypted_message ; Set return value to the pointer to the encrypted message
    mov edx, decrypted_message ; Set edx register to the pointer to the decrypted message
    ret
